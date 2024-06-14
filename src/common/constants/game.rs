use crate::logic::Piece;

use super::Position;

/// Represents the color of a chess piece
///
/// # Variants
/// * `White`: The white color
/// * `Black`: The black color
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Gets the opposite color
    ///
    /// # Returns
    /// The color opposite to the current one
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::Color;
    ///
    /// assert_eq!(Color::White.opposite(), Color::Black);
    /// assert_eq!(Color::Black.opposite(), Color::White);
    /// ```
    ///
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// Represents the type of a chess piece
///
/// # Variants
/// * `Pawn`: A pawn
/// * `Knight`: A knight
/// * `Bishop`: A bishop
/// * `Rook`: A rook
/// * `Queen`: A queen
/// * `King`: A king
///
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
    /// Gets the piece type from a character
    ///
    /// # Arguments
    /// * `c`: The character to convert (only valid uppercase characters)
    ///
    /// # Returns
    /// The piece type if the character is valid, otherwise `None`
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::PieceType;
    ///
    /// assert_eq!(PieceType::from_char('P'), Some(PieceType::Pawn));
    /// assert_eq!(PieceType::from_char('N'), Some(PieceType::Knight));
    /// assert_eq!(PieceType::from_char('B'), Some(PieceType::Bishop));
    /// assert_eq!(PieceType::from_char('R'), Some(PieceType::Rook));
    /// assert_eq!(PieceType::from_char('Q'), Some(PieceType::Queen));
    /// assert_eq!(PieceType::from_char('K'), Some(PieceType::King));
    ///
    /// assert_eq!(PieceType::from_char('x'), None);
    /// assert_eq!(PieceType::from_char('y'), None);
    /// assert_eq!(PieceType::from_char('p'), None);
    /// ```
    ///
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

    /// Gets the character representation of the piece type
    ///
    /// # Returns
    /// The character representation of the piece type
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::PieceType;
    ///
    /// assert_eq!(PieceType::Pawn.to_char(), 'P');
    /// assert_eq!(PieceType::Knight.to_char(), 'N');
    /// assert_eq!(PieceType::Bishop.to_char(), 'B');
    /// assert_eq!(PieceType::Rook.to_char(), 'R');
    /// assert_eq!(PieceType::Queen.to_char(), 'Q');
    /// assert_eq!(PieceType::King.to_char(), 'K');
    /// ```
    ///
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

/// Represents the type of a move
///
/// # Variants
/// * `Normal`: A normal move
///     - `capture`: Whether the move is a capture
///     - `promotion`: The piece type to promote to
/// * `Castle`: A castle move
///     - `side`: The side of the board to castle on
/// * `EnPassant`: An en passant move
#[derive(Debug, Clone)]
pub enum MoveType {
    Normal {
        capture: bool,
        promotion: Option<PieceType>,
    },
    Castle {
        side: CastleType,
    },
    EnPassant,
}

/// Represents the side of the board to castle on
///
/// # Variants
/// * `KingSide`: The king side
/// * `QueenSide`: The queen side
///
#[derive(Debug, Clone)]
pub enum CastleType {
    KingSide,
    QueenSide,
}

/// Represents a move in a chess game
///
/// # Examples:
/// ```
/// use chess_lib::constants::{Color, PieceType, Position, Move, MoveType};
/// use chess_lib::logic::Piece;
///
/// let piece = Piece {
///     color: Color::White,
///     piece_type: PieceType::Pawn,
/// };
/// let from = Position::new(4, 1);
/// let to = Position::new(4, 3);
/// let move_type = MoveType::Normal {
///     capture: false,
///     promotion: None,
/// };
/// let captured_piece = None;
/// let rook_from = None;
/// let mv = Move::new(piece, from, to, move_type, captured_piece, rook_from);
///
/// assert_eq!(mv.to_string(), "e2e4");
/// ```
///
#[derive(Debug, Clone)]
pub struct Move {
    pub piece: Piece,
    pub from: Position,
    pub to: Position,
    pub move_type: MoveType,
    pub captured_piece: Option<PieceType>,
    pub rook_from: Option<Position>,
}

impl Move {
    pub fn new(
        piece: Piece,
        from: Position,
        to: Position,
        move_type: MoveType,
        captured_piece: Option<PieceType>,
        rook_from: Option<Position>,
    ) -> Move {
        match &move_type {
            MoveType::Normal { capture, promotion } => {
                if *capture {
                    assert!(
                        captured_piece.is_some(),
                        "The move is a capture, but no captured piece is provided"
                    );
                } else {
                    assert!(
                        captured_piece.is_none(),
                        "The move is not a capture, but a captured piece is provided"
                    );
                }
                if promotion.is_some() {
                    assert!(
                        piece.piece_type == PieceType::Pawn,
                        "The move is a promotion, but the piece is not a pawn"
                    );
                }
            }
            MoveType::Castle { side: _ } => {
                assert!(
                    rook_from.is_some(),
                    "The move is a castle, but no rook position is provided"
                );
            }
            _ => (),
        }
        Move {
            piece,
            from,
            to,
            move_type,
            captured_piece,
            rook_from,
        }
    }
}

impl ToString for Move {
    fn to_string(&self) -> String {
        let mut result = String::new();
        if self.piece.piece_type != PieceType::Pawn {
            result.push(self.piece.piece_type.to_char());
        }
        match &self.move_type {
            MoveType::Castle { side } => {
                return match side {
                    CastleType::KingSide => "O-O".to_string(),
                    CastleType::QueenSide => "O-O-O".to_string(),
                };
            }
            MoveType::Normal { capture, promotion } => {
                result.push_str(&self.from.to_string());
                if *capture {
                    result.push('x');
                }
                result.push_str(&self.to.to_string());
                if let Some(promotion) = promotion {
                    result.push('=');
                    result.push(promotion.to_char());
                }
            }
            MoveType::EnPassant => {
                result.push_str(&self.from.to_string());
                result.push('x');
                result.push_str(&self.to.to_string());
            }
        }
        result
    }
}
