use crate::constants::{
    movements::{diagonal_movement, l_movement, linear_movement, max_movement, movement_direction},
    Color, PieceType, Position,
};

/// Represents a piece on the board with a color and a piece type
///
/// # Examples
/// ```
/// use chess_lib::logic::Piece;
/// use chess_lib::constants::{Color, PieceType};
///
/// let piece = Piece::new(Color::White, PieceType::Pawn);
///
/// assert_eq!(piece.color, Color::White);
/// assert_eq!(piece.piece_type, PieceType::Pawn);
/// assert_eq!(piece.to_string(), "P");
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    /// Creates a new piece with a given color and piece type
    ///
    /// # Arguments
    /// * `color` - The color of the piece
    /// * `piece_type` - The type of the piece
    ///
    /// # Returns
    /// A new piece with the given color and piece type
    ///
    /// # Examples
    /// ```
    /// use chess_lib::logic::Piece;
    /// use chess_lib::constants::{Color, PieceType};
    ///
    /// let piece = Piece::new(Color::White, PieceType::Pawn);
    ///
    /// assert_eq!(piece.color, Color::White);
    /// assert_eq!(piece.piece_type, PieceType::Pawn);
    /// ```
    pub fn new(color: Color, piece_type: PieceType) -> Piece {
        Piece { color, piece_type }
    }

    /// Creates a new piece from a FEN character
    ///
    /// # Arguments
    /// * `char` - The FEN character representing the piece
    ///
    /// # Returns
    /// A new piece with the color and piece type represented by the FEN character
    ///
    /// # Examples
    /// ```
    /// use chess_lib::logic::Piece;
    /// use chess_lib::constants::{Color, PieceType};
    ///
    /// let piece = Piece::from_fen('P');
    ///
    /// assert_eq!(piece.color, Color::White);
    /// assert_eq!(piece.piece_type, PieceType::Pawn);
    /// ```
    ///
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
    /// Converts the piece to a FEN character
    ///
    /// # Returns
    /// The FEN character representing the piece
    ///
    /// # Examples
    /// ```
    /// use chess_lib::logic::Piece;
    /// use chess_lib::constants::{Color, PieceType};
    ///
    /// let piece = Piece::new(Color::White, PieceType::Pawn);
    ///
    /// assert_eq!(piece.to_string(), "P");
    /// ```
    ///
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
///
/// # Arguments
/// * `color` - The color of the pawn
/// * `start_pos` - The starting position of the pawn
/// * `end_pos` - The ending position of the pawn
///
/// # Returns
/// Whether the movement is valid for a pawn
///
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
///
/// # Arguments
/// * `start_pos` - The starting position of the knight
/// * `end_pos` - The ending position of the knight
///
/// # Returns
/// Whether the movement is valid for a knight
///
fn knight_movement(start_pos: &Position, end_pos: &Position) -> bool {
    l_movement(start_pos, end_pos)
}

/// Returns true if the movement is valid for a bishop
///
/// # Arguments
/// * `start_pos` - The starting position of the bishop
/// * `end_pos` - The ending position of the bishop
///
/// # Returns
/// Whether the movement is valid for a bishop
///
fn bishop_movement(start_pos: &Position, end_pos: &Position) -> bool {
    diagonal_movement(start_pos, end_pos)
}

/// Returns true if the movement is valid for a rook
///
/// # Arguments
/// * `start_pos` - The starting position of the rook
/// * `end_pos` - The ending position of the rook
///
/// # Returns
/// Whether the movement is valid for a rook
///
fn rook_movement(start_pos: &Position, end_pos: &Position) -> bool {
    linear_movement(start_pos, end_pos)
}

/// Returns true if the movement is valid for a queen
///
/// # Arguments
/// * `start_pos` - The starting position of the queen
/// * `end_pos` - The ending position of the queen
///
/// # Returns
/// Whether the movement is valid for a queen
///
fn queen_movement(start_pos: &Position, end_pos: &Position) -> bool {
    linear_movement(start_pos, end_pos) || diagonal_movement(start_pos, end_pos)
}

/// Returns true if the movement is valid for a king
///
/// # Arguments
/// * `start_pos` - The starting position of the king
/// * `end_pos` - The ending position of the king
///
/// # Returns
/// Whether the movement is valid for a king
///
fn king_movement(start_pos: &Position, end_pos: &Position) -> bool {
    max_movement(start_pos, end_pos, 1)
}

