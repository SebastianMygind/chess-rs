mod chess;
mod ui;


use iced::{Application, Settings};

use crate::chess::ChessBoard;
use crate::chess::fen::{is_fen_valid, FEN_START_POS};
use crate::ui::game_state::GameState;

fn main() -> iced::Result {
    let mut board = ChessBoard::new();

    board.set_fen_position_arr(FEN_START_POS).expect("NOT VALID FEN");

    print!("{}", board);
    GameState::run(Settings::default())


}