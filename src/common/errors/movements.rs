use crate::{
    constants::{MoveType, PieceType, Position},
    logic::Piece,
};
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

#[derive(Debug, Error)]
#[error("Error moving piece: {error}")]
pub struct MoveInfoError {
    pub error: String,
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
