mod chess;
mod ui;

use iced::{Application, Settings};

use crate::chess::fen::{is_fen_valid, FEN_START_POS};
use crate::chess::{ChessBoard, Move};
use crate::ui::game_state::GameState;

fn main() -> iced::Result {
    let mut board = ChessBoard::new();

    board
        .set_fen_position_arr(FEN_START_POS)
        .expect("NOT VALID FEN");

    let test_move = Move {
        start_pos: [5, 2],
        end_pos: [5, 4],
    };

    board.make_move(test_move);

    print!("{}", board);
    GameState::run(Settings::default())
}
