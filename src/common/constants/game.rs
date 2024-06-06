use super::Position;

/// Represents the color of a chess piece.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// Represents the type of a chess piece.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// Represents the type of a move.
pub enum MoveType {
    Normal,
    Capture,
    Castle(CastleType),
    EnPassant(Position),
    Promotion(PieceType),
}

/// Represents the side of the board to castle on.
pub enum CastleType {
    KingSide,
    QueenSide,
}
