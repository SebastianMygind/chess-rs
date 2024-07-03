mod chess;
mod ui;


use iced::{Application, Settings};

use crate::chess::ChessBoard;
use crate::chess::fen::{is_fen_valid};
use crate::ui::game_state::GameState;

fn main() -> iced::Result {
    let board = ChessBoard::new();

    print!("{}", board);
    GameState::run(Settings::default())


}