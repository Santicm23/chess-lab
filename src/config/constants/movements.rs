use super::position::Position;

pub fn diagonal_movement(start_pos: Position, end_pos: Position) -> bool {
    let diff = end_pos - start_pos;
    diff.0.abs() == diff.1.abs() && diff != (0, 0)
}

pub fn linear_movement(start_pos: Position, end_pos: Position) -> bool {
    let diff = end_pos - start_pos;
    (diff.0 == 0 || diff.1 == 0) && diff != (0, 0)
}

pub fn max_movement(start_pos: Position, end_pos: Position, max: i8) -> bool {
    let diff = end_pos - start_pos;
    diff.0.abs() <= max && diff.1.abs() <= max && diff != (0, 0)
}

pub fn direction(start_pos: Position, end_pos: Position, direction: (i8, i8)) -> bool {
    let diff = end_pos - start_pos;
    diff.0 % direction.0 == 0 && diff.1 % direction.1 == 0 && diff != (0, 0)
}
