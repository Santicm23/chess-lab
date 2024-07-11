use std::fmt::{self, Display, Error, Formatter};

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
    /// # Example
    /// ```
    /// use chess_lab::constants::Color;
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
    /// # Example
    /// ```
    /// use chess_lab::constants::PieceType;
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
    /// # Example
    /// ```
    /// use chess_lab::constants::PieceType;
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

/// Represents the status of a chess game
///
/// # Variants
/// * `InProgress`: The game is in progress
/// * `Draw`: The game is a draw
///     - `reason`: The reason for the draw
/// * `WhiteWins`: White wins the game
///     - `reason`: The reason for the win
/// * `BlackWins`: Black wins the game
///     - `reason`: The reason for the win
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameStatus {
    InProgress,
    Draw(DrawReason),
    WhiteWins(WinReason),
    BlackWins(WinReason),
}

/// Represents the reason for a draw
///
/// # Variants
/// * `Stalemate`: The game is a stalemate
/// * `InsufficientMaterial`: The game is a draw due to insufficient material
/// * `ThreefoldRepetition`: The game is a draw due to threefold repetition
/// * `FiftyMoveRule`: The game is a draw due to the fifty move rule
/// * `Agreement`: The game is a draw due to agreement
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DrawReason {
    Stalemate,
    InsufficientMaterial,
    ThreefoldRepetition,
    FiftyMoveRule,
    Agreement,
}

impl Display for DrawReason {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DrawReason::Stalemate => write!(f, "Stalemate"),
            DrawReason::InsufficientMaterial => write!(f, "Insufficient material"),
            DrawReason::ThreefoldRepetition => write!(f, "Threefold repetition"),
            DrawReason::FiftyMoveRule => write!(f, "Fifty move rule"),
            DrawReason::Agreement => write!(f, "Agreement"),
        }
    }
}

/// Represents the reason for a win
///
/// # Variants
/// * `Checkmate`: The game is a win due to checkmate
/// * `Resignation`: The game is a win due to resignation
/// * `Time`: The game is a win due to time
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WinReason {
    Checkmate,
    Resignation,
    Time,
}

impl Display for WinReason {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            WinReason::Checkmate => write!(f, "Checkmate"),
            WinReason::Resignation => write!(f, "Resignation"),
            WinReason::Time => write!(f, "Time"),
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
///    - The move is an en passant
///
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum CastleType {
    KingSide,
    QueenSide,
}

/// Represents a move in a chess game
///
/// # Example
/// ```
/// use chess_lab::constants::{Color, PieceType, Position, Move, MoveType};
/// use chess_lab::logic::Piece;
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
/// let ambiguity = (false, false);
/// let mv = Move::new(
///     piece,
///     from,
///     to,
///     move_type,
///     captured_piece,
///     rook_from,
///     ambiguity,
///     false,
///     false
/// );
///
/// assert_eq!(mv.to_string(), "e4");
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub piece: Piece,
    pub from: Position,
    pub to: Position,
    pub move_type: MoveType,
    pub captured_piece: Option<PieceType>,
    pub rook_from: Option<Position>,
    pub ambiguity: (bool, bool),
    pub check: bool,
    pub checkmate: bool,
}

impl Move {
    /// Creates a new move
    ///
    /// # Arguments
    /// * `piece`: The piece that is moving
    /// * `from`: The position the piece is moving from
    /// * `to`: The position the piece is moving to
    /// * `move_type`: The type of the move
    /// * `captured_piece`: The piece that is captured, if any
    /// * `rook_from`: The position of the rook, if the move is a castle
    /// * `ambiguity`: A tuple of booleans representing the ambiguity of the move
    /// * `check`: Whether the move puts the opponent in check
    /// * `checkmate`: Whether the move puts the opponent in checkmate
    ///
    /// # Panics
    /// Panics if the move is a capture, but no captured piece is provided
    /// Panics if the move is not a capture, but a captured piece is provided
    /// Panics if the move is a promotion, but the piece is not a pawn
    /// Panics if the move is a castle, but the piece is not a king
    /// Panics if the move is a castle, but the rook position is not provided
    /// Panics if the move is an en passant, but the piece is not a pawn
    ///
    /// # Example
    /// ```
    /// use chess_lab::constants::{Color, PieceType, Position, Move, MoveType};
    /// use chess_lab::logic::Piece;
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
    /// let ambiguity = (false, false);
    /// let mv = Move::new(
    ///     piece,
    ///     from,
    ///     to,
    ///     move_type,
    ///     captured_piece,
    ///     rook_from,
    ///     ambiguity,
    ///     false,
    ///     false
    /// );
    /// ```
    ///
    pub fn new(
        piece: Piece,
        from: Position,
        to: Position,
        move_type: MoveType,
        captured_piece: Option<PieceType>,
        rook_from: Option<Position>,
        ambiguity: (bool, bool),
        check: bool,
        checkmate: bool,
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
                    piece.piece_type == PieceType::King,
                    "The move is a castle, but the piece is not a king"
                );
                assert!(
                    rook_from.is_some(),
                    "The move is a castle, but no rook position is provided"
                );
            }
            MoveType::EnPassant => {
                assert!(
                    piece.piece_type == PieceType::Pawn,
                    "The move is an en passant, but the piece is not a pawn"
                );
            }
        }
        Move {
            piece,
            from,
            to,
            move_type,
            captured_piece,
            rook_from,
            ambiguity,
            check,
            checkmate,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut result = String::new();
        if self.piece.piece_type != PieceType::Pawn {
            result.push(self.piece.piece_type.to_char());
        }
        match &self.move_type {
            MoveType::Castle { side } => {
                result = match side {
                    CastleType::KingSide => "O-O".to_string(),
                    CastleType::QueenSide => "O-O-O".to_string(),
                };
            }
            MoveType::Normal { capture, promotion } => {
                let from_string = self.from.to_string();
                if self.ambiguity.0 || (PieceType::Pawn == self.piece.piece_type && *capture) {
                    result.push(from_string.chars().nth(0).unwrap());
                }
                if self.ambiguity.1 {
                    result.push(from_string.chars().nth(1).unwrap());
                }
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
        if self.checkmate {
            result.push('#');
        } else if self.check {
            result.push('+');
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let mov1 = Move {
            piece: Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
            },
            from: Position::new(4, 1),
            to: Position::new(4, 3),
            move_type: MoveType::Normal {
                capture: false,
                promotion: None,
            },
            captured_piece: None,
            rook_from: None,
            ambiguity: (false, false),
            check: false,
            checkmate: false,
        };
        let mov2 = Move {
            piece: Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
            },
            from: Position::new(4, 1),
            to: Position::new(4, 3),
            move_type: MoveType::Normal {
                capture: false,
                promotion: None,
            },
            captured_piece: None,
            rook_from: None,
            ambiguity: (false, false),
            check: false,
            checkmate: false,
        };

        assert_eq!(mov1, mov2);
    }
}
