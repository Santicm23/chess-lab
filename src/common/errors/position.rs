#[derive(Debug, PartialEq)]
pub enum PositionError {
    InvalidPosition,
    PositionOccupied,
    PositionOutOfBounds,
    EmptyPosition,
}
