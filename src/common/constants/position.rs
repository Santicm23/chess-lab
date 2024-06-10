use std::ops;

/// Represents a position on the board.
/// The position is represented by a column and a row.
///
/// # Examples
/// ```
/// use chess_lib::constants::Position;
///
/// let pos = Position::new(0, 0);
///
/// assert_eq!(pos.to_string(), "a1");
///
/// let pos = Position::from_string("a1");
///
/// assert_eq!(pos.col, 0);
/// assert_eq!(pos.row, 0);
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub col: u8,
    pub row: u8,
}

impl Position {
    /// Creates a new position
    ///
    /// # Arguments
    /// * `col`: The column of the position (between 0 and 7)
    /// * `row`: The row of the position (between 0 and 7)
    ///
    /// # Returns
    /// A new position
    ///
    /// # Panics
    /// Panics if the column or row is out of bounds
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::Position;
    ///
    /// let pos = Position::new(0, 0);
    ///
    /// assert_eq!(pos.col, 0);
    /// assert_eq!(pos.row, 0);
    /// ```
    ///
    pub fn new(col: u8, row: u8) -> Position {
        assert!(col < 8 && row < 8, "Position out of bounds");
        Position { col, row }
    }

    /// Creates a new position from a string
    ///
    /// # Arguments
    /// * `s`: The string representation of the position
    ///
    /// # Returns
    /// A new position
    ///
    /// # Panics
    /// Panics if the string is not a valid position
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::Position;
    ///
    /// let pos = Position::from_string("a1");
    ///
    /// assert_eq!(pos.col, 0);
    /// assert_eq!(pos.row, 0);
    /// ```
    ///
    pub fn from_string(s: &str) -> Position {
        assert!(s.len() == 2, "Invalid position string");
        if s.len() != 2 {
            assert!(false, "Invalid position string");
        }
        let col = s.chars().nth(0).unwrap() as u8 - 'a' as u8;
        let row = s.chars().nth(1).unwrap() as u8 - '1' as u8;
        Position::new(col, row)
    }

    /// Converts the position to a string
    ///
    /// # Returns
    /// The string representation of the position
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::Position;
    ///
    /// let pos = Position::new(0, 0);
    ///
    /// assert_eq!(pos.to_bitboard(), 0x0000000000000001);
    /// ```
    ///
    pub fn to_bitboard(&self) -> u64 {
        1 << (self.row * 8 + self.col)
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
    /// use chess_lib::constants::Position;
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
            positions.push(Position::new(pos % 8, pos / 8));
            bitboard &= bitboard - 1;
        }
        positions
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
    /// use chess_lib::constants::Position;
    ///
    /// let pos = Position::new(0, 0);
    /// let new_pos = &pos + (1, 1);
    ///
    /// assert_eq!(new_pos.col, 1);
    /// assert_eq!(new_pos.row, 1);
    /// ```
    ///
    fn add(self, other: (i8, i8)) -> Position {
        Position {
            col: (self.col as i8 + other.0) as u8,
            row: (self.row as i8 + other.1) as u8,
        }
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
    /// use chess_lib::constants::Position;
    ///
    /// let pos1 = Position::new(0, 0);
    /// let pos2 = Position::new(1, 1);
    ///
    /// let offset = &pos1 - &pos2;
    ///
    /// assert_eq!(offset, (-1, -1));
    /// ```
    ///
    fn sub(self, other: &Position) -> (i8, i8) {
        (
            self.col as i8 - other.col as i8,
            self.row as i8 - other.row as i8,
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
    /// use chess_lib::constants::Position;
    ///
    /// let pos = Position::new(1, 1);
    /// let new_pos = &pos - (1, 1);
    ///
    /// assert_eq!(new_pos.col, 0);
    /// assert_eq!(new_pos.row, 0);
    /// ```
    ///
    fn sub(self, other: (i8, i8)) -> Position {
        Position {
            col: (self.col as i8 - other.0) as u8,
            row: (self.row as i8 - other.1) as u8,
        }
    }
}

impl ToString for Position {
    /// Converts the position to a string
    ///
    /// # Returns
    /// The string representation of the position
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::Position;
    ///
    /// let pos = Position::new(0, 0);
    ///
    /// assert_eq!(pos.to_string(), "a1");
    /// ```
    ///
    fn to_string(&self) -> String {
        format!(
            "{}{}",
            ('a' as u8 + self.col) as char,
            ('1' as u8 + self.row) as char
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn test_position() {
        let pos = Position::new(0, 0);
        assert_eq!(pos.to_string(), "a1");
        let pos = Position::new(7, 7);
        assert_eq!(pos.to_string(), "h8");
        let pos = Position::from_string("a1");
        assert_eq!(pos.col, 0);
        assert_eq!(pos.row, 0);
        let pos = Position::from_string("h8");
        assert_eq!(pos.col, 7);
        assert_eq!(pos.row, 7);
    }

    #[test]
    fn test_position_to_bitboard() {
        let pos = Position::new(0, 0);
        assert_eq!(pos.to_bitboard(), 0x0000000000000001);
        let pos = Position::new(7, 7);
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
    }

    #[test]
    fn test_position_sub() {
        let pos1 = Position::new(1, 1);
        let pos2 = Position::new(0, 0);
        let pos3 = &pos1 - &pos2;
        assert_eq!(pos3.0, 1);
        assert_eq!(pos3.1, 1);
    }

    #[test]
    fn test_position_add_tuple() {
        let pos1 = Position::new(0, 0);
        let pos2 = (1, 1);
        let pos3 = &pos1 + pos2;
        assert_eq!(pos3.to_string(), "b2");
    }

    #[test]
    fn test_position_sub_tuple() {
        let pos1 = Position::new(1, 1);
        let pos2 = (1, 1);
        let pos3 = &pos1 - pos2;
        assert_eq!(pos3.to_string(), "a1");
    }
}
