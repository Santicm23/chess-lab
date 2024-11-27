use crate::{
    constants::{MoveType, PieceType, Position},
    logic::Piece,
};

/// Errors that can occur when trying to move a piece
///
/// # Variants
/// * `Invalid`: The move is invalid
/// * `Illegal`: The move is illegal
/// * `Ambiguous`: The move is ambiguous
///
#[derive(Debug, PartialEq)]
pub enum MoveError {
    Invalid(String),
    Illegal(String),
    Ambiguous(String),
}

#[derive(Debug)]
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
