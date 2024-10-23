pub mod cli;

use iced;
use iced::application::Update;
use iced::widget::{container, row, text};
use iced::Element;
use iced::Result;
use iced::Theme;
use unified_chess_engine::array_engine::{ChessBoard, Piece, PieceType, Position};

struct UserMove {
    start_position: Position,
    end_position: Position,
    promotion_piece: Option<PieceType>,
}

enum ColoredPieces {
    WKing,
    BKing,
    WQueen,
    BQueen,
    WRook,
    BRook,
    WBishop,
    BBishop,
    WKnight,
    BKnight,
    WPawn,
    BPawn,
}

#[derive(Default)]
pub struct ChessApplication {
    pub game_instance: Option<GameState>,
}

pub struct GameState {
    selected_square: Option<Coordinate>,
    chess_board: ChessBoard,
}

#[derive(Clone, Copy, Debug)]
struct Move {
    start_position: Coordinate,
    end_position: Coordinate,
}

#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    StartNewGame,
    QuitGame,
    ClickSquare(Coordinate),
    MakeMove(Move),
}

impl ChessApplication {
    pub fn run(&mut self) -> Result {
        iced::run(Self::title, Self::update, Self::view)
    }
    fn title(&self) -> String {
        String::from("Chess-rs")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::StartNewGame => {}

            Message::QuitGame => {}

            Message::ClickSquare(coordinate) => {}

            Message::MakeMove(chess_move) => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(row![
            text("This should be left"),
            text("This should be right")
        ])
        .padding(20)
        .into()
    }
    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

const PIECES: [&str; 12] = [
    "bB.svg", "bK.svg", "bN.svg", "bP.svg", "bQ.svg", "bR.svg", "wB.svg", "wK.svg", "wN.svg",
    "wP.svg", "wQ.svg", "wR.svg",
];
