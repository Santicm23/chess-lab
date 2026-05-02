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
    /// # use chess_lab::errors::FenError;
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
    /// The piece character that caused the error
    pub piece_repr: char,
}

impl PieceReprError {
    /// Creates a new [PieceReprError] with the given [Piece](crate::core::Piece) representation
    ///
    /// # Arguments
    /// * `piece_repr` - The char representation of a [Piece](crate::core::Piece) that caused the error
    ///
    /// # Example
    /// ```
    /// # use chess_lab::errors::PieceReprError;
    /// let error = PieceReprError::new('X');
    /// ```
    ///
    pub fn new(piece_repr: char) -> Self {
        PieceReprError { piece_repr }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen_error_creation() {
        let fen = "invalid_fen".to_string();
        let error = FenError::new(fen.clone());
        assert_eq!(error.fen, fen);
        assert_eq!(format!("{}", error), format!("Invalid FEN: {}", fen));
    }

    #[test]
    fn test_piece_repr_error_creation() {
        let piece_repr = 'X';
        let error = PieceReprError::new(piece_repr);
        assert_eq!(error.piece_repr, piece_repr);
        assert_eq!(
            format!("{}", error),
            format!("Invalid piece representation: {}", piece_repr)
        );
    }
}
