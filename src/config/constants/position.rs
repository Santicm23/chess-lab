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
