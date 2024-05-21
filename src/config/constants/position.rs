use std::ops;

pub struct Position {
    col: u8,
    row: u8,
}

impl Position {
    pub fn new(col: u8, row: u8) -> Position {
        assert!(col < 8 && row < 8, "Invalid position");
        Position { col, row }
    }

    pub fn from_string(s: &str) -> Position {
        let col = s.chars().nth(0).unwrap() as u8 - 'a' as u8;
        let row = s.chars().nth(1).unwrap() as u8 - '1' as u8;
        Position { col, row }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            col: self.col + other.col,
            row: self.row + other.row,
        }
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            col: self.col - other.col,
            row: self.row - other.row,
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
