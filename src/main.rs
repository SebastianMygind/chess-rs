mod chess_logic;

use chess_logic::chessboard::{
  ChessBoard,
};

fn main() {
    let mut chessboard = ChessBoard::new();
    chessboard.set_start_position();
}
