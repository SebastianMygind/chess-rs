mod chess;
mod ui;

use crate::chess::fen::KIWIPETE_FEN_POSITION;
use crate::chess::{ChessBoard, MetaData, Move};
use crate::ui::game_state::ChessApplication;

fn main() -> iced::Result {
    let mut board = match ChessBoard::new_from_fen(KIWIPETE_FEN_POSITION) {
        Ok(chess_board) => chess_board,
        Err(e) => {
            println!("{e}");
            ChessBoard::new()
        }
    };

    ChessBoard::test_chessboard_perft_print_legal_moves(board, 1);

    iced::run(
        ChessApplication::title,
        ChessApplication::update,
        ChessApplication::view,
    )
}
