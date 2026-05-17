use std::{fmt::Display, ops};

use crate::errors::{SquareInvalidError, SquareOutOfRangeError};

/// Represents a square of the board.
/// The square is the coordinates in a chess board, composed by file and rank.
///
/// # Examples
/// ```
/// use chess_lab::core::Square;
///
/// let sqr = Square::new(0, 0).unwrap();
///
/// assert_eq!(sqr.to_string(), "a1");
///
/// let sqr = Square::from_string("a1").unwrap();
///
/// assert_eq!(sqr.file, 0);
/// assert_eq!(sqr.rank, 0);
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    /// The file of the square (between 0 and 7)
    pub file: u8,
    /// The rank of the square (between 0 and 7)
    pub rank: u8,
}

impl Square {
    /// Creates a new square from file and rank
    ///
    /// # Arguments
    /// * `file`: The fileumn of the square (between 0 and 7)
    /// * `rank`: The rank of the square (between 0 and 7)
    ///
    /// # Returns
    /// * `Ok(Square)`: The new square
    /// * `Err(SquareOutOfRangeError)`: If the square is out of range
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Square;
    ///
    /// let sqr = Square::new(0, 0).unwrap();
    ///
    /// assert_eq!(sqr.file, 0);
    /// assert_eq!(sqr.rank, 0);
    /// ```
    ///
    pub fn new(file: u8, rank: u8) -> Result<Square, SquareOutOfRangeError> {
        if file >= 8 || rank >= 8 {
            return Err(SquareOutOfRangeError::new(rank, file));
        }
        Ok(Square { file, rank })
    }

    /// Creates a new sqaure from a string
    ///
    /// # Arguments
    /// * `s`: The string representation of the square (e.g. "a1", "h8")
    ///
    /// # Returns
    /// * `Ok(Square)`: The new square
    /// * `Err(SquareInvalidError)`: If the string is invalid
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Square;
    ///
    /// let sqr = Square::from_string("a1").unwrap();
    ///
    /// assert_eq!(sqr.file, 0);
    /// assert_eq!(sqr.rank, 0);
    /// ```
    ///
    pub fn from_string(s: &str) -> Result<Square, SquareInvalidError> {
        if s.len() != 2 {
            return Err(SquareInvalidError::new(s.to_string()));
        }

        let file = s.chars().nth(0).unwrap() as u8 - 'a' as u8;
        let rank = s.chars().nth(1).unwrap() as u8 - '1' as u8;
        Square::new(file, rank).map_err(|_| SquareInvalidError::new(s.to_string()))
    }

    /// Converts the square to a bitboard
    ///
    /// # Returns
    /// The bitboard representation of the square
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Square;
    ///
    /// let sqr = Square::new(0, 0).unwrap();
    ///
    /// assert_eq!(sqr.to_bitboard(), 0x0000000000000001);
    /// ```
    ///
    pub fn to_bitboard(&self) -> u64 {
        1 << (self.rank * 8 + self.file)
    }

    /// Converts a bitboard to a square list
    ///
    /// # Arguments
    /// * `bitboard`: The bitboard to convert
    ///
    /// # Returns
    /// A square list
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Square;
    ///
    /// let sqrs = Square::from_bitboard(0x0000000000000001);
    ///
    /// assert_eq!(sqrs.len(), 1);
    /// assert_eq!(sqrs[0].to_string(), "a1");
    /// ```
    ///
    pub fn from_bitboard(bitboard: u64) -> Vec<Square> {
        let mut sqrs = Vec::new();
        let mut bitboard = bitboard;
        while bitboard != 0 {
            let sqr = bitboard.trailing_zeros() as u8;
            sqrs.push(Square::new(sqr % 8, sqr / 8).unwrap());
            bitboard &= bitboard - 1;
        }
        sqrs
    }

    /// Gets the direction between two squares
    ///
    /// # Arguments
    /// * `other`: The other square
    ///
    /// # Returns
    /// The direction between the two squares
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Square;
    ///
    /// let sqr1 = Square::new(0, 0).unwrap();
    /// let sqr2 = Square::new(1, 1).unwrap();
    ///
    /// let direction = sqr1.direction(&sqr2);
    ///
    /// assert_eq!(direction, (1, 1));
    /// ```
    pub fn direction(&self, other: &Square) -> (i8, i8) {
        let mut file = other.file as i8 - self.file as i8;
        let mut rank = other.rank as i8 - self.rank as i8;

        file = if file == 0 { 0 } else { file / file.abs() };
        rank = if rank == 0 { 0 } else { rank / rank.abs() };
        (file, rank)
    }
}

impl ops::Add<(i8, i8)> for &Square {
    type Output = Square;

