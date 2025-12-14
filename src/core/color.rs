/// Represents the color of a chess piece
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    /// The white color
    White,
    /// The black color
    Black,
}

impl Color {
    /// Gets the opposite [color](Color)
    ///
    /// # Returns
    /// The [color](Color) opposite to the current one
    ///
    /// # Example
    /// ```
    /// use chess_lab::core::Color;
    ///
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
