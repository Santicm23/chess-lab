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
    /// let error = MoveInfoError::new("Invalid move", mov);
    /// ```
    ///
    pub fn new(error: String, mov: Move) -> MoveInfoError {
        MoveInfoError { error, mov }
    }
}
