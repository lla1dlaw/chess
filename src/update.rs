use crate::App;
use crate::message::Message;

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            // forward messages to widget's update function
            Message::Board(board_msg) => {
                self.board.update(board_msg);
            }
            // handle application level messages
            Message::ResetGame => {
                self.board = Default::default();
            }
        }
    }
} 
