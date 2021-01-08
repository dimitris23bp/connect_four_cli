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

pub fn menu() -> (Player, Player, Settings) {

    let (player1, player2) = player_menu();
    let settings = game_menu();


    (player1, player2, settings)
}


fn player_menu() -> (Player, Player) {

    print!("Name of player 1:");
    Write::flush(&mut io::stdout()).expect("flush failed!");

    let mut name1: String = String::new();
    stdin().read_line(&mut name1).unwrap();
    let name1 = String::from(name1.trim());

    print!("Name of player 2:");
    Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut name2: String = String::new();
    stdin().read_line(&mut name2).unwrap();
    let name2 = String::from(name2.trim());

    (Player {
        name: name1,
        chip: 'X',
        points: 0,

    },
    Player {
        name: name2,
        chip: 'O',
        points: 0,

    })

}

// Choose points to win
// Choose the size of the board
fn game_menu() -> Settings {

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
    Settings {
        rows,
        cols,
        points
    }
}
