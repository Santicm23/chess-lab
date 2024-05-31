use crate::config::constants::{
    movements::{diagonal_movement, l_movement, linear_movement, max_movement, movement_direction},
    Color, PieceType, Position,
};

pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Piece {
        Piece { color, piece_type }
    }

    pub fn from_fen(char: char) -> Piece {
        let color = match char.is_uppercase() {
            true => Color::WHITE,
            false => Color::BLACK,
        };

        let piece_type = match char.to_lowercase().to_string().as_str() {
            "p" => PieceType::PAWN,
            "n" => PieceType::KNIGHT,
            "b" => PieceType::BISHOP,
            "r" => PieceType::ROOK,
            "q" => PieceType::QUEEN,
            "k" => PieceType::KING,
            _ => panic!("Invalid piece type"),
        };

        Piece::new(color, piece_type)
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        let char = match self.piece_type {
            PieceType::PAWN => "p",
            PieceType::KNIGHT => "n",
            PieceType::BISHOP => "b",
            PieceType::ROOK => "r",
            PieceType::QUEEN => "q",
            PieceType::KING => "k",
        }
        .to_string();

        match self.color {
            Color::WHITE => char.to_uppercase(),
            Color::BLACK => char,
        }
    }
}

/// Returns true if the movement is valid for a pawn
fn pawn_movement(color: Color, start_pos: &Position, end_pos: &Position) -> bool {
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
fn knight_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    l_movement(start_pos, end_pos)
}

/// Returns true if the movement is valid for a bishop
fn bishop_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    diagonal_movement(start_pos, end_pos)
}

/// Returns true if the movement is valid for a rook
fn rook_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    linear_movement(start_pos, end_pos)
}

/// Returns true if the movement is valid for a queen
fn queen_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    linear_movement(start_pos, end_pos) || diagonal_movement(start_pos, end_pos)
}

/// Returns true if the movement is valid for a king
fn king_movement(_: Color, start_pos: &Position, end_pos: &Position) -> bool {
    max_movement(start_pos, end_pos, 1)
}

/// Returns the movement function for a given piece type
pub fn piece_movement(piece: Piece, start_pos: &Position, end_pos: &Position) -> bool {
    match piece.piece_type {
        PieceType::PAWN => pawn_movement(piece.color, start_pos, end_pos),
        PieceType::KNIGHT => knight_movement(piece.color, start_pos, end_pos),
        PieceType::BISHOP => bishop_movement(piece.color, start_pos, end_pos),
        PieceType::ROOK => rook_movement(piece.color, start_pos, end_pos),
        PieceType::QUEEN => queen_movement(piece.color, start_pos, end_pos),
        PieceType::KING => king_movement(piece.color, start_pos, end_pos),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        bishop_movement, king_movement, knight_movement, pawn_movement, queen_movement,
        rook_movement, Piece,
    };
    use crate::config::constants::{Color, PieceType, Position};

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

    #[test]
    fn test_from_fen() {
        let wpawn = Piece::from_fen('P');
        assert_eq!(wpawn.color, Color::WHITE);
        assert_eq!(wpawn.piece_type, PieceType::PAWN);

        let bpawn = Piece::from_fen('p');
        assert_eq!(bpawn.color, Color::BLACK);
        assert_eq!(bpawn.piece_type, PieceType::PAWN);

        let wrook = Piece::from_fen('R');
        assert_eq!(wrook.color, Color::WHITE);
        assert_eq!(wrook.piece_type, PieceType::ROOK);

        let brook = Piece::from_fen('r');
        assert_eq!(brook.color, Color::BLACK);
        assert_eq!(brook.piece_type, PieceType::ROOK);

        let wknight = Piece::from_fen('N');
        assert_eq!(wknight.color, Color::WHITE);
        assert_eq!(wknight.piece_type, PieceType::KNIGHT);

        let bknight = Piece::from_fen('n');
        assert_eq!(bknight.color, Color::BLACK);
        assert_eq!(bknight.piece_type, PieceType::KNIGHT);

        let wbishop = Piece::from_fen('B');
        assert_eq!(wbishop.color, Color::WHITE);
        assert_eq!(wbishop.piece_type, PieceType::BISHOP);

        let bbishop = Piece::from_fen('b');
        assert_eq!(bbishop.color, Color::BLACK);
        assert_eq!(bbishop.piece_type, PieceType::BISHOP);

        let wqueen = Piece::from_fen('Q');
        assert_eq!(wqueen.color, Color::WHITE);
        assert_eq!(wqueen.piece_type, PieceType::QUEEN);

        let bqueen = Piece::from_fen('q');
        assert_eq!(bqueen.color, Color::BLACK);
        assert_eq!(bqueen.piece_type, PieceType::QUEEN);

        let wking = Piece::from_fen('K');
        assert_eq!(wking.color, Color::WHITE);
        assert_eq!(wking.piece_type, PieceType::KING);

        let bking = Piece::from_fen('k');
        assert_eq!(bking.color, Color::BLACK);
        assert_eq!(bking.piece_type, PieceType::KING);
    }
}
