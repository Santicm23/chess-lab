/// Errors that can occur when trying to move a piece
///
/// # Variants
/// * `Invalid` - The move is invalid
/// * `Illegal` - The move is illegal
/// * `Ambiguous` - The move is ambiguous
///
#[derive(Debug, PartialEq)]
pub enum MoveError {
    Invalid,
    Illegal,
    Ambiguous,
}
