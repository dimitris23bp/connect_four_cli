use std::io;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct Settings {
    rows: u8,
    cols: u8,
    points: u8,
}

fn read_int() -> u8{
    loop {
        let mut input_text = String::new();
        stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<u8>() {
            Ok(i) => return i,
            Err(..) => println!("Please enter an integer"),
        };
    }
}

// Choose points to win
// Choose the size of the board
fn menu() -> Settings {

    print!("Points to win: ");
    Write::flush(&mut io::stdout()).expect("flush failed!");
    let points = read_int();

    print!("Rows of the board: ");
    Write::flush(&mut io::stdout()).expect("flush failed!");
    let rows = read_int();

    print!("Columns of the board: ");
    Write::flush(&mut io::stdout()).expect("flush failed!");
    let cols = read_int();

    Settings {
        rows,
        cols,
        points
    }
}

mod display {

    pub fn show_game(rows: &u8, cols: &u8, data: &Vec<Vec<char>>, position: &u8) {
        cursor_as_line(cols, position);
        board(rows, cols, data);
    }

    pub fn cursor_as_line(cols: &u8, position: &u8) {

        for col in 0..*cols {
            print!(" ");

            if *position == col {
                print!("v");
            } else {
                print!(" ");
            }
        }
        print!("\n\r");
    }

    // The board of the game
    pub fn board(rows: &u8, cols: &u8, data: &Vec<Vec<char>>) {

        for row in 0..*rows {

            for col in 0..*cols {
                print!("|");
               // print!("{}", data.get((row * cols + col) as usize).unwrap());
                print!("{}", data[row as usize][col as usize]);
            
            }
            print!("|\n\r");
        }
    }

}

fn is_winner(chips: &Vec<Vec<char>>) -> bool {

    for i in 0..chips.len(){
        let row = chips.get(i).unwrap();
        for j in 0..row.len() {
            let chip = row.get(j).unwrap();

            if !chip.eq(&' ') {
                
                // Empty values to catch out of index errors
                let empty_chip: char = ' ';
                let empty_vec: Vec<char> = Vec::new();

                // If statements are seperated for reading purposes

                // Check vertical
                if chip.eq(&chips.get(i + 1).unwrap_or(&empty_vec).get(j).unwrap_or(&empty_chip)) 
                && chip.eq(&chips.get(i + 2).unwrap_or(&empty_vec).get(j).unwrap_or(&empty_chip))
                && chip.eq(&chips.get(i + 3).unwrap_or(&empty_vec).get(j).unwrap_or(&empty_chip)) {
                    return true
                }    
    
                // Check horizontal
                if chip.eq(&chips.get(i).unwrap_or(&empty_vec).get(j + 1).unwrap_or(&empty_chip)) 
                && chip.eq(&chips.get(i).unwrap_or(&empty_vec).get(j + 2).unwrap_or(&empty_chip))
                && chip.eq(&chips.get(i).unwrap_or(&empty_vec).get(j + 3).unwrap_or(&empty_chip)) {
                    return true
                }    
    
                //Check diagonal
                if chip.eq(&chips.get(i + 1).unwrap_or(&empty_vec).get(j + 1).unwrap_or(&empty_chip)) 
                && chip.eq(&chips.get(i + 2).unwrap_or(&empty_vec).get(j + 2).unwrap_or(&empty_chip))
                && chip.eq(&chips.get(i + 3).unwrap_or(&empty_vec).get(j + 3).unwrap_or(&empty_chip)) {
                    return true
                }  

                // Check reversed diagonal
                if chip.eq(&chips.get(i - 1).unwrap_or(&empty_vec).get(j + 1).unwrap_or(&empty_chip)) 
                && chip.eq(&chips.get(i - 2).unwrap_or(&empty_vec).get(j + 2).unwrap_or(&empty_chip))
                && chip.eq(&chips.get(i - 3).unwrap_or(&empty_vec).get(j + 3).unwrap_or(&empty_chip)) {
                    return true
                }    

            }

    

        }

        
    }

    false

}

fn load_chip(position: &u8, rows: &u8, chip: &char, data: &mut Vec<Vec<char>>) -> bool{

    let col: usize = (*position).into();

    for i in (0..*rows as usize).rev(){
        if data[i][col] == ' ' {
            data[i][col] = *chip;
            return true
        }
    }

    false
}

fn main() {
    println!("Welcome!");

    // Get the basic settings
    let settings = menu();

    // Position of the cursor
    let mut position: u8 = 0;

    let mut chip: char = 'O';

    // Initialization of data vector
    let mut data = vec![vec![' '; settings.cols.into()]; settings.rows.into()];

    let stdin = stdin();

    //setting up stdout and going into raw mode
    let _stdout = stdout().into_raw_mode().unwrap();

    print!(
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All
    );

    display::show_game(&settings.rows, &settings.cols, &data, &position);

    print!("{}", termion::cursor::Hide);

    //detecting keydown events
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        // print!(
        //     "{}{}",
        //     termion::cursor::Goto(1, 1),
        //     termion::clear::All
        // );

        match c.unwrap() {
            Key::Right => {
                if position < (settings.cols - 1) {
                    position = position + 1;
                }
                print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine);
                display::cursor_as_line(&settings.cols, &position);
            },
            Key::Left => {
                if position > 0 {
                    position = position - 1;
                }
                print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine);
                display::cursor_as_line(&settings.cols, &position);
            },
            Key::Down => {

                // Delete board
                print!("{}{}", termion::cursor::Goto(1, 2), termion::clear::AfterCursor);

                // Load chip to the data structure
                let valid_turn: bool = load_chip(&position, &settings.rows, &chip, &mut data);
                
                // Display board once again
                display::board(&settings.rows, &settings.cols, &data);
                
                // Check if the current player won
                let won_game: bool = is_winner(&data);
                
                if won_game {
                    print!("{}{}{}", termion::cursor::Goto(1, 1), termion::clear::All, termion::cursor::Show);
                    break;
                }
                // Change chip, so the other player can play
                if valid_turn {
                    if chip == 'O' {
                        chip = 'X';
                    } else {
                        chip = 'O';
                    }
                }
            }
            Key::Ctrl('c') => {
                print!("{}{}{}", termion::cursor::Goto(1, 1), termion::clear::All, termion::cursor::Show);
                break;
            },
            _ => (),
        }

    }

}
