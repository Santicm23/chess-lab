use thiserror::Error;

use super::FenError;

/// An error that occurs when parsing a PGN string
///
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PGNError {
    /// The FEN string is invalid
    #[error("Invalid FEN: {0}")]
    InvalidFen(#[from] FenError),
    /// The PGN string is invalid
    #[error("Invalid PGN: {0}")]
    InvalidPgn(String),
    /// The file does not exist
    #[error("No such file or directory: {0}")]
    NoSuchFile(#[from] std::io::Error),
    /// The metadata is invalid
    #[error("Invalid or not supported metadata: {0}")]
    InvalidMetadata(#[from] PGNMetadataError),
    /// The variant does not exists
    #[error("Invalid or not variant provided: {0}")]
    InvalidVariant(String),
}

/// An error that occurs when parsing PGN metadata
///
/// # Attributes
/// * `metadata` - The metadata that caused the error
///
#[derive(Debug, Error)]
#[error("Invalid or not supported metadata: {metadata}")]
pub struct PGNMetadataError {
    pub metadata: String,
}

impl PGNMetadataError {
    /// Creates a new [PGNMetadataError] with the given metadata
    ///
    /// # Arguments
    /// * `metadata` - The metadata that caused the error
    ///
    /// # Example
    /// ```
    /// # use chess_lab::errors::PGNMetadataError;
    /// let error = PGNMetadataError::new("Invalid metadata".to_string());
    /// ```
    ///
    pub fn new(metadata: String) -> Self {
        PGNMetadataError { metadata }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pgn_metadata_error_new() {
        let metadata = "Invalid metadata".to_string();
        let error = PGNMetadataError::new(metadata.clone());
        assert_eq!(error.metadata, metadata);
    }
}
