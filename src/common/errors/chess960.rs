use thiserror::Error;

/// An error that occurs when an invalid Chess960 Starting Position ID is encountered
///
#[derive(Debug, Error, PartialEq)]
#[error("Invalid Chess960 ID: {spid}, must be between 0 and 959")]
pub struct Chess960SPIDError {
    /// The Chess960 ID that caused the error
    pub spid: u16,
}

impl Chess960SPIDError {
    /// Creates a new [Chess960SPIDError] with the given Chess960 Starting Position ID
    ///
    /// # Arguments
    /// * `spid` - The Chess960 Starting Position ID that caused the error
    ///
    /// # Example
    /// ```
    /// # use chess_lab::errors::Chess960SPIDError;
    /// let error = Chess960SPIDError::new(960);
    /// ```
    ///
    pub fn new(spid: u16) -> Self {
        Chess960SPIDError { spid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chess960_id_error_creation() {
        let error = Chess960SPIDError::new(960);
        assert_eq!(error.spid, 960);
        assert_eq!(
            error.to_string(),
            "Invalid Chess960 ID: 960, must be between 0 and 959"
        );
    }
}
