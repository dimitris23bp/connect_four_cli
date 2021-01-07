use std::io::{self, stdin, Write};

use crate::{Player, Settings};

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
pub fn menu() -> (Player, Player, Settings) {

    print!("Points to win: ");
    Write::flush(&mut io::stdout()).expect("flush failed!");
    let points = read_int();

    print!("Rows of the board: ");
    Write::flush(&mut io::stdout()).expect("flush failed!");
    let rows = read_int();

    print!("Columns of the board: ");
    Write::flush(&mut io::stdout()).expect("flush failed!");
    let cols = read_int();

    // Return the players and the settings of the game all at once
    (Player {
        name: String::from("player1"),
        chip: 'X',
        points: 0,

    },
    Player {
        name: String::from("player2"),
        chip: 'O',
        points: 0,

    },
    Settings {
        rows,
        cols,
        points
    })
}
