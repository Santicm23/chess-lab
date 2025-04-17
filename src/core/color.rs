/// Represents the color of a chess piece
///
/// # Variants
/// * `White`: The white color
/// * `Black`: The black color
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Gets the opposite color
    ///
    /// # Returns
    /// The color opposite to the current one
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
