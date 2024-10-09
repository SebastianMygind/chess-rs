use crate::chess::ChessBoard;
use iced::widget::{container, row, text};
use iced::Element;
use iced::Theme;

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
    game_instance: Option<GameState>,
}

struct GameState {
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
    pub fn title(&self) -> String {
        String::from("Chess-rs")
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::StartNewGame => {}

            Message::QuitGame => {}

            Message::ClickSquare(coordinate) => {}

            Message::MakeMove(chess_move) => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(row![
            text("This should be left"),
            text("This should be right")
        ])
        .padding(20)
        .into()
    }
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}

const PIECES: [&str; 12] = [
    "bB.svg", "bK.svg", "bN.svg", "bP.svg", "bQ.svg", "bR.svg", "wB.svg", "wK.svg", "wN.svg",
    "wP.svg", "wQ.svg", "wR.svg",
];
