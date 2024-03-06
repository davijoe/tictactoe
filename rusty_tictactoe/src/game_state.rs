use crate::player::Player;

pub struct GameState {
    board: [[Option<Player>; 3]; 3],
    current_player: Player,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board: [[None; 3]; 3],
            current_player: Player::X,
        }
    }
}

