use crate::board::BoardMessage;

#[derive(Debug, Clone, Copy)]
pub enum Message { 
    Board(BoardMessage),
    ResetGame,
}
