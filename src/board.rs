use iced::{ Font, Color, Element, Length };
use iced::widget::{ center, button, responsive, text, svg, Text };
use iced::widget::{ container, Grid, stack };
use iced::font::{ Weight };
use crate::pieces::{ Piece, PieceHandles, PieceKind, PieceColor };

const BOLD_FONT: Font = Font {
    weight: Weight::Bold,
    ..Font::DEFAULT
};

#[derive(Debug, Clone, Copy)]
pub enum BoardMessage {
    SquareClicked(usize, usize),
}

pub struct Board {
    pub turn: PieceColor,
    pub selected: Option<(usize, usize)>,
    files: [char; 8],
    ranks: [u8; 8],
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
        let files  = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let ranks = [ 1,   2,   3,   4,   5,   6,   7,   8 ];
        
        Self {
            turn: White,
            selected: None,
            squares: squares, 
            handles: PieceHandles::default(),
            files: files,
            ranks: ranks,
        }
    }
}


impl Board {
    pub fn rowcol_to_filerank(&self, row: usize, col: usize) -> (char, u8) {
        (self.files[col], self.ranks[row])
    }

    pub fn update(&mut self, message: BoardMessage) {
        match message {
            BoardMessage::SquareClicked(row, col) => {
                self.on_square_clicked(row, col);
            }
        }
    }

    fn on_square_clicked(&mut self, row: usize, col: usize) {
        match (self.selected, self.squares[row][col]) {
            // no square currently selected, and there is a piece clicked
            (_, Some(piece)) if piece.color == self.turn => {
                self.selected = Some((row, col));
            }

            // currently selected square is the square clicked
            (Some(from), _) if from == (row, col) => {
                self.selected = None;
            }

            (Some(_from), Some(piece)) if piece.color == self.turn => {
                self.selected = Some((row, col));
            }
            // a 'from' square is registered, and there is no white piece 
            (Some(from), _) => {
                self.move_piece(from, (row, col))
            }

            (_, _) => {}
        }

    }

    fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        // move piece to new location
        self.squares[to.0][to.1] = self.squares[from.0][from.1].take();
        self.selected = None; // deselect
        // toggle turn
        match self.turn {
            PieceColor::White => self.turn = PieceColor::Black,  
            PieceColor::Black => self.turn = PieceColor::White, 
        }
    }

    pub fn view(&self) -> Element<'_, BoardMessage> {
        responsive(|available| {
            let board_size    = (available.width.min(available.height) / 8.0).floor() * 8.0;
            let square_size   = board_size / 8.0;
            let piece_padding = square_size * 0.06; // scale padding proportional to board size/square size
            let label_size    = (square_size * 0.18).max(10.0);
            let light_color   = Color::from_rgb8(240, 217, 181);
            let dark_color    = Color::from_rgb8(181, 136, 99);

             
            // create board instance
            let mut board_grid = 
                Grid::new()
                .columns(8)
                .spacing(0);
            
            // populate board grid
            for row in 0..8 {
                for col in 0..8 {
                    let is_light = (row + col) % 2 == 0;
                    
                    let label_color = match is_light {
                        true => dark_color, // label is inverse color of background
                        _    => light_color,
                    };

                    let piece_layer: Element<'_, BoardMessage> = match self.squares[row][col] {
                        Some(piece) => container(
                            svg(self.handles.get(piece).clone())   
                                .width(Length::Fill)
                                .height(Length::Fill)
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(piece_padding)
                        .into(),
                        _ => text("").into(),
                    };

                    // layers for labels
                    let mut layers = stack![center(piece_layer)];

                    if col == 0 {
                        let rank_text: Text = text(self.ranks[row])
                            .size(label_size)
                            .font(BOLD_FONT)
                            .color(label_color);
                        
                        let rank_overlay = container(rank_text)
                            .padding([2.0, 4.0])
                            // automatically sets the length and height to fill as well
                            .align_left(Length::Fill) 
                            .align_top(Length::Fill);

                        layers = layers.push(rank_overlay);
                    }

                    if row == 7 {
                        let file_char = self.files[col];
                        let file_text = text(file_char)
                            .size(label_size)
                            .font(BOLD_FONT)
                            .color(label_color);

                        let file_overlay = container(file_text)
                            .padding([2.0, 4.0])
                            .align_right(Length::Fill)
                            .align_bottom(Length::Fill);

                        layers = layers.push(file_overlay);
                    }


                    let square = button(layers)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(0) // no padding so label is not pushed inward
                        .on_press(BoardMessage::SquareClicked(row, col))
                        .style(move |_theme, _status| {
                            let bg_color = if is_light {
                                light_color
                            } else {
                                dark_color
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