    /// Adds a certain offset to the square
    ///
    /// # Arguments
    /// * `other`: The offset to add
    ///
    /// # Returns
    /// The new square
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Square;
    ///
    /// let sqr = Square::new(0, 0).unwrap();
    /// let new_sqr = &sqr + (1, 1);
    ///
    /// assert_eq!(new_sqr.file, 1);
    /// assert_eq!(new_sqr.rank, 1);
    /// ```
    ///
    fn add(self, other: (i8, i8)) -> Square {
        Square::new(
            (self.file as i8 + other.0) as u8,
            (self.rank as i8 + other.1) as u8,
        )
        .unwrap()
    }
}

impl ops::Sub<&Square> for &Square {
    type Output = (i8, i8);

    /// Gets the offset between two squares
    ///
    /// # Arguments
    /// * `other`: The other square
    ///
    /// # Returns
    /// The offset between the two squares
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Square;
    ///
    /// let sqr1 = Square::new(0, 0).unwrap();
    /// let sqr2 = Square::new(1, 1).unwrap();
    ///
    /// let offset = &sqr1 - &sqr2;
    ///
    /// assert_eq!(offset, (-1, -1));
    /// ```
    ///
    fn sub(self, other: &Square) -> (i8, i8) {
        (
            self.file as i8 - other.file as i8,
            self.rank as i8 - other.rank as i8,
        )
    }
}

impl ops::Sub<(i8, i8)> for &Square {
    type Output = Square;

    /// Subtracts a certain offset from the square
    ///
    /// # Arguments
    /// * `other`: The offset to subtract
    ///
    /// # Returns
    /// The new square
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Square;
    ///
    /// let sqr = Square::new(1, 1).unwrap();
    /// let new_sqr = &sqr - (1, 1);
    ///
    /// assert_eq!(new_sqr.file, 0);
    /// assert_eq!(new_sqr.rank, 0);
    /// ```
    ///
    fn sub(self, other: (i8, i8)) -> Square {
        Square {
            file: (self.file as i8 - other.0) as u8,
            rank: (self.rank as i8 - other.1) as u8,
        }
    }
}

impl Display for Square {
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
    fn test_square() {
        let sqr = Square::new(0, 0).unwrap();
        assert_eq!(sqr.to_string(), "a1");
        let sqr = Square::new(7, 7).unwrap();
        assert_eq!(sqr.to_string(), "h8");
        let sqr = Square::from_string("a1").unwrap();
        assert_eq!(sqr.file, 0);
        assert_eq!(sqr.rank, 0);
        let sqr = Square::from_string("h8").unwrap();
        assert_eq!(sqr.file, 7);
        assert_eq!(sqr.rank, 7);
    }

    #[test]
    fn test_square_invalid() {
        let sqr = Square::new(8, 0);
        assert!(sqr.is_err());
        let sqr = Square::new(0, 8);
        assert!(sqr.is_err());
        let sqr = Square::from_string("i1");
        assert!(sqr.is_err());
        let sqr = Square::from_string("a9");
        assert!(sqr.is_err());
        let sqr = Square::from_string("a");
        assert!(sqr.is_err());
        let sqr = Square::from_string("a12");
        assert!(sqr.is_err());
    }

    #[test]
    fn test_square_to_bitboard() {
        let sqr = Square::new(0, 0).unwrap();
        assert_eq!(sqr.to_bitboard(), 0x0000000000000001);
        let sqr = Square::new(7, 7).unwrap();
        assert_eq!(sqr.to_bitboard(), 0x8000000000000000);
    }

    #[test]
    fn test_square_from_bitboard() {
        let sqrs = Square::from_bitboard(0x0000000000000001);
        let sqr = sqrs.first().unwrap();
        assert_eq!(sqr.to_string(), "a1");
        assert_eq!(sqrs.len(), 1);
        let sqrs = Square::from_bitboard(0x8000000000000000);
        let sqr = sqrs.first().unwrap();
        assert_eq!(sqr.to_string(), "h8");
        assert_eq!(sqrs.len(), 1);
        let sqrs = Square::from_bitboard(0xFFFFFFFFFFFFFFFF);
        assert_eq!(sqrs.len(), 64);
    }

    #[test]
    fn test_square_sub() {
        let sqr1 = Square::new(1, 1).unwrap();
        let sqr2 = Square::new(0, 0).unwrap();
        let sqr3 = &sqr1 - &sqr2;
        assert_eq!(sqr3.0, 1);
        assert_eq!(sqr3.1, 1);
    }

    #[test]
    fn test_square_add_tuple() {
        let sqr1 = Square::new(0, 0).unwrap();
        let sqr2 = (1, 1);
        let sqr3 = &sqr1 + sqr2;
        assert_eq!(sqr3.to_string(), "b2");
    }

    #[test]
    fn test_square_sub_tuple() {
        let sqr1 = Square::new(1, 1).unwrap();
        let sqr2 = (1, 1);
        let sqr3 = &sqr1 - sqr2;
        assert_eq!(sqr3.to_string(), "a1");
    }

    #[test]
    fn test_direction() {
        let sqr1 = Square::new(0, 0).unwrap();
        let sqr2 = Square::new(1, 1).unwrap();
        let dir = sqr1.direction(&sqr2);
        assert_eq!(dir, (1, 1));
    }
}
