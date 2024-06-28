use super::position::Position;

/// Check if the movement is diagonal
///
/// # Arguments
/// * `start_pos`: The starting position
/// * `end_pos`: The ending position
///
/// # Returns
/// * Whether the movement is diagonal
///
pub fn diagonal_movement(start_pos: &Position, end_pos: &Position) -> bool {
    let diff = end_pos - start_pos;
    diff.0.abs() == diff.1.abs() && diff != (0, 0)
}

/// Check if the movement is linear
///
/// # Arguments
/// * `start_pos`: The starting position
/// * `end_pos`: The ending position
///
/// # Returns
/// * Whether the movement is linear
///
pub fn linear_movement(start_pos: &Position, end_pos: &Position) -> bool {
    let diff = end_pos - start_pos;
    (diff.0 == 0 || diff.1 == 0) && diff != (0, 0)
}

/// Check if the movement is an L
///
/// # Arguments
/// * `start_pos`: The starting position
/// * `end_pos`: The ending position
///
/// # Returns
/// * Whether the movement is an L
///
pub fn l_movement(start_pos: &Position, end_pos: &Position) -> bool {
    let diff = end_pos - start_pos;
    (diff.0.abs() == 2 && diff.1.abs() == 1) || (diff.0.abs() == 1 && diff.1.abs() == 2)
}

/// Check if the movement is of a certain length
///
/// # Arguments
/// * `start_pos`: The starting position
/// * `end_pos`: The ending position
///
/// # Returns
/// * Whether the movement is of a certain length
///
pub fn max_movement(start_pos: &Position, end_pos: &Position, max: i8) -> bool {
    let diff = end_pos - start_pos;
    diff.0.abs() <= max && diff.1.abs() <= max && diff != (0, 0)
}

/// Check if the movement is in a certain direction
///
/// # Arguments
/// * `start_pos`: The starting position
/// * `end_pos`: The ending position
///
/// # Returns
/// * Whether the movement is in a certain direction
///
pub fn movement_direction(start_pos: &Position, end_pos: &Position, direction: (i8, i8)) -> bool {
    let diff = end_pos - start_pos;
    if direction.0 == 0 {
        diff.1 * direction.1 > 0 && diff.0 == 0
    } else if direction.1 == 0 {
        diff.0 * direction.0 > 0 && diff.1 == 0
    } else {
        diff.0 * direction.0 == diff.1 * direction.1
    }
}

#[cfg(test)]
mod tests {
    use super::{
        diagonal_movement, l_movement, linear_movement, max_movement, movement_direction, Position,
    };

    #[test]
    fn test_diagonal_movement() {
        let start_pos = Position::new(0, 0);
        let end_pos = Position::new(3, 3);
        assert!(diagonal_movement(&start_pos, &end_pos));
    }

    #[test]
    fn test_linear_movement() {
        let start_pos = Position::new(0, 0);
        let end_pos = Position::new(0, 3);
        assert!(linear_movement(&start_pos, &end_pos));
    }

    #[test]
    fn test_l_movement() {
        let start_pos = Position::new(0, 0);
        let end_pos = Position::new(1, 2);
        assert!(l_movement(&start_pos, &end_pos));
    }

    #[test]
    fn test_max_movement() {
        let start_pos = Position::new(0, 0);
        let end_pos = Position::new(1, 1);
        assert!(max_movement(&start_pos, &end_pos, 1));
    }

    #[test]
    fn test_movement_direction() {
        let start_pos = Position::new(0, 2);
        let end_pos = Position::new(2, 0);
        assert!(movement_direction(&start_pos, &end_pos, (1, -1)));
    }
}
