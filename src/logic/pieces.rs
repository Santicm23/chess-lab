use crate::constants::{
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
            true => Color::White,
            false => Color::Black,
        };

        let piece_type = match char.to_lowercase().to_string().as_str() {
            "p" => PieceType::Pawn,
            "n" => PieceType::Knight,
            "b" => PieceType::Bishop,
            "r" => PieceType::Rook,
            "q" => PieceType::Queen,
            "k" => PieceType::King,
            _ => panic!("Invalid piece type"),
        };

        Piece::new(color, piece_type)
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        let char = match self.piece_type {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
        }
        .to_string();

        match self.color {
            Color::White => char.to_uppercase(),
            Color::Black => char,
        }
    }
}

/// Returns true if the movement is valid for a pawn
fn pawn_movement(color: Color, start_pos: &Position, end_pos: &Position) -> bool {
    let direction;
    let starting_row;

    match color {
        Color::White => {
            direction = 1;
            starting_row = 1;
        }
        Color::Black => {
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
pub fn piece_movement(piece: &Piece, start_pos: &Position, end_pos: &Position) -> bool {
    match piece.piece_type {
        PieceType::Pawn => pawn_movement(piece.color, start_pos, end_pos),
        PieceType::Knight => knight_movement(piece.color, start_pos, end_pos),
        PieceType::Bishop => bishop_movement(piece.color, start_pos, end_pos),
        PieceType::Rook => rook_movement(piece.color, start_pos, end_pos),
        PieceType::Queen => queen_movement(piece.color, start_pos, end_pos),
        PieceType::King => king_movement(piece.color, start_pos, end_pos),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        bishop_movement, king_movement, knight_movement, pawn_movement, queen_movement,
        rook_movement, Piece,
    };
    use crate::constants::{Color, PieceType, Position};

    #[test]
    fn test_pawn_movement() {
        assert!(pawn_movement(
            Color::White,
            &Position::from_string("e2").unwrap(),
            &Position::from_string("e3").unwrap()
        ));
        assert!(pawn_movement(
            Color::White,
            &Position::from_string("e2").unwrap(),
            &Position::from_string("e4").unwrap()
        ));
        assert!(pawn_movement(
            Color::White,
            &Position::from_string("e2").unwrap(),
            &Position::from_string("d3").unwrap()
        ));
        assert!(pawn_movement(
            Color::White,
            &Position::from_string("e2").unwrap(),
            &Position::from_string("f3").unwrap()
        ));
        assert!(pawn_movement(
            Color::Black,
            &Position::from_string("e7").unwrap(),
            &Position::from_string("e6").unwrap()
        ));
        assert!(pawn_movement(
            Color::Black,
            &Position::from_string("e7").unwrap(),
            &Position::from_string("e5").unwrap()
        ));
        assert!(pawn_movement(
            Color::Black,
            &Position::from_string("e7").unwrap(),
            &Position::from_string("d6").unwrap()
        ));
        assert!(pawn_movement(
            Color::Black,
            &Position::from_string("e7").unwrap(),
            &Position::from_string("f6").unwrap()
        ));
        assert!(!pawn_movement(
            Color::White,
            &Position::from_string("e4").unwrap(),
            &Position::from_string("e4").unwrap()
        ));
        assert!(!pawn_movement(
            Color::White,
            &Position::from_string("e2").unwrap(),
            &Position::from_string("e5").unwrap()
        ));
        assert!(!pawn_movement(
            Color::White,
            &Position::from_string("e2").unwrap(),
            &Position::from_string("d4").unwrap()
        ));
        assert!(!pawn_movement(
            Color::White,
            &Position::from_string("e2").unwrap(),
            &Position::from_string("f4").unwrap()
        ));
        assert!(!pawn_movement(
            Color::Black,
            &Position::from_string("e4").unwrap(),
            &Position::from_string("e4").unwrap()
        ));
        assert!(!pawn_movement(
            Color::Black,
            &Position::from_string("e7").unwrap(),
            &Position::from_string("e4").unwrap()
        ));
        assert!(!pawn_movement(
            Color::Black,
            &Position::from_string("e7").unwrap(),
            &Position::from_string("d5").unwrap()
        ));
        assert!(!pawn_movement(
            Color::Black,
            &Position::from_string("e7").unwrap(),
            &Position::from_string("f5").unwrap()
        ));
    }

    #[test]
    fn test_knight_movement() {
        fn asserts(color: Color) {
            assert!(knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f6").unwrap()
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g5").unwrap()
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g3").unwrap()
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f2").unwrap()
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d2").unwrap()
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("c3").unwrap()
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("c5").unwrap()
            ));
            assert!(knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d6").unwrap()
            ));
            assert!(!knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e4").unwrap()
            ));
            assert!(!knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e6").unwrap()
            ));
            assert!(!knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g6").unwrap()
            ));
            assert!(!knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g4").unwrap()
            ));
            assert!(!knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f3").unwrap()
            ));
            assert!(!knight_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d3").unwrap()
            ));
        }
        asserts(Color::White);
        asserts(Color::Black);
    }

    #[test]
    fn test_bishop_movement() {
        fn asserts(color: Color) {
            assert!(bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f5").unwrap()
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g6").unwrap()
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("h7").unwrap()
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f3").unwrap()
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g2").unwrap()
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("h1").unwrap()
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d3").unwrap()
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("c2").unwrap()
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("b1").unwrap()
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d5").unwrap()
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("c6").unwrap()
            ));
            assert!(bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("b7").unwrap()
            ));
            assert!(!bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e4").unwrap()
            ));
            assert!(!bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e5").unwrap()
            ));
            assert!(!bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f6").unwrap()
            ));
            assert!(!bishop_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g7").unwrap()
            ));
        }

        asserts(Color::White);
        asserts(Color::Black);
    }

    #[test]
    fn test_rook_movement() {
        fn asserts(color: Color) {
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e5").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e6").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e7").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e8").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e3").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e2").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e1").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f4").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g4").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("h4").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d4").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("c4").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("b4").unwrap()
            ));
            assert!(rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("a4").unwrap()
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e4").unwrap()
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f5").unwrap()
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g6").unwrap()
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("h7").unwrap()
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d3").unwrap()
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("c2").unwrap()
            ));
            assert!(!rook_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("b1").unwrap()
            ));
        }

        asserts(Color::White);
        asserts(Color::Black);
    }

    #[test]
    fn test_queen_movement() {
        fn asserts(color: Color) {
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f5").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g6").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("h7").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f3").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g2").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("h1").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d3").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("c2").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("b1").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d5").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("c6").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("b7").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e5").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e6").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e7").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e8").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e3").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e2").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e1").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f4").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g4").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("h4").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d4").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("c4").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("b4").unwrap()
            ));
            assert!(queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("a4").unwrap()
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e4").unwrap()
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d7").unwrap()
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d1").unwrap()
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f7").unwrap()
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f1").unwrap()
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g7").unwrap()
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g1").unwrap()
            ));
            assert!(!queen_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("h8").unwrap()
            ));
        }

        asserts(Color::White);
        asserts(Color::Black);
    }

    #[test]
    fn test_king_movement() {
        fn asserts(color: Color) {
            assert!(king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f5").unwrap()
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e5").unwrap()
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d5").unwrap()
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d4").unwrap()
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d3").unwrap()
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e3").unwrap()
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f3").unwrap()
            ));
            assert!(king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f4").unwrap()
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e4").unwrap()
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e6").unwrap()
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f6").unwrap()
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g6").unwrap()
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g5").unwrap()
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g4").unwrap()
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("g3").unwrap()
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("f2").unwrap()
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("e2").unwrap()
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d2").unwrap()
            ));
            assert!(!king_movement(
                color,
                &Position::from_string("e4").unwrap(),
                &Position::from_string("d6").unwrap()
            ));
        }

        asserts(Color::White);
        asserts(Color::Black);
    }

    #[test]
    fn test_from_fen() {
        let wpawn = Piece::from_fen('P');
        assert_eq!(wpawn.color, Color::White);
        assert_eq!(wpawn.piece_type, PieceType::Pawn);

        let bpawn = Piece::from_fen('p');
        assert_eq!(bpawn.color, Color::Black);
        assert_eq!(bpawn.piece_type, PieceType::Pawn);

        let wrook = Piece::from_fen('R');
        assert_eq!(wrook.color, Color::White);
        assert_eq!(wrook.piece_type, PieceType::Rook);

        let brook = Piece::from_fen('r');
        assert_eq!(brook.color, Color::Black);
        assert_eq!(brook.piece_type, PieceType::Rook);

        let wknight = Piece::from_fen('N');
        assert_eq!(wknight.color, Color::White);
        assert_eq!(wknight.piece_type, PieceType::Knight);

        let bknight = Piece::from_fen('n');
        assert_eq!(bknight.color, Color::Black);
        assert_eq!(bknight.piece_type, PieceType::Knight);

        let wbishop = Piece::from_fen('B');
        assert_eq!(wbishop.color, Color::White);
        assert_eq!(wbishop.piece_type, PieceType::Bishop);

        let bbishop = Piece::from_fen('b');
        assert_eq!(bbishop.color, Color::Black);
        assert_eq!(bbishop.piece_type, PieceType::Bishop);

        let wqueen = Piece::from_fen('Q');
        assert_eq!(wqueen.color, Color::White);
        assert_eq!(wqueen.piece_type, PieceType::Queen);

        let bqueen = Piece::from_fen('q');
        assert_eq!(bqueen.color, Color::Black);
        assert_eq!(bqueen.piece_type, PieceType::Queen);

        let wking = Piece::from_fen('K');
        assert_eq!(wking.color, Color::White);
        assert_eq!(wking.piece_type, PieceType::King);

        let bking = Piece::from_fen('k');
        assert_eq!(bking.color, Color::Black);
        assert_eq!(bking.piece_type, PieceType::King);
    }

    #[test]
    fn test_to_fen() {
        let wpawn = Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        };
        assert_eq!(wpawn.to_string(), String::from("P"));

        let bpawn = Piece {
            color: Color::Black,
            piece_type: PieceType::Pawn,
        };
        assert_eq!(bpawn.to_string(), String::from("p"));

        let wrook = Piece {
            color: Color::White,
            piece_type: PieceType::Rook,
        };
        assert_eq!(wrook.to_string(), String::from("R"));

        let brook = Piece {
            color: Color::Black,
            piece_type: PieceType::Rook,
        };
        assert_eq!(brook.to_string(), String::from("r"));

        let wknight = Piece {
            color: Color::White,
            piece_type: PieceType::Knight,
        };
        assert_eq!(wknight.to_string(), String::from("N"));

        let bknight = Piece {
            color: Color::Black,
            piece_type: PieceType::Knight,
        };
        assert_eq!(bknight.to_string(), String::from("n"));

        let wbishop = Piece {
            color: Color::White,
            piece_type: PieceType::Bishop,
        };
        assert_eq!(wbishop.to_string(), String::from("B"));

        let bbishop = Piece {
            color: Color::Black,
            piece_type: PieceType::Bishop,
        };
        assert_eq!(bbishop.to_string(), String::from("b"));

        let wqueen = Piece {
            color: Color::White,
            piece_type: PieceType::Queen,
        };
        assert_eq!(wqueen.to_string(), String::from("Q"));

        let bqueen = Piece {
            color: Color::Black,
            piece_type: PieceType::Queen,
        };
        assert_eq!(bqueen.to_string(), String::from("q"));

        let wking = Piece {
            color: Color::White,
            piece_type: PieceType::King,
        };
        assert_eq!(wking.to_string(), String::from("K"));

        let bking = Piece {
            color: Color::Black,
            piece_type: PieceType::King,
        };
        assert_eq!(bking.to_string(), String::from("k"));
    }
}
