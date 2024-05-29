use crate::config::constants::{
    movements::{diagonal_movement, l_movement, linear_movement, max_movement, movement_direction},
    Color, PieceType, Position,
};

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

pub fn knight_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    l_movement(start_pos, end_pos)
}

pub fn bishop_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    diagonal_movement(start_pos, end_pos)
}

pub fn rook_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    linear_movement(start_pos, end_pos)
}

pub fn queen_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    linear_movement(start_pos, end_pos) || diagonal_movement(start_pos, end_pos)
}

pub fn king_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    max_movement(start_pos, end_pos, 1)
}

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
