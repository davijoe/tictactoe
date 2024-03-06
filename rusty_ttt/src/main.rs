use std::io::stdin;

mod board;

mod player;
use player::Player;



fn main() {
    let name = what_is_your_name();
    let game_piece = Player::choose_game_piece();
    let player = Player::new(name, game_piece);

    println!("Welcome, {}! You will be playing as {:?}", player.name, player.game_piece); 

    // Display the board
    let board = vec![vec![' '; 3]; 3];
    display_board(&board);
}
