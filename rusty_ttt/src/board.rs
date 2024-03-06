use tabled::Tabled;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamePiece {
    X,
    O,
}

#[derive(Clone, Default)]
pub struct Board {
    size: i32,
    spaces: Vec<Option<GamePiece>>,
}

impl Board {
    pub fn new(size: i32) -> Self {
        Board {
            size,
            spaces: vec![None; (size * size) as usize],
        }
    }

    pub fn place_marker(&mut self, index: usize, piece: GamePiece) -> bool {
        if index >= self.spaces.len() || self.spaces[index].is_some() {
            false
        } else {
            self.spaces[index] = Some(piece);
            true
        }
    }

    pub fn check_winner(&self) -> Option<GamePiece> {
        let lines = [
            // Rows
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            // Columns
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            // Diagonals
            (0, 4, 8),
            (2, 4, 6),
        ];

        for &(a, b, c) in &lines {
            if let (Some(pa), Some(pb), Some(pc)) = (self.spaces[a], self.spaces[b], self.spaces[c]) {
                if pa == pb && pb == pc {
                    return Some(pa);
                }
            }
        }

        None
    }

    pub fn is_full(&self) -> bool {
        self.spaces.iter().all(|space| space.is_some())
    }

    pub fn display(&self) {
        for (i, space) in self.spaces.iter().enumerate() {
            match space {
                Some(GamePiece::X) => print!(" X "),
                Some(GamePiece::O) => print!(" O "),
                None => print!("   "),
            }

            if (i + 1) as i32 % self.size == 0 {
                println!();
            } else {
                print!("|");
            }
        }
    }
}

