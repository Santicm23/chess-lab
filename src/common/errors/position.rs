/// Errors that can occur when working with positions
///
/// # Variants
/// * `Invalid` - The position is invalid
/// * `OutOfBounds` - The position is out of bounds
///
#[derive(Debug, PartialEq)]
pub enum PositionError {
    Invalid,
    OutOfBounds,
}
