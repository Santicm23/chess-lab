use crate::constants::Position;

/// Error indicating that a specific position is already occupied.
///
/// This error is returned when an attempt is made to place an item
/// in a position that is already taken.
///
#[derive(Debug)]
pub struct PositionOccupiedError {
    pub position: Position,
}

impl PositionOccupiedError {
    /// Creates a new `PositionOccupiedError` with the given position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position that is occupied.
    ///
    /// # Example
    ///
    /// ```
    /// let error = PositionOccupiedError::new(position);
    /// ```
    ///
    pub fn new(position: Position) -> PositionOccupiedError {
        PositionOccupiedError { position }
    }
}

/// Error indicating that a specific position is empty.
///
/// This error is returned when an operation expects a position to be occupied
/// but finds it empty instead.
///
#[derive(Debug)]
pub struct PositionEmptyError {
    pub position: Position,
}

impl PositionEmptyError {
    /// Creates a new `PositionEmptyError` with the given position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position that is empty.
    ///
    /// # Example
    ///
    /// ```
    /// let error = PositionEmptyError::new(position);
    /// ```
    ///
    pub fn new(position: Position) -> PositionEmptyError {
        PositionEmptyError { position }
    }
}

/// Error indicating that a position is out of the allowed range.
///
/// This error is returned when a specified position exceeds the defined
/// column (`col`) or row (`row`) boundaries.
///
#[derive(Debug, PartialEq)]
pub struct PositionOutOfRangeError {
    pub col: u8,
    pub row: u8,
}

impl PositionOutOfRangeError {
    /// Creates a new `PositionOutOfRangeError` with the given column and row.
    ///
    /// # Arguments
    ///
    /// * `col` - The column index that is out of range.
    /// * `row` - The row index that is out of range.
    ///
    /// # Example
    ///
    /// ```
    /// let error = PositionOutOfRangeError::new(col, row);
    /// ```
    ///
    pub fn new(col: u8, row: u8) -> PositionOutOfRangeError {
        PositionOutOfRangeError { col, row }
    }
}

/// Enum representing invalid position errors.
///
/// This enum encapsulates different kinds of invalid position errors, including:
/// - `OutOfRange`: Indicates that the position is outside the allowed range.
/// - `Invalid`: Indicates that the position is fundamentally invalid for other reasons.
///
#[derive(Debug, PartialEq)]
pub enum PositionInvalidError {
    OutOfRange(PositionOutOfRangeError),
    Invalid(String),
}
