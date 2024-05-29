/// Represents the color of a chess piece.
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

/// implement clone
impl Clone for Color {
    fn clone(&self) -> Color {
        match self {
            Color::WHITE => Color::WHITE,
            Color::BLACK => Color::BLACK,
        }
    }
}

impl Copy for Color {}

/// Represents the type of a chess piece.
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}
