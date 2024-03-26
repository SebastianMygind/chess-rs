mod chess_logic;

use crate::chess_logic::chessboard::{
    ChessBoard,
    ChessBoard::is_fen_valid,

};

fn main() {
    let mut chess = ChessBoard::new();
    let validity = ChessBoard::is_valid_fen("no");


}
