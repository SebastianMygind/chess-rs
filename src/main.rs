mod chess;
mod ui;

use iced::{Application, Settings};

use crate::chess::{ChessBoard, MetaData, Move};
use crate::ui::game_state::GameState;

fn main() -> iced::Result {
    let mut board = ChessBoard::new();

    let chess_move: Move = Move {
        start_pos: 12,
        end_pos: 20,
        meta_data: MetaData::PawnMove,
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

    GameState::run(Settings::default())
}
