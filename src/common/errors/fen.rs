/// Errors that can occur when parsing a FEN string
///
/// # Variants
/// * `Invalid` - The FEN string is invalid
///
#[derive(Debug, PartialEq)]
pub enum FenError {
    Invalid,
}
