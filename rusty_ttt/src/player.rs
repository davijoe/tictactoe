use std::io::stdin;


#[derive(Debug, Clone, Copy)]
pub enum GamePiece {
    X,
    O,
}

pub struct Player {
    pub name: String,
    pub game_piece: GamePiece
}

fn what_is_your_name() -> String {
    println!("Hello! Enter your name:");
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        // "Unwraps" a result object and terminates programe
        .expect("Failed to read your name");
    your_name   
}

impl Player {
    pub fn new(name: String, game_piece: GamePiece) -> Self {
        Player { name, game_piece }
    }

    pub fn choose_game_piece() -> GamePiece {
        loop {
            println!("Choose whether you want to b \"X\" or \"O\"");
            let mut choice = String::new();
            stdin()
                .read_line(&mut choice)
                .expect("Failed to read your choice");

            match choice.trim().to_uppercase().as_str() {
                "X" => return GamePiece::X,
                "O" => return GamePiece::O,
                _ => println!("Invalid choide. Please enter only X or O"),
            }
        }
    }
}

