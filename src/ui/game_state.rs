use iced::widget::text;
use iced::{executor, Application, Command, Element, Theme};

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
        text("Hello World").into()
    }
}
