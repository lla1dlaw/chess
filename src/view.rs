use crate::App;
use crate::message::Message;

use iced::widget::{ center };
use iced::{ Element };


impl App {
    pub fn view(&self) -> Element<'_, Message> {
        // '.map(Message::Board)' converts 'Element<BoardMessage>' -> 'Element<Message>'
        self.board.view().map(Message::Board) 
    }
}
