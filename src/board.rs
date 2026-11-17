use crate::message::Message;
use iced::widget::{ center, button, responsive, Grid };
use iced::{ Color, Element, Length };

#[derive(Default)]
pub struct Board {
    // add pieces, current board state, currently selected pieces, moves, etc.
}


impl Board {
    pub fn view(&self) -> Element<'_, Message> {
        responsive(|available| {
            let board_size = available.width.min(available.height);
    
            // create board instance
            let mut board_grid = 
                Grid::new()
                .columns(8)
                .spacing(0);
            
            // populate board grid
            for row in 0..8 {
                for col in 0..8 {
                    let is_light = (row + col) % 2 == 0;

                    let square = button(center(""))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(move |_theme, _status| {
                            let bg_color = if is_light {
                                Color::from_rgb8(240, 217, 181)
                            } else {
                                Color::from_rgb8(181, 136, 99)
                            };

                            button::Style {
                                background: Some(bg_color.into()),
                                ..button::Style::default()
                            }
                        });

                    board_grid = board_grid.push(square);
                }
            }
            center(
                board_grid
                .width(board_size)
                .height(board_size)
            ).into()
        })
        .into()
    }
}
