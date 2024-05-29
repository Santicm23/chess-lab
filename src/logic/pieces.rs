use crate::config::constants::{
    movements::{diagonal_movement, l_movement, linear_movement, max_movement, movement_direction},
    Color, PieceType, Position,
};

/// Returns true if the movement is valid for a pawn
pub fn pawn_movement(color: Color, start_pos: &Position, end_pos: &Position) -> bool {
    let direction;
    let starting_row;

    match color {
        Color::WHITE => {
            direction = 1;
            starting_row = 1;
        }
        Color::BLACK => {
            direction = -1;
            starting_row = 6;
        }
    }

    if max_movement(start_pos, end_pos, 1) {
        movement_direction(start_pos, end_pos, (0, direction))
            || movement_direction(start_pos, end_pos, (1, direction))
            || movement_direction(start_pos, end_pos, (-1, direction))
    } else if max_movement(start_pos, end_pos, 2) {
        movement_direction(start_pos, end_pos, (0, direction)) && start_pos.row == starting_row
    } else {
        false
    }
}

/// Returns true if the movement is valid for a knight
pub fn knight_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    l_movement(start_pos, end_pos)
}

/// Returns true if the movement is valid for a bishop
pub fn bishop_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    diagonal_movement(start_pos, end_pos)
}

/// Returns true if the movement is valid for a rook
pub fn rook_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    linear_movement(start_pos, end_pos)
}

/// Returns true if the movement is valid for a queen
pub fn queen_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    linear_movement(start_pos, end_pos) || diagonal_movement(start_pos, end_pos)
}

/// Returns true if the movement is valid for a king
pub fn king_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    max_movement(start_pos, end_pos, 1)
}

/// Returns the movement function for a given piece type
pub fn get_piece_movement(piece_type: PieceType) -> fn(Color, &Position, &Position) -> bool {
    match piece_type {
        PieceType::PAWN => pawn_movement,
        PieceType::KNIGHT => knight_movement,
        PieceType::BISHOP => bishop_movement,
        PieceType::ROOK => rook_movement,
        PieceType::QUEEN => queen_movement,
        PieceType::KING => king_movement,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        bishop_movement, king_movement, knight_movement, pawn_movement, queen_movement,
        rook_movement,
    };
    use crate::config::constants::{Color, Position};

    #[test]
    fn test_pawn_movement() {
        assert!(pawn_movement(
            Color::WHITE,
            &Position::from_string("e2"),
            &Position::from_string("e3")
        ));
        assert!(pawn_movement(
            Color::WHITE,
            &Position::from_string("e2"),
            &Position::from_string("e4")
        ));
        assert!(pawn_movement(
            Color::WHITE,
            &Position::from_string("e2"),
            &Position::from_string("d3")
        ));
        assert!(pawn_movement(
            Color::WHITE,
            &Position::from_string("e2"),
            &Position::from_string("f3")
        ));
        assert!(pawn_movement(
            Color::BLACK,
            &Position::from_string("e7"),
            &Position::from_string("e6")
        ));
        assert!(pawn_movement(
            Color::BLACK,
            &Position::from_string("e7"),
            &Position::from_string("e5")
        ));
        assert!(pawn_movement(
            Color::BLACK,
            &Position::from_string("e7"),
            &Position::from_string("d6")
        ));
        assert!(pawn_movement(
            Color::BLACK,
            &Position::from_string("e7"),
            &Position::from_string("f6")
        ));
        assert!(!pawn_movement(
            Color::WHITE,
            &Position::from_string("e4"),
            &Position::from_string("e4")
        ));
        assert!(!pawn_movement(
            Color::WHITE,
            &Position::from_string("e2"),
            &Position::from_string("e5")
        ));
        assert!(!pawn_movement(
            Color::WHITE,
            &Position::from_string("e2"),
            &Position::from_string("d4")
        ));
        assert!(!pawn_movement(
            Color::WHITE,
            &Position::from_string("e2"),
            &Position::from_string("f4")
        ));
        assert!(!pawn_movement(
            Color::BLACK,
            &Position::from_string("e4"),
            &Position::from_string("e4")
        ));
        assert!(!pawn_movement(
            Color::BLACK,
            &Position::from_string("e7"),
            &Position::from_string("e4")
        ));
        assert!(!pawn_movement(
            Color::BLACK,
            &Position::from_string("e7"),
            &Position::from_string("d5")
        ));
        assert!(!pawn_movement(
            Color::BLACK,
            &Position::from_string("e7"),
            &Position::from_string("f5")
        ));
    }

    #[test]
    fn test_knight_movement() {
        fn asserts(color: Color) {
            assert!(knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f6")
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g5")
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g3")
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f2")
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d2")
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("c3")
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("c5")
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d6")
            ));
            assert!(!knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e4")
            ));
            assert!(!knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e6")
            ));
            assert!(!knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g6")
            ));
            assert!(!knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g4")
            ));
            assert!(!knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f3")
            ));
            assert!(!knight_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d3")
            ));
        }
        asserts(Color::WHITE);
        asserts(Color::BLACK);
    }

    #[test]
    fn test_bishop_movement() {
        fn asserts(color: Color) {
            assert!(bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f5")
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g6")
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("h7")
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f3")
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g2")
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("h1")
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d3")
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("c2")
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("b1")
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d5")
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("c6")
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("b7")
            ));
            assert!(!bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e4")
            ));
            assert!(!bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e5")
            ));
            assert!(!bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f6")
            ));
            assert!(!bishop_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g7")
            ));
        }

        asserts(Color::WHITE);
        asserts(Color::BLACK);
    }

    #[test]
    fn test_rook_movement() {
        fn asserts(color: Color) {
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e5")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e6")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e7")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e8")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e3")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e2")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e1")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f4")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g4")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("h4")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d4")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("c4")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("b4")
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("a4")
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e4")
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f5")
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g6")
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("h7")
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d3")
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("c2")
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("b1")
            ));
        }

        asserts(Color::WHITE);
        asserts(Color::BLACK);
    }

    #[test]
    fn test_queen_movement() {
        fn asserts(color: Color) {
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f5")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g6")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("h7")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f3")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g2")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("h1")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d3")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("c2")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("b1")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d5")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("c6")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("b7")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e5")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e6")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e7")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e8")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e3")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e2")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e1")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f4")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g4")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("h4")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d4")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("c4")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("b4")
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("a4")
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e4")
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d7")
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d1")
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f7")
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f1")
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g7")
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g1")
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("h8")
            ));
        }

        asserts(Color::WHITE);
        asserts(Color::BLACK);
    }

    #[test]
    fn test_king_movement() {
        fn asserts(color: Color) {
            assert!(king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f5")
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e5")
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d5")
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d4")
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d3")
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e3")
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f3")
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f4")
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e4")
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e6")
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f6")
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g6")
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g5")
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g4")
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("g3")
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("f2")
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("e2")
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d2")
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4"),
                &Position::from_string("d6")
            ));
        }

        asserts(Color::WHITE);
        asserts(Color::BLACK);
    }
}
