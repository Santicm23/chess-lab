use thiserror::Error;

/// An error that occurs when parsing a FEN string.
///
/// # Attributes
/// * `fen` - the FEN string that caused the error.
///
#[derive(Debug, Error, PartialEq)]
#[error("Invalid FEN: {fen}")]
pub struct FenError {
    pub fen: String,
}

impl FenError {
    /// Creates a new `FenError` with the given FEN string.
    ///
    /// # Arguments
    /// * `fen` - The FEN string that caused the error.
    ///
    pub fn new(fen: String) -> Self {
        FenError { fen }
    }
}

#[derive(Debug, Error, PartialEq)]
#[error("Invalid piece representation: {piece_repr}")]
pub struct PieceReprError {
    pub piece_repr: char,
}

impl PieceReprError {
    /// Creates a new `PieceReprError` with the given piece representation.
    ///
    /// # Arguments
    /// * `piece_repr` - The piece representation that caused the error.
    ///
    pub fn new(piece_repr: char) -> Self {
        PieceReprError { piece_repr }
    }
}
