use crate::array_engine::Square;

pub mod array_engine;
mod bitboard_engine;
mod fen;

pub struct Position {
    x: u8, // Position file.
    y: u8, // Position rank.
}

pub struct Move {}

pub struct RatedMove {
    chess_move: Move,
    rating: i64,
}

pub struct MoveError {}

pub struct FenError {}

trait ChessEngine: ChessState {
    fn set_state(&mut self, fen_state: String) -> Result<(), FenError>;

    fn get_legal_moves(&self) -> Vec<Move>;

    fn get_best_moves(&self) -> Vec<RatedMove>;

    fn perft(depth: i64) -> Vec<(String, i64)>;
}

trait ChessState {
    fn internal_state_to_fen(&self) -> String;

    fn fen_to_internal_state(fen: String) -> Self;
}
