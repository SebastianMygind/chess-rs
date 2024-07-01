mod chess;
mod ui;


use iced::{Application, Settings};

use crate::chess::ChessBoard;
use crate::chess::fen::is_fen_valid;
use crate::ui::game_state::GameState;

fn main() -> iced::Result {
    let test_true = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let test_false = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR z KQkq - 0 1";

    let board = ChessBoard::new();


    println!("this should be true: {}", is_fen_valid(test_true));
    println!("this should be false: {}", is_fen_valid(test_false));

    GameState::run(Settings::default())

}