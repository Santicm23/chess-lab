use thiserror::Error;

/// An error that occurs when parsing a FEN string
///
#[derive(Debug, Error, PartialEq)]
#[error("Invalid FEN: {fen}")]
pub struct FenError {
    /// The FEN string that caused the error
    pub fen: String,
}

impl FenError {
    /// Creates a new [FenError] with the given FEN string
    ///
    /// # Arguments
    /// * `fen` - The FEN string that caused the error
    ///
    /// # Example
    /// ```
    /// use chess_lab::errors::FenError;
    ///
    /// let error = FenError::new("invalid_fen".to_string());
    /// ```
    ///
    pub fn new(fen: String) -> Self {
        FenError { fen }
    }
}

/// An error that occurs when an invalid piece representation is encountered
///
#[derive(Debug, Error, PartialEq)]
#[error("Invalid piece representation: {piece_repr}")]
pub struct PieceReprError {
    pub piece_repr: char,
}

impl PieceReprError {
    /// Creates a new [PieceReprError] with the given piece representation
    ///
    /// # Arguments
    /// * `piece_repr` - The piece representation that caused the error
    ///
    /// # Example
    /// ```
    /// use chess_lab::errors::PieceReprError;
    ///
    /// let error = PieceReprError::new('X');
    /// ```
    ///
    pub fn new(piece_repr: char) -> Self {
        PieceReprError { piece_repr }
    }
}
