mod chess_logic;

use iced::{Application, Element, Sandbox, Settings};
use iced::widget::{text};
use crate::chess_logic::chessboard::{ChessBoard, is_fen_valid};
use crate::chess_logic::chessboard;

fn main() -> iced::Result {
    let test_true = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let test_false = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR z KQkq - 0 1";

    let board = ChessBoard::new();


    println!("this should be true: {}", is_fen_valid(test_true));
    println!("this should be false: {}", is_fen_valid(test_false));

    GameState::run(Settings::default())

}

struct GameState;

#[derive(Debug)]
enum Message {}

impl Application for GameState {
    type Executor = ();
    type Message = Message;
    type Theme = ();
    type Flags = ();

    fn new() -> Self {
        Self
    }
    fn title(&self) -> String {
        String::from("Chess-rs")
    }
    fn update(&mut self, message: Message) {
        match message {}
    }
    fn view(&self) -> Element<'_, Message> {
        text("Hello World").into()
    }
}
