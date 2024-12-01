use thiserror::Error;

use super::FenError;

/// An error that occurs when parsing a PGN string.
///
/// # Variants
/// * `InvalidFen`: The FEN string is invalid
/// * `InvalidPgn`: The PGN string is invalid
/// * `NoSuchFile`: The file does not exist
/// * `InvalidMetadata`: The metadata is invalid
///
#[derive(Debug, Error)]
pub enum PgnError {
    #[error("Invalid FEN: {0}")]
    InvalidFen(#[from] FenError),
    #[error("Invalid PGN: {0}")]
    InvalidPgn(String),
    #[error("No such file or directory: {0}")]
    NoSuchFile(#[from] std::io::Error),
    #[error("Invalid or not supported metadata: {0}")]
    InvalidMetadata(#[from] PgnMetadataError),
    #[error("Invalid or not variant: {0}")]
    InvalidVariant(String),
}

/// An error that occurs when parsing PGN metadata.
///
/// # Attributes
/// * `metadata` - The metadata that caused the error.
///
#[derive(Debug, Error)]
#[error("Invalid or not supported metadata: {metadata}")]
pub struct PgnMetadataError {
    pub metadata: String,
}

impl PgnMetadataError {
    /// Creates a new `PgnMetadataError` with the given metadata.
    ///
    /// # Arguments
    /// * `metadata` - The metadata that caused the error.
    ///
    /// # Example
    /// ```
    /// use chess_lab::errors::PgnMetadataError;
    ///
    /// let metadata = String::from("Invalid metadata");
    /// let error = PgnMetadataError::new(metadata);
    /// ```
    ///
    pub fn new(metadata: String) -> Self {
        PgnMetadataError { metadata }
    }
}
