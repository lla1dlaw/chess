use iced::widget::svg;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook, 
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub kind:  PieceKind,
}


pub struct PieceHandles {
    pub w_pawn:   svg::Handle,
    pub w_knight: svg::Handle,
    pub w_bishop: svg::Handle,
    pub w_rook:   svg::Handle,
    pub w_queen:  svg::Handle,
    pub w_king:   svg::Handle,

    pub b_pawn:   svg::Handle,
    pub b_knight: svg::Handle,
    pub b_bishop: svg::Handle,
    pub b_rook:   svg::Handle,
    pub b_queen:  svg::Handle,
    pub b_king:   svg::Handle,
}

impl Default for PieceHandles {
    fn default() -> Self {
        Self {
            w_pawn: svg::Handle::from_memory(include_bytes!("../assets/pieces/cburnett/wp.svg")),
            w_knight: svg::Handle::from_memory(include_bytes!("../assets/pieces/cburnett/wk.svg")),
            w_bishop: svg::Handle::from_memory(include_bytes!("../assets/pieces/cburnett/wb.svg")),
            w_rook: svg::Handle::from_memory(include_bytes!("../assets/pieces/cburnett/wr.svg")),
            w_queen: svg::Handle::from_memory(include_bytes!("../assets/pieces/cburnett/wq.svg")),
            w_king: svg::Handle::from_memory(include_bytes!("../assets/pieces/cburnett/wk.svg")),

            b_pawn: svg::Handle::from_memory(include_bytes!("../assets/pieces/cburnett/bp.svg")),
            b_knight: svg::Handle::from_memory(include_bytes!("../assets/pieces/cburnett/bk.svg")),
            b_bishop: svg::Handle::from_memory(include_bytes!("../assets/pieces/cburnett/bb.svg")),
            b_rook: svg::Handle::from_memory(include_bytes!("../assets/pieces/cburnett/br.svg")),
            b_queen: svg::Handle::from_memory(include_bytes!("../assets/pieces/cburnett/bq.svg")),
            b_king: svg::Handle::from_memory(include_bytes!("../assets/pieces/cburnett/bk.svg")),
        }
    }
}

impl PieceHandles {
    pub fn get(&self, piece: Piece) -> &svg::Handle {
        use PieceColor::*;
        use PieceKind::*;

        match (piece.color, piece.kind) {
            (White, Pawn)   => &self.w_pawn,
            (White, Knight) => &self.w_knight,
            (White, Bishop) => &self.w_bishop,
            (White, Rook)   => &self.w_rook,
            (White, Queen)  => &self.w_queen,
            (White, King)   => &self.w_king,

            (Black, Pawn)   => &self.b_pawn,
            (Black, Knight) => &self.b_knight,
            (Black, Bishop) => &self.b_bishop,
            (Black, Rook)   => &self.b_rook,
            (Black, Queen)  => &self.b_queen,
            (Black, King)   => &self.b_king,
        }
    }
}
