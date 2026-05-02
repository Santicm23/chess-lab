/// Represents the color of a chess [Piece](crate::core::Piece)
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    /// The white color
    White,
    /// The black color
    Black,
}

impl Color {
    /// Gets the opposite [Color]
    ///
    /// # Returns
    /// The [Color] opposite to the current one
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::Color;
    /// assert_eq!(Color::White.opposite(), Color::Black);
    /// assert_eq!(Color::Black.opposite(), Color::White);
    /// ```
    ///
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opposite_color() {
        assert_eq!(Color::White.opposite(), Color::Black);
        assert_eq!(Color::Black.opposite(), Color::White);
    }
}
