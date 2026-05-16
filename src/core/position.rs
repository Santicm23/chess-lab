use std::{fmt::Display, ops};

use crate::errors::{PositionInvalidError, PositionOutOfRangeError};

/// Represents a position on the board.
/// The position is the coordinates in a chess board, composed by file and rank.
///
/// # Examples
/// ```
/// use chess_lab::core::Position;
///
/// let pos = Position::new(0, 0).unwrap();
///
/// assert_eq!(pos.to_string(), "a1");
///
/// let pos = Position::from_string("a1").unwrap();
///
/// assert_eq!(pos.file, 0);
/// assert_eq!(pos.rank, 0);
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// The file of the position (between 0 and 7)
    pub file: u8,
    /// The rank of the position (between 0 and 7)
    pub rank: u8,
}

impl Position {
    /// Creates a new position
    ///
    /// # Arguments
    /// * `file`: The fileumn of the position (between 0 and 7)
    /// * `rank`: The rank of the position (between 0 and 7)
    ///
    /// # Returns
    /// * `Ok(Position)`: The new position
    /// * `Err(PositionOutOfRangeError)`: If the position is out of range
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Position;
    ///
    /// let pos = Position::new(0, 0).unwrap();
    ///
    /// assert_eq!(pos.file, 0);
    /// assert_eq!(pos.rank, 0);
    /// ```
    ///
    pub fn new(file: u8, rank: u8) -> Result<Position, PositionOutOfRangeError> {
        if file >= 8 || rank >= 8 {
            return Err(PositionOutOfRangeError::new(rank, file));
        }
        Ok(Position { file, rank })
    }

    /// Creates a new position from a string
    ///
    /// # Arguments
    /// * `s`: The string representation of the position
    ///
    /// # Returns
    /// * `Ok(Position)`: The new position
    /// * `Err(PositionInvalidError)`: If the string is invalid
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Position;
    ///
    /// let pos = Position::from_string("a1").unwrap();
    ///
    /// assert_eq!(pos.file, 0);
    /// assert_eq!(pos.rank, 0);
    /// ```
    ///
    pub fn from_string(s: &str) -> Result<Position, PositionInvalidError> {
        if s.len() != 2 {
            return Err(PositionInvalidError::new(s.to_string()));
        }

        let file = s.chars().nth(0).unwrap() as u8 - 'a' as u8;
        let rank = s.chars().nth(1).unwrap() as u8 - '1' as u8;
        Position::new(file, rank).map_err(|_| PositionInvalidError::new(s.to_string()))
    }

    /// Converts the position to a string
    ///
    /// # Returns
    /// The string representation of the position
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Position;
    ///
    /// let pos = Position::new(0, 0).unwrap();
    ///
    /// assert_eq!(pos.to_bitboard(), 0x0000000000000001);
    /// ```
    ///
    pub fn to_bitboard(&self) -> u64 {
        1 << (self.rank * 8 + self.file)
    }

    /// Converts a bitboard to a list of positions
    ///
    /// # Arguments
    /// * `bitboard`: The bitboard to convert
    ///
    /// # Returns
    /// A list of positions
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Position;
    ///
    /// let positions = Position::from_bitboard(0x0000000000000001);
    ///
    /// assert_eq!(positions.len(), 1);
    /// assert_eq!(positions[0].to_string(), "a1");
    /// ```
    ///
    pub fn from_bitboard(bitboard: u64) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut bitboard = bitboard;
        while bitboard != 0 {
            let pos = bitboard.trailing_zeros() as u8;
            positions.push(Position::new(pos % 8, pos / 8).unwrap());
            bitboard &= bitboard - 1;
        }
        positions
    }

    /// Gets the direction between two positions
    ///
    /// # Arguments
    /// * `other`: The other position
    ///
    /// # Returns
    /// The direction between the two positions
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Position;
    ///
    /// let pos1 = Position::new(0, 0).unwrap();
    /// let pos2 = Position::new(1, 1).unwrap();
    ///
    /// let direction = pos1.direction(&pos2);
    ///
    /// assert_eq!(direction, (1, 1));
    /// ```
    pub fn direction(&self, other: &Position) -> (i8, i8) {
        let mut file = other.file as i8 - self.file as i8;
        let mut rank = other.rank as i8 - self.rank as i8;

        file = if file == 0 { 0 } else { file / file.abs() };
        rank = if rank == 0 { 0 } else { rank / rank.abs() };
        (file, rank)
    }
}

impl ops::Add<(i8, i8)> for &Position {
    type Output = Position;

