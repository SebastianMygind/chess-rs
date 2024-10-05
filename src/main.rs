mod chess;
mod ui;

use crate::chess::{ChessBoard, MetaData, Move};
use crate::ui::game_state::ChessApplication;

fn main() -> iced::Result {
    let mut board = ChessBoard::new();

    let chess_move: Move = Move {
        start_pos: (4, 1),
        end_pos: (4, 3),
        meta_data: MetaData::PawnDoubleMove,
    };

    match board.make_move(chess_move) {
        Ok(move_to_make) => {
            println!("{move_to_make}")
        }
        Err(e) => {
            println!("ERROR: {}", e)
        }
    }
    print!("{}", board);

    iced::run(
        ChessApplication::title,
        ChessApplication::update,
        ChessApplication::view,
    )
}
