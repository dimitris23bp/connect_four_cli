use std::io;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use crate::display::show_game;

struct Settings {
    rows: u8,
    cols: u8,
    points: u8,
}

fn read_int() -> u8{
    loop {
        let mut input_text = String::new();
        io::stdin()
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

    pub fn show_game(rows: &u8, cols: &u8, data: &Vec<char>, position: &u8) {
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
    pub fn board(rows: &u8, cols: &u8, data: &Vec<char>) {

        for row in 0..*rows {

            for col in 0..*cols {
                print!("|");
                print!("{}", data.get((row * cols + col) as usize).unwrap());
            }
            print!("|\n\r");
        }
    }


}

fn main() {
    println!("Welcome!");

    // Get the basic settings
    let settings = menu();

    let mut position: u8 = 0;

    // Initialization of data vector
    let mut data= vec![' '; (settings.rows * settings.cols) as usize];

    // display::show_game(&settings.rows, &settings.cols, &data, &2);

    let stdin = stdin();

    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();

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

        //i reckon this speaks for itself
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
            // Key::Down => turn(),
            Key::Ctrl('c') => {
                print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::All);
                break;
            },
            _ => (),
        }

    }

}
