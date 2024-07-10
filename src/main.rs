use iced::widget::{container, text};
use iced::{Element, Length, Task, Theme};

pub fn main() -> iced::Result {
    iced::application("MyApp", MyApp::update, MyApp::view)
        .theme(MyApp::theme)
        .settings(iced::Settings::default())
        .run()
}

#[derive(Default)]
pub struct MyApp {
    theme: Theme,
}

#[derive(Debug)]
pub enum Message {}

impl MyApp {
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        container(text("Hello from iced!"))
            .center(Length::Fill)
            .into()
    }
}
