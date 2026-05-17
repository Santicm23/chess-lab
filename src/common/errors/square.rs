use thiserror::Error;

use crate::core::Square;

/// Error indicating that a specific [Square] is already occupied
///
#[derive(Debug, Error)]
#[error("Square {sqr} is already occupied")]
pub struct SquareOccupiedError {
    /// The [Square] that is already occupied
    pub sqr: Square,
}

impl SquareOccupiedError {
    /// Creates a new [SquareOccupiedError] with the given [Square]
    ///
    /// # Arguments
    /// * `Square` - The [Square] that is occupied
    ///
    /// # Example
    /// ```
    /// # use chess_lab::errors::SquareOccupiedError;
    /// use chess_lab::core::Square;
    ///
    /// let Square = Square::from_string("a1").unwrap();
    /// let error = SquareOccupiedError::new(Square);
    /// ```
    ///
    pub fn new(sqr: Square) -> Self {
        SquareOccupiedError { sqr }
    }
}

/// Error indicating that a specific [Square] is empty
///
#[derive(Debug, Error)]
#[error("Square {sqr} is empty")]
pub struct SquareEmptyError {
    /// The [Square] that is empty
    pub sqr: Square,
}

impl SquareEmptyError {
    /// Creates a new [SquareEmptyError] with the given [Square]
    ///
    /// # Arguments
    /// * `Square` - The [Square] that is empty
    ///
    /// # Example
    /// ```
    /// # use chess_lab::errors::SquareEmptyError;
    /// use chess_lab::core::Square;
    ///
    /// let Square = Square::from_string("a1").unwrap();
    /// let error = SquareEmptyError::new(Square);
    /// ```
    ///
    pub fn new(sqr: Square) -> Self {
        SquareEmptyError { sqr }
    }
}

/// Error indicating that a [Square] is out of the allowed range
///
#[derive(Debug, PartialEq, Error)]
#[error("Square ({file}, {rank}) is out of range")]
pub struct SquareOutOfRangeError {
    /// The file index that is out of range
    pub file: u8,
    /// The rank index that is out of range
    pub rank: u8,
}

impl SquareOutOfRangeError {
    /// Creates a new [SquareOutOfRangeError] with the given file and rank
    ///
    /// # Arguments
    /// * `file` - The file index that is out of range
    /// * `rank` - The rank index that is out of range
    ///
    /// # Example
    /// ```
    /// # use chess_lab::errors::SquareOutOfRangeError;
    /// let file = 8;
    /// let rank = 8;
    /// let error = SquareOutOfRangeError::new(file, rank);
    /// ```
    ///
    pub fn new(file: u8, rank: u8) -> Self {
        SquareOutOfRangeError { file, rank }
    }
}

/// Error indicating that a [Square] is invalid
///
#[derive(Debug, PartialEq, Error)]
#[error("Invalid square: {square_str}")]
pub struct SquareInvalidError {
    /// The string representation of the [Square] that is invali
    pub square_str: String,
}

impl SquareInvalidError {
    /// Creates a new [SquareInvalidError] with the given message
    ///
    /// # Arguments
    /// * `message` - The message that describes the error
    ///
    /// # Example
    /// ```
    /// # use chess_lab::errors::SquareInvalidError;
    /// let error = SquareInvalidError::new("Invalid Square".to_string());
    /// ```
    ///
    pub fn new(square_str: String) -> Self {
        SquareInvalidError { square_str }
    }
}

/// Error indicating that two [Squares](Square) are not aligned
///
#[derive(Debug, PartialEq, Error)]
#[error("Squares {sqr1} and {sqr2} are not aligned")]
pub struct UnalignedSquaresError {
    /// The first [Square] that is not aligned
    pub sqr1: Square,
    /// The second [Square] that is not aligned
    pub sqr2: Square,
}

impl UnalignedSquaresError {
    /// Creates a new [UnalignedSquaresError] with the given Squares
    ///
    /// # Arguments
    /// * `Square1` - The first [Square] that is not aligned
    /// * `Square2` - The second [Square] that is not aligned
    ///
    /// # Example
    /// ```
    /// # use chess_lab::errors::UnalignedSquaresError;
    /// use chess_lab::core::Square;
    ///
    /// let Square1 = Square::from_string("a1").unwrap();
    /// let Square2 = Square::from_string("b3").unwrap();
    ///
    /// let error = UnalignedSquaresError::new(Square1, Square2);
    /// ```
    ///
    pub fn new(sqr1: Square, sqr2: Square) -> Self {
        UnalignedSquaresError { sqr1, sqr2 }
    }
}

/// Error types for [Square] between operations
///
#[non_exhaustive]
#[derive(Debug, PartialEq, Error)]
pub enum SquareBetweenError {
    /// Indicates that two [Squares](Square) are not aligned and they should be
    #[error(transparent)]
    Unaligned(#[from] UnalignedSquaresError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_occupied_error() {
        let sqr = Square::from_string("a1").unwrap();
        let error = SquareOccupiedError::new(sqr);

        assert_eq!(error.sqr, sqr);
        assert_eq!(format!("{}", error), "Square a1 is already occupied");
    }

    #[test]
    fn test_square_empty_error() {
        let sqr = Square::from_string("a1").unwrap();
        let error = SquareEmptyError::new(sqr);

        assert_eq!(error.sqr, sqr);
        assert_eq!(format!("{}", error), "Square a1 is empty");
    }

    #[test]
    fn test_square_out_of_range_error() {
        let file = 8;
        let rank = 8;
        let error = SquareOutOfRangeError::new(file, rank);

        assert_eq!(error.file, file);
        assert_eq!(error.rank, rank);
        assert_eq!(format!("{}", error), "Square (8, 8) is out of range");
    }

    #[test]
    fn test_square_invalid_error() {
        let square_str = "abc".to_string();
        let error = SquareInvalidError::new(square_str.clone());

        assert_eq!(error.square_str, square_str);
        assert_eq!(format!("{}", error), "Invalid Square: abc");
    }

    #[test]
    fn test_unaligned_squares_error() {
        let sqr1 = Square::from_string("a1").unwrap();
        let sqr2 = Square::from_string("b3").unwrap();
        let error = UnalignedSquaresError { sqr1, sqr2 };

        assert_eq!(error.sqr1, sqr1);
        assert_eq!(error.sqr2, sqr2);
        assert_eq!(format!("{}", error), "Squares a1 and b3 are not aligned");
    }
}
