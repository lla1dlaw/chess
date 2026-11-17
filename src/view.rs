use crate::App;
use crate::message::Message;

use iced::widget::{ center };
use iced::{ Element };


impl App {
    pub fn view(&self) -> Element<'_, Message> {
        center(self.board.view()).into()
    }
}
