use iced::widget::{container, row, text};
use iced::Element;
use iced::Theme;

#[derive(Default)]
pub struct GameState;

#[derive(Clone, Copy, Debug)]
struct Square {}
#[derive(Clone, Copy, Debug)]
pub enum Message {
    Click(Square),
}

impl GameState {
    pub fn title(&self) -> String {
        "Chess-rs".to_string()
    }

    pub fn update(&mut self, message: Message) {}

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
