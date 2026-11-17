use crate::message::Message;
use crate::pieces::{ Piece, PieceHandles, PieceKind, PieceColor };
use iced::widget::{ center, button, responsive, text, Grid, svg };
use iced::{ Color, Element, Length };


pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    handles: PieceHandles,
}

impl Default for Board {
    fn default() -> Self {
        use PieceColor::*;
        use PieceKind::*;

        let back_rank = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
        let mut squares = [[None; 8]; 8];
        
        // populate board
        for col in 0..8 {
            squares[0][col] = Some(Piece { color: Black, kind: back_rank[col] });
            squares[1][col] = Some(Piece { color: Black, kind: Pawn });
            squares[6][col] = Some(Piece { color: White, kind: Pawn });
            squares[7][col] = Some(Piece { color: White, kind: back_rank[col] });
        }

        Self {
            squares, 
            handles: PieceHandles::default(),
        }
    }
}


impl Board {
    pub fn view(&self) -> Element<'_, Message> {
        responsive(|available| {
            let board_size = (available.width.min(available.height) / 8.0).floor() * 8.0;
            let square_padding = (board_size / 8.0) * 0.12; // scale padding proportional to board size
    
            // create board instance
            let mut board_grid = 
                Grid::new()
                .columns(8)
                .spacing(0);
            
            // populate board grid
            for row in 0..8 {
                for col in 0..8 {
                    let is_light = (row + col) % 2 == 0;
                    let content: Element<'_, Message> = match self.squares[row][col] {
                        Some(piece) => svg(self.handles.get(piece).clone())   
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .into(),
                        _ => text("").into(),
                    };

                    let square = button(center(content))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(square_padding)
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
