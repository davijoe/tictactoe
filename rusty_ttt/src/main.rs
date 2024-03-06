mod board;
mod player;

use board::Board;
use player::{Player, GamePiece};

fn main() {
    let mut board = Board::new(3);
    let player1 = Player::new("Player 1".to_string(), GamePiece::X);
    let player2 = Player::new("Player 2".to_string(), GamePiece::O);
    let mut current_player = &player1;

    loop {
        println!("{}'s turn", current_player.name);
        board.display();

        // Example move input and placement
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let index: usize = input.trim().parse().expect("Please type a number!");

        if !board.place_marker(index, current_player.game_piece) {
            println!("Invalid move! Try again.");
            continue;
        }

        if let Some(winner) = board.check_winner() {
            println!("{} wins!", current_player.name);
            board.display();
            break;
        } else if board.is_full() {
            println!("It's a draw!");
            board.display();
            break;
        }

        current_player = if current_player == &player1 { &player2 } else { &player1 };
    }
}

