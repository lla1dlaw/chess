use crate::message::Message;
use crate::pieces::{ Piece, PieceHandles, PieceKind, PieceColor };
use iced::widget::{ center, button, responsive, text, Grid, svg };
use iced::{ Color, Element, Length };

#[derive(Debug, Clone, Copy)]
pub enum BoardMessage {
    SquareClicked(usize, usize),
}

pub struct Board {
    pub turn: PieceColor,
    pub selected: Option<(usize, usize)>,
    handles: PieceHandles,
    squares: [[Option<Piece>; 8]; 8],
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
            turn: White,
            selected: None,
            squares: squares, 
            handles: PieceHandles::default(),
        }
    }
}


impl Board {
    pub fn update(&mut self, message: BoardMessage) {
        match message {
            BoardMessage::SquareClicked(row, col) => {
                self.on_square_clicked(row, col);
            }
        }
    }

    pub fn on_square_clicked(&mut self, row: usize, col: usize) {
        let piece_clicked = self.squares[row][col];

        match self.selected {
            Some(from) => {
                match piece_clicked {
                    Some(piece) {
                        if from == (row, col) { self.selected = None; }
                        else if piece.color == self.turn {
                            self.selected = Some((row, col))
                        }
                    }
                    _ => {}
                }
            }
            _ => {
                match piece_clicked {
                    Some(piece) => {
                        if piece.color == self.turn { self.selected = Some((row, col)); }
                    },
                    _ => {}

                }
            }
        }
    }

    fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        // check if move is legal
        // apply move if it is
        todo!()
    }

    pub fn view(&self) -> Element<'_, BoardMessage> {
        responsive(|available| {
            let board_size = (available.width.min(available.height) / 8.0).floor() * 8.0;
            let square_padding = (board_size / 8.0) * 0.06; // scale padding proportional to board size
    
            // create board instance
            let mut board_grid = 
                Grid::new()
                .columns(8)
                .spacing(0);
            
            // populate board grid
            for row in 0..8 {
                for col in 0..8 {
                    let is_light = (row + col) % 2 == 0;
                    let content: Element<'_, BoardMessage> = match self.squares[row][col] {
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
                        .on_press(BoardMessage::SquareClicked(row, col))
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
