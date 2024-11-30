use crate::constants::Move;
use thiserror::Error;

/// Errors that can occur when trying to move a piece
///
/// # Variants
/// * `Invalid`: The move is invalid
/// * `Illegal`: The move is illegal
/// * `Ambiguous`: The move is ambiguous
///
#[derive(Debug, Error, PartialEq)]
pub enum MoveError {
    #[error("Invalid move: {0}")]
    Invalid(String),
    #[error("Illegal move: {0}")]
    Illegal(String),
    #[error("Ambiguous move: {0}")]
    Ambiguous(String),
}

/// Errors that can occur when trying to move a piece
///
/// # Arguments
/// * `error` - The error message
/// * `mov` - The move that caused the error
///
#[derive(Debug, Error)]
#[error("Error moving piece: {error}")]
pub struct MoveInfoError {
    pub error: String,
    pub mov: Move,
}

impl MoveInfoError {
    /// Creates a new `MoveInfoError` with the given error message and move.
    ///
    /// # Arguments
    /// * `error` - The error message
    /// * `mov` - The move that caused the error
    ///
    /// # Example
    ///
    /// ```
    /// use chess_lab::constants::{Color, PieceType, Position, Move, MoveType};
    /// use chess_lab::logic::Piece;
    /// use chess_lab::errors::MoveInfoError;
    ///
    /// let piece = Piece {
    ///     color: Color::White,
    ///     piece_type: PieceType::Pawn,
    /// };
    /// let from = Position::new(4, 1).unwrap();
    /// let to = Position::new(4, 3).unwrap();
    /// let move_type = MoveType::Normal {
    ///     capture: false,
    ///     promotion: None,
    /// };
    /// let captured_piece = None;
    /// let rook_from = None;
    /// let ambiguity = (false, false);
    ///
    /// let mov = Move::new(
    ///     piece,
    ///     from,
    ///     to,
    ///     move_type,
    ///     captured_piece,
    ///     rook_from,
    ///     ambiguity,
    ///     false,
    ///     false
    /// ).unwrap();
    /// let error = MoveInfoError::new("Invalid move".to_string(), mov);
    /// ```
    ///
    pub fn new(error: String, mov: Move) -> MoveInfoError {
        MoveInfoError { error, mov }
    }
}