    /// Adds a certain offset to the position
    ///
    /// # Arguments
    /// * `other`: The offset to add
    ///
    /// # Returns
    /// The new position
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Position;
    ///
    /// let pos = Position::new(0, 0).unwrap();
    /// let new_pos = &pos + (1, 1);
    ///
    /// assert_eq!(new_pos.file, 1);
    /// assert_eq!(new_pos.rank, 1);
    /// ```
    ///
    fn add(self, other: (i8, i8)) -> Position {
        Position::new(
            (self.file as i8 + other.0) as u8,
            (self.rank as i8 + other.1) as u8,
        )
        .unwrap()
    }
}

impl ops::Sub<&Position> for &Position {
    type Output = (i8, i8);

    /// Gets the offset between two positions
    ///
    /// # Arguments
    /// * `other`: The other position
    ///
    /// # Returns
    /// The offset between the two positions
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Position;
    ///
    /// let pos1 = Position::new(0, 0).unwrap();
    /// let pos2 = Position::new(1, 1).unwrap();
    ///
    /// let offset = &pos1 - &pos2;
    ///
    /// assert_eq!(offset, (-1, -1));
    /// ```
    ///
    fn sub(self, other: &Position) -> (i8, i8) {
        (
            self.file as i8 - other.file as i8,
            self.rank as i8 - other.rank as i8,
        )
    }
}

impl ops::Sub<(i8, i8)> for &Position {
    type Output = Position;

    /// Subtracts a certain offset from the position
    ///
    /// # Arguments
    /// * `other`: The offset to subtract
    ///
    /// # Returns
    /// The new position
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Position;
    ///
    /// let pos = Position::new(1, 1).unwrap();
    /// let new_pos = &pos - (1, 1);
    ///
    /// assert_eq!(new_pos.file, 0);
    /// assert_eq!(new_pos.rank, 0);
    /// ```
    ///
    fn sub(self, other: (i8, i8)) -> Position {
        Position {
            file: (self.file as i8 - other.0) as u8,
            rank: (self.rank as i8 - other.1) as u8,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            ('a' as u8 + self.file) as char,
            ('1' as u8 + self.rank) as char
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let pos = Position::new(0, 0).unwrap();
        assert_eq!(pos.to_string(), "a1");
        let pos = Position::new(7, 7).unwrap();
        assert_eq!(pos.to_string(), "h8");
        let pos = Position::from_string("a1").unwrap();
        assert_eq!(pos.file, 0);
        assert_eq!(pos.rank, 0);
        let pos = Position::from_string("h8").unwrap();
        assert_eq!(pos.file, 7);
        assert_eq!(pos.rank, 7);
    }

    #[test]
    fn test_position_invalid() {
        let pos = Position::new(8, 0);
        assert!(pos.is_err());
        let pos = Position::new(0, 8);
        assert!(pos.is_err());
        let pos = Position::from_string("i1");
        assert!(pos.is_err());
        let pos = Position::from_string("a9");
        assert!(pos.is_err());
        let pos = Position::from_string("a");
        assert!(pos.is_err());
        let pos = Position::from_string("a12");
        assert!(pos.is_err());
    }

    #[test]
    fn test_position_to_bitboard() {
        let pos = Position::new(0, 0).unwrap();
        assert_eq!(pos.to_bitboard(), 0x0000000000000001);
        let pos = Position::new(7, 7).unwrap();
        assert_eq!(pos.to_bitboard(), 0x8000000000000000);
    }

    #[test]
    fn test_position_from_bitboard() {
        let positions = Position::from_bitboard(0x0000000000000001);
        let pos = positions.first().unwrap();
        assert_eq!(pos.to_string(), "a1");
        assert_eq!(positions.len(), 1);
        let positions = Position::from_bitboard(0x8000000000000000);
        let pos = positions.first().unwrap();
        assert_eq!(pos.to_string(), "h8");
        assert_eq!(positions.len(), 1);
        let positions = Position::from_bitboard(0xFFFFFFFFFFFFFFFF);
        assert_eq!(positions.len(), 64);
    }

    #[test]
    fn test_position_sub() {
        let pos1 = Position::new(1, 1).unwrap();
        let pos2 = Position::new(0, 0).unwrap();
        let pos3 = &pos1 - &pos2;
        assert_eq!(pos3.0, 1);
        assert_eq!(pos3.1, 1);
    }

    #[test]
    fn test_position_add_tuple() {
        let pos1 = Position::new(0, 0).unwrap();
        let pos2 = (1, 1);
        let pos3 = &pos1 + pos2;
        assert_eq!(pos3.to_string(), "b2");
    }

    #[test]
    fn test_position_sub_tuple() {
        let pos1 = Position::new(1, 1).unwrap();
        let pos2 = (1, 1);
        let pos3 = &pos1 - pos2;
        assert_eq!(pos3.to_string(), "a1");
    }

    #[test]
    fn test_direction() {
        let pos1 = Position::new(0, 0).unwrap();
        let pos2 = Position::new(1, 1).unwrap();
        let dir = pos1.direction(&pos2);
        assert_eq!(dir, (1, 1));
    }
}
