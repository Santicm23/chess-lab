/// Represents the color of a chess piece.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    WHITE,
    BLACK,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        }
    }
}

/// Represents the type of a chess piece.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}
