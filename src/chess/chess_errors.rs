use crate::chess::Move;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct IllegalMove {
    pub(crate) attempted_move: Move,
}

impl error::Error for IllegalMove {}
impl fmt::Display for IllegalMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Illegal move: {}", self.attempted_move.move_to_string())
    }
}

#[derive(Debug, Clone)]
pub struct InvalidFen;

impl error::Error for InvalidFen {}
impl fmt::Display for InvalidFen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid FEN string")
    }
}