/// Returns the movement function for a given piece type
///
/// # Arguments
/// * `piece` - The piece to get the movement function for
/// * `start_pos` - The starting position of the piece
/// * `end_pos` - The ending position of the piece
///
/// # Returns
/// Whether the movement is valid for the piece
///
/// # Examples
/// ```
/// use chess_lib::logic::{Piece, piece_movement};
/// use chess_lib::constants::{Color, PieceType, Position};
///
/// let piece = Piece::new(Color::White, PieceType::Pawn);
/// let start_pos = Position::new(0, 1);
/// let end_pos = Position::new(0, 2);
///
/// assert_eq!(piece_movement(&piece, &start_pos, &end_pos), true);
/// ```
///
pub fn piece_movement(piece: &Piece, start_pos: &Position, end_pos: &Position) -> bool {
    match piece.piece_type {
        PieceType::Pawn => pawn_movement(piece.color, start_pos, end_pos),
        PieceType::Knight => knight_movement(start_pos, end_pos),
        PieceType::Bishop => bishop_movement(start_pos, end_pos),
        PieceType::Rook => rook_movement(start_pos, end_pos),
        PieceType::Queen => queen_movement(start_pos, end_pos),
        PieceType::King => king_movement(start_pos, end_pos),
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
            &Position::from_string("e2"),
            &Position::from_string("e3")
        ));
        assert!(pawn_movement(
            Color::White,
            &Position::from_string("e2"),
            &Position::from_string("e4")
        ));
        assert!(pawn_movement(
            Color::White,
            &Position::from_string("e2"),
            &Position::from_string("d3")
        ));
        assert!(pawn_movement(
            Color::White,
            &Position::from_string("e2"),
            &Position::from_string("f3")
        ));
        assert!(pawn_movement(
            Color::Black,
            &Position::from_string("e7"),
            &Position::from_string("e6")
        ));
        assert!(pawn_movement(
            Color::Black,
            &Position::from_string("e7"),
            &Position::from_string("e5")
        ));
        assert!(pawn_movement(
            Color::Black,
            &Position::from_string("e7"),
            &Position::from_string("d6")
        ));
        assert!(pawn_movement(
            Color::Black,
            &Position::from_string("e7"),
            &Position::from_string("f6")
        ));
        assert!(!pawn_movement(
            Color::White,
            &Position::from_string("e4"),
            &Position::from_string("e4")
        ));
        assert!(!pawn_movement(
            Color::White,
            &Position::from_string("e2"),
            &Position::from_string("e5")
        ));
        assert!(!pawn_movement(
            Color::White,
            &Position::from_string("e2"),
            &Position::from_string("d4")
        ));
        assert!(!pawn_movement(
            Color::White,
            &Position::from_string("e2"),
            &Position::from_string("f4")
        ));
        assert!(!pawn_movement(
            Color::Black,
            &Position::from_string("e4"),
            &Position::from_string("e4")
        ));
        assert!(!pawn_movement(
            Color::Black,
            &Position::from_string("e7"),
            &Position::from_string("e4")
        ));
        assert!(!pawn_movement(
            Color::Black,
            &Position::from_string("e7"),
            &Position::from_string("d5")
        ));
        assert!(!pawn_movement(
            Color::Black,
            &Position::from_string("e7"),
            &Position::from_string("f5")
        ));
    }

    #[test]
    fn test_knight_movement() {
        assert!(knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("f6")
        ));
        assert!(knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("g5")
        ));
        assert!(knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("g3")
        ));
        assert!(knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("f2")
        ));
        assert!(knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("d2")
        ));
        assert!(knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("c3")
        ));
        assert!(knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("c5")
        ));
        assert!(knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("d6")
        ));
        assert!(!knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("e4")
        ));
        assert!(!knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("e6")
        ));
        assert!(!knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("g6")
        ));
        assert!(!knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("g4")
        ));
        assert!(!knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("f3")
        ));
        assert!(!knight_movement(
            &Position::from_string("e4"),
            &Position::from_string("d3")
        ));
    }

    #[test]
    fn test_bishop_movement() {
        assert!(bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("f5")
        ));
        assert!(bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("g6")
        ));
        assert!(bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("h7")
        ));
        assert!(bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("f3")
        ));
        assert!(bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("g2")
        ));
        assert!(bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("h1")
        ));
        assert!(bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("d3")
        ));
        assert!(bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("c2")
        ));
        assert!(bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("b1")
        ));
        assert!(bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("d5")
        ));
        assert!(bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("c6")
        ));
        assert!(bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("b7")
        ));
        assert!(!bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("e4")
        ));
        assert!(!bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("e5")
        ));
        assert!(!bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("f6")
        ));
        assert!(!bishop_movement(
            &Position::from_string("e4"),
            &Position::from_string("g7")
        ));
    }

    #[test]
    fn test_rook_movement() {
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("e5")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("e6")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("e7")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("e8")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("e3")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("e2")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("e1")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("f4")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("g4")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("h4")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("d4")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("c4")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("b4")
        ));
        assert!(rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("a4")
        ));
        assert!(!rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("e4")
        ));
        assert!(!rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("f5")
        ));
        assert!(!rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("g6")
        ));
        assert!(!rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("h7")
        ));
        assert!(!rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("d3")
        ));
        assert!(!rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("c2")
        ));
        assert!(!rook_movement(
            &Position::from_string("e4"),
            &Position::from_string("b1")
        ));
    }

    #[test]
    fn test_queen_movement() {
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("f5")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("g6")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("h7")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("f3")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("g2")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("h1")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("d3")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("c2")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("b1")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("d5")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("c6")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("b7")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("e5")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("e6")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("e7")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("e8")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("e3")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("e2")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("e1")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("f4")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("g4")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("h4")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("d4")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("c4")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("b4")
        ));
        assert!(queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("a4")
        ));
        assert!(!queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("e4")
        ));
        assert!(!queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("d7")
        ));
        assert!(!queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("d1")
        ));
        assert!(!queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("f7")
        ));
        assert!(!queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("f1")
        ));
        assert!(!queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("g7")
        ));
        assert!(!queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("g1")
        ));
        assert!(!queen_movement(
            &Position::from_string("e4"),
            &Position::from_string("h8")
        ));
    }

    #[test]
    fn test_king_movement() {
        assert!(king_movement(
            &Position::from_string("e4"),
            &Position::from_string("f5")
        ));
        assert!(king_movement(
            &Position::from_string("e4"),
            &Position::from_string("e5")
        ));
        assert!(king_movement(
            &Position::from_string("e4"),
            &Position::from_string("d5")
        ));
        assert!(king_movement(
            &Position::from_string("e4"),
            &Position::from_string("d4")
        ));
        assert!(king_movement(
            &Position::from_string("e4"),
            &Position::from_string("d3")
        ));
        assert!(king_movement(
            &Position::from_string("e4"),
            &Position::from_string("e3")
        ));
        assert!(king_movement(
            &Position::from_string("e4"),
            &Position::from_string("f3")
        ));
        assert!(king_movement(
            &Position::from_string("e4"),
            &Position::from_string("f4")
        ));
        assert!(!king_movement(
            &Position::from_string("e4"),
            &Position::from_string("e4")
        ));
        assert!(!king_movement(
            &Position::from_string("e4"),
            &Position::from_string("e6")
        ));
        assert!(!king_movement(
            &Position::from_string("e4"),
            &Position::from_string("f6")
        ));
        assert!(!king_movement(
            &Position::from_string("e4"),
            &Position::from_string("g6")
        ));
        assert!(!king_movement(
            &Position::from_string("e4"),
            &Position::from_string("g5")
        ));
        assert!(!king_movement(
            &Position::from_string("e4"),
            &Position::from_string("g4")
        ));
        assert!(!king_movement(
            &Position::from_string("e4"),
            &Position::from_string("g3")
        ));
        assert!(!king_movement(
            &Position::from_string("e4"),
            &Position::from_string("f2")
        ));
        assert!(!king_movement(
            &Position::from_string("e4"),
            &Position::from_string("e2")
        ));
        assert!(!king_movement(
            &Position::from_string("e4"),
            &Position::from_string("d2")
        ));
        assert!(!king_movement(
            &Position::from_string("e4"),
            &Position::from_string("d6")
        ));
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
