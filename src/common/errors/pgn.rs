use thiserror::Error;

use super::FenError;

#[derive(Debug, Error)]
pub enum PgnError {
    #[error("Invalid FEN: {0}")]
    InvalidFen(#[from] FenError),
    #[error("Invalid PGN: {0}")]
    InvalidPgn(String),
    #[error("No such file or directory: {0}")]
    NoSuchFile(#[from] std::io::Error),
}
