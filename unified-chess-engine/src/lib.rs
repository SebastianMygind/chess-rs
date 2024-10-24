use crate::array_engine::Square;

pub mod array_engine;
mod bitboard_engine;
mod fen;

pub struct Position {
    x: u8, // Position file.
    y: u8, // Position rank.
}

pub struct Move {
    
}

pub struct MoveError {
    
}

struct ArrayBoard {
    board: [[Square; 8]; 8],
}


trait ChessEngine {
    fn get_legal_moves<T>() -> Vec<Move>;
    fn perft(depth: i64) -> Vec<(String, i64)>;
    fn make_move(legal_moves: Move) -> Result<Move,MoveError>;
}
