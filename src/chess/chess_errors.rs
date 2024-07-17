use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct IllegalMove;

impl error::Error for IllegalMove {}
impl fmt::Display for IllegalMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "illegal move")
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
