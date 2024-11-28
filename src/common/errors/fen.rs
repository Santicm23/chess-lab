use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FenError {
    #[error("Invalid FEN: {0}")]
    InvalidFen(String),
}
