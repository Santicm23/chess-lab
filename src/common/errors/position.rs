use thiserror::Error;

use crate::core::Position;

/// Error indicating that a specific [position](Position) is already occupied
///
/// TODO add example
#[derive(Debug, Error)]
#[error("Position {position} is already occupied")]
pub struct PositionOccupiedError {
    /// The position that is already occupied
    pub position: Position,
}

impl PositionOccupiedError {
    /// Creates a new `PositionOccupiedError` with the given [position](Position)
    ///
    /// # Arguments
    /// * `position` - The [position](Position) that is occupied
    ///
    /// # Example
    /// ```
    /// use chess_lab::errors::PositionOccupiedError;
    /// use chess_lab::core::Position;
    ///
    /// let position = Position::from_string("a1").unwrap();
    /// let error = PositionOccupiedError::new(position);
    /// ```
    ///
    pub fn new(position: Position) -> PositionOccupiedError {
        PositionOccupiedError { position }
    }
}

/// Error indicating that a specific position is empty
///
/// TODO add example
#[derive(Debug, Error)]
#[error("Position {position} is empty")]
pub struct PositionEmptyError {
    /// The position that is empty
    pub position: Position,
}

impl PositionEmptyError {
    /// Creates a new `PositionEmptyError` with the given [position](Position)
    ///
    /// # Arguments
    /// * `position` - The [position](Position) that is empty
    ///
    /// # Example
    /// ```
    /// use chess_lab::errors::PositionEmptyError;
    /// use chess_lab::core::Position;
    ///
    /// let position = Position::from_string("a1").unwrap();
    /// let error = PositionEmptyError::new(position);
    /// ```
    ///
    pub fn new(position: Position) -> PositionEmptyError {
        PositionEmptyError { position }
    }
}

/// Error indicating that a [position](Position) is out of the allowed range
///
/// TODO add example
#[derive(Debug, PartialEq, Error)]
#[error("Position ({col}, {row}) is out of range")]
pub struct PositionOutOfRangeError {
    /// The column index that is out of range
    pub col: u8,
    /// The row index that is out of range
    pub row: u8,
}

impl PositionOutOfRangeError {
    /// Creates a new `PositionOutOfRangeError` with the given column and row
    ///
    /// # Arguments
    /// * `col` - The column index that is out of range
    /// * `row` - The row index that is out of range
    ///
    /// # Example
    /// ```
    /// use chess_lab::errors::PositionOutOfRangeError;
    ///
    /// let col = 8;
    /// let row = 8;
    /// let error = PositionOutOfRangeError::new(col, row);
    /// ```
    ///
    pub fn new(col: u8, row: u8) -> PositionOutOfRangeError {
        PositionOutOfRangeError { col, row }
    }
}

/// Error indicating that a [position](Position) is invalid
///
/// TODO add example
#[derive(Debug, PartialEq, Error)]
#[error("Invalid position: {position_str}")]
pub struct PositionInvalidError {
    /// The string representation of the [position](Position) that is invali
    pub position_str: String,
}

impl PositionInvalidError {
    /// Creates a new `PositionInvalidError` with the given message
    ///
    /// # Arguments
    /// * `message` - The message that describes the error
    ///
    /// # Example
    /// ```
    /// use chess_lab::errors::PositionInvalidError;
    ///
    /// let error = PositionInvalidError::new("Invalid position".to_string());
    /// ```
    ///
    pub fn new(position_str: String) -> PositionInvalidError {
        PositionInvalidError { position_str }
    }
}

/// Error indicating that two [positions](Position) are not aligned
///
/// TODO add example
#[derive(Debug, PartialEq, Error)]
#[error("Positions {position1} and {position2} are not aligned")]
pub struct UnalignedPositionsError {
    /// The first position that is not aligned
    pub position1: Position,
    /// The second position that is not aligned
    pub position2: Position,
}

impl UnalignedPositionsError {
    /// Creates a new `UnalignedPositionsError` with the given positions
    ///
    /// # Arguments
    /// * `position1` - The first position that is not aligned
    /// * `position2` - The second position that is not aligned
    ///
    /// # Example
    /// ```
    /// use chess_lab::errors::UnalignedPositionsError;
    /// use chess_lab::core::Position;
    ///
    /// let position1 = Position::from_string("a1").unwrap();
    /// let position2 = Position::from_string("b3").unwrap();
    /// let error = UnalignedPositionsError::new(position1, position2);
    /// ```
    ///
    pub fn new(position1: Position, position2: Position) -> UnalignedPositionsError {
        UnalignedPositionsError {
            position1,
            position2,
        }
    }
}

/// Error type to handle errors on `piece_between` function
///
/// TODO add example
#[derive(Debug, PartialEq, Error)]
pub enum PositionBetweenError {
    /// Indicates that the position is outside the allowed range
    #[error(transparent)]
    OutOfRange(#[from] PositionOutOfRangeError),
    /// Indicates that two positions are not aligned and they should be
    #[error(transparent)]
    Unaligned(#[from] UnalignedPositionsError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_occupied_error() {
        let position = Position::from_string("a1").unwrap();
        let error = PositionOccupiedError::new(position);

        assert_eq!(error.position, position);
        assert_eq!(format!("{}", error), "Position a1 is already occupied");
    }

    #[test]
    fn test_position_empty_error() {
        let position = Position::from_string("a1").unwrap();
        let error = PositionEmptyError::new(position);

        assert_eq!(error.position, position);
        assert_eq!(format!("{}", error), "Position a1 is empty");
    }

    #[test]
    fn test_position_out_of_range_error() {
        let col = 8;
        let row = 8;
        let error = PositionOutOfRangeError::new(col, row);

        assert_eq!(error.col, col);
        assert_eq!(error.row, row);
        assert_eq!(format!("{}", error), "Position (8, 8) is out of range");
    }

    #[test]
    fn test_position_invalid_error() {
        let position_str = "abc".to_string();
        let error = PositionInvalidError::new(position_str.clone());

        assert_eq!(error.position_str, position_str);
        assert_eq!(format!("{}", error), "Invalid position: abc");
    }

    #[test]
    fn test_unaligned_positions_error() {
        let position1 = Position::from_string("a1").unwrap();
        let position2 = Position::from_string("b3").unwrap();
        let error = UnalignedPositionsError {
            position1,
            position2,
        };

        assert_eq!(error.position1, position1);
        assert_eq!(error.position2, position2);
        assert_eq!(format!("{}", error), "Positions a1 and b3 are not aligned");
    }

    #[test]
    fn test_position_between_error() {
        let col = 8;
        let row = 8;
        let error = PositionBetweenError::OutOfRange(PositionOutOfRangeError::new(col, row));

        assert_eq!(error.to_string(), "Position (8, 8) is out of range");
    }
}
