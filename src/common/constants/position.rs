use std::ops;

/// Represents a position on the board.
pub struct Position {
    pub col: u8,
    pub row: u8,
}

impl Position {
    pub fn new(col: u8, row: u8) -> Position {
        assert!(col < 8 && row < 8);
        Position { col, row }
    }

    pub fn from_string(s: &str) -> Position {
        let col = s.chars().nth(0).unwrap() as u8 - 'a' as u8;
        let row = s.chars().nth(1).unwrap() as u8 - '1' as u8;
        Position { col, row }
    }

    pub fn to_bitboard(&self) -> u64 {
        1 << (self.row * 8 + self.col)
    }
}

impl ops::Add<&Position> for &Position {
    type Output = (i8, i8);

    fn add(self, other: &Position) -> (i8, i8) {
        (
            self.col as i8 + other.col as i8,
            self.row as i8 + other.row as i8,
        )
    }
}

impl ops::Add<(i8, i8)> for &Position {
    type Output = Position;

    fn add(self, other: (i8, i8)) -> Position {
        Position {
            col: (self.col as i8 + other.0) as u8,
            row: (self.row as i8 + other.1) as u8,
        }
    }
}

impl ops::Sub<&Position> for &Position {
    type Output = (i8, i8);

    fn sub(self, other: &Position) -> (i8, i8) {
        (
            self.col as i8 - other.col as i8,
            self.row as i8 - other.row as i8,
        )
    }
}

impl ops::Sub<(i8, i8)> for &Position {
    type Output = Position;

    fn sub(self, other: (i8, i8)) -> Position {
        Position {
            col: (self.col as i8 - other.0) as u8,
            row: (self.row as i8 - other.1) as u8,
        }
    }
}

impl ToString for Position {
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
    fn test_position_add() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(1, 1);
        let pos3 = &pos1 + &pos2;
        assert_eq!(pos3.0, 1);
        assert_eq!(pos3.1, 1);
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
