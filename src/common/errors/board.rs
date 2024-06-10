/// Enum for errors that can occur when interacting with the board
///
/// # Variants
/// * `Occupied` - The space is already occupied
/// * `Empty` - The space is empty
///
#[derive(Debug, PartialEq)]
pub enum BoardError {
    Occupied,
    Empty,
}
