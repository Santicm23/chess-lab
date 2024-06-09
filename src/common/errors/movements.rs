#[derive(Debug, PartialEq)]
pub enum MoveError {
    Invalid,
    Illegal,
    Ambiguous,
}
