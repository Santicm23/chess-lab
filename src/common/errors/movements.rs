use thiserror::Error;

use crate::core::Move;

/// Errors that can occur when trying to move a [Piece](crate::core::Piece)
///
#[non_exhaustive]
#[derive(Debug, Error, PartialEq)]
pub enum MoveError {
    /// The move is invalid
    #[error("Invalid move: {0}")]
    Invalid(String),
    /// The move is illegal
    #[error("Illegal move: {0}")]
    Illegal(String),
    /// The move is ambiguous
    #[error("Ambiguous move: {0}")]
    Ambiguous(String),
}

/// Errors that can occur when trying to move a [Piece](crate::core::Piece)
///
#[derive(Debug, Error)]
#[error("Error moving piece: {error}")]
pub struct MoveInfoError {
    /// The error message
    pub error: String,
    /// The move that caused the error
    pub mov: Move,
}

impl MoveInfoError {
    /// Creates a new [MoveInfoError] with the given error message and [Move]
    ///
    /// # Arguments
    /// * `error` - The error message
    /// * `mov` - The [Move] that caused the error
    ///
    /// # Example
    /// ```
    /// # use chess_lab::errors::MoveInfoError;
    /// use chess_lab::core::{Color, PieceType, Square, Move, MoveType, Piece};
    ///
    /// let piece = Piece::new(Color::White, PieceType::Pawn);
    /// let from = Square::new(4, 1).unwrap();
    /// let to = Square::new(4, 3).unwrap();
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
    pub fn new(error: String, mov: Move) -> Self {
        MoveInfoError { error, mov }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Color, MoveType, Piece, PieceType, Square};

    use super::*;

    #[test]
    fn test_move_info_error_creation() {
        let piece = Piece::new(Color::White, PieceType::Pawn);
        let from = Square::new(4, 1).unwrap();
        let to = Square::new(4, 3).unwrap();
        let move_type = MoveType::Normal {
            capture: false,
            promotion: None,
        };
        let captured_piece = None;
        let rook_from = None;
        let ambiguity = (false, false);

        let mov = Move::new(
            piece,
            from,
            to,
            move_type,
            captured_piece,
            rook_from,
            ambiguity,
            false,
            false,
        )
        .unwrap();
        let error = MoveInfoError::new("Invalid move".to_string(), mov.clone());

        assert_eq!(error.error, "Invalid move");
        assert_eq!(error.mov, mov);
    }
}
