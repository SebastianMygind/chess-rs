use iced::executor;
use iced::widget::{container, row, text};
use iced::Application;
use iced::Command;
use iced::Element;
use iced::Theme;

pub struct GameState;

#[derive(Debug)]
pub enum Message {}

impl Application for GameState {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self, Command::none())
    }
    fn title(&self) -> String {
        String::from("Chess-rs")
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {}
        Command::none()
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

// fn iced_chessboard(board_data: [BoardPiece; ARR_SIZE]) -> iced::widget::container {}
