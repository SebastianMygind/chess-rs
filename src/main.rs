mod chess;
mod ui;

use crate::chess::{ChessBoard, MetaData, Move};
use crate::ui::game_state::ChessApplication;

fn main() -> iced::Result {
    let mut board = ChessBoard::new();
    let perft_depth = 1;

    let perft_count = board.perft(perft_depth);

    println!("perft({perft_depth}) = {perft_count}");

    iced::run(
        ChessApplication::title,
        ChessApplication::update,
        ChessApplication::view,
    )
}
