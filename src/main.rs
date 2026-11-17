mod message;
mod update;
mod view;
mod board;
mod pieces;

use board::Board;
use iced::{ Size, window };

#[derive(Default)]
pub struct App {
    pub board: Board,
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .window(window::Settings {
            size: Size::new(600.0, 600.0),
            min_size: Some(Size::new(256.0, 256.0)),
            ..Default::default()
        })
        .run()
}
