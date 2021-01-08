use std::io::{stdin, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod menu;

use menu::menu;

pub struct Settings {
    pub rows: u8,
    pub cols: u8,
    pub points: u8,
}

pub struct Player {
    pub name: String,
    pub chip: char,
    pub points: u8,
}

// Every function about display is placed into this module
mod display {
    use crate::{Settings, Player};

    // Display the whole game
    pub fn show_game(settings: &Settings, player1: &Player, player2: &Player, data: &Vec<Vec<char>>, position: &u8) {
        print!(
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        );
    
        score(&settings.points, &player1, &player2);
        cursor_as_line(&settings.cols, position);
        board(&settings.rows, &settings.cols, data);
    }

    // Displays the Scoreboard line
    pub fn score(points: &u8, player1: &Player, player2: &Player){

        print!("{}: {} - {}: {} - {}/{}", 
        player1.name, player1.points, 
        player2.name, player2.points, 
        player1.points + player2.points, points);

        print!("\n\r");
    }

    // Displays the line of the player movement
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

// Check if a player won the round
fn is_winner(chips: &Vec<Vec<char>>) -> bool {

    for i in 0..chips.len(){
        let row = chips.get(i).unwrap();
        for j in 0..row.len() {
            let chip = row.get(j).unwrap();

            // If the chip is not empty
            if !chip.eq(&' ') {
                
                // Empty values to catch out of index errors
                let empty_chip: char = ' ';
                let empty_vec: Vec<char> = Vec::new();

                // 'If' statements are seperated for reading purposes

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

// Load the required chip to the Vector
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
    let (mut player1, mut player2, settings) = menu();

    // Position of the cursor
    let mut position: u8 = 0;

    let mut chip: char = 'O';

    // Initialization of data vector
    let mut data = vec![vec![' '; settings.cols.into()]; settings.rows.into()];

    //setting up stdout and going into raw mode
    let _stdout = stdout().into_raw_mode().unwrap();

    display::show_game(&settings, &player1, &player2,  &data, &position);

    print!("{}", termion::cursor::Hide);

    while settings.points > player1.points + player2.points {
        let stdin = stdin();
        
        //detecting keydown events
        for c in stdin.keys() {

            match c.unwrap() {
                Key::Right => {
                    if position < (settings.cols - 1) {
                        position = position + 1;
                    }
                    print!("{}{}", termion::cursor::Goto(1, 2), termion::clear::CurrentLine);
                    display::cursor_as_line(&settings.cols, &position);
                },
                Key::Left => {
                    if position > 0 {
                        position = position - 1;
                    }
                    print!("{}{}", termion::cursor::Goto(1, 2), termion::clear::CurrentLine);
                    display::cursor_as_line(&settings.cols, &position);
                },
                Key::Down => {

                    // Delete board
                    print!("{}{}", termion::cursor::Goto(1, 3), termion::clear::AfterCursor);

                    // Load chip to the data structure
                    let valid_turn: bool = load_chip(&position, &settings.rows, &chip, &mut data);
                    
                    // Display board once again
                    display::board(&settings.rows, &settings.cols, &data);
                    
                    // Check if the current player won
                    let won_game: bool = is_winner(&data);
                    
                    if won_game {

                        // Clear data
                        data = vec![vec![' '; settings.cols.into()]; settings.rows.into()];

                        // Give point to the winner
                        if player1.chip == chip {
                            player1.points += 1;
                        } else {
                            player2.points += 1;
                        }

                        display::show_game(&settings, &player1, &player2, &data, &position);

                        // Check if the game is over
                        if settings.points == player1.points + player2.points {
                            print!("{}{}{}", termion::cursor::Goto(1, 1), termion::clear::All, termion::cursor::Show);
                            break;
                        }
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
}
