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

impl PieceType {
    pub fn from_char(c: char) -> Option<PieceType> {
        match c {
            'P' => Some(PieceType::Pawn),
            'N' => Some(PieceType::Knight),
            'B' => Some(PieceType::Bishop),
            'R' => Some(PieceType::Rook),
            'Q' => Some(PieceType::Queen),
            'K' => Some(PieceType::King),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }
}

/// Represents the type of a move.
#[derive(Debug)]
pub enum MoveType {
    Normal {
        piece: PieceType,
        start: (Option<u8>, Option<u8>),
        end: Position,
        capture: bool,
        promotion: Option<PieceType>,
    },
    Castle {
        side: CastleType,
    },
    EnPassant {
        start: (Option<u8>, Option<u8>),
        end: Position,
    },
}

/// Represents the side of the board to castle on.
#[derive(Debug)]
pub enum CastleType {
    KingSide,
    QueenSide,
}
