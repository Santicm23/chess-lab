use std::fmt::Display;

use crate::{
    core::{piece_movement, Color, Piece, PieceType, Position},
    errors::{
        FenError, PositionBetweenError, PositionEmptyError, PositionOccupiedError,
        UnalignedPositionsError,
    },
    parsing::fen::parse_simple_fen,
    utils::movements::{diagonal_movement, linear_movement},
};

/// A struct that represents a chess board
/// The board is represented by bitboards of each piece (color and type)
///
#[derive(Debug, Clone)]
pub struct Board {
    /// Bitboard of white pawns
    wpawns: u64,
    /// Bitboard of black pawns
    bpawns: u64,
    /// Bitboard of white knights
    wknights: u64,
    /// Bitboard of black knights
    bknights: u64,
    /// Bitboard of white bishops
    wbishops: u64,
    /// Bitboard of black bishops
    bbishops: u64,
    /// Bitboard of white rooks
    wrooks: u64,
    /// Bitboard of black rooks
    brooks: u64,
    /// Bitboard of white queens
    wqueens: u64,
    /// Bitboard of black queens
    bqueens: u64,
    /// Bitboard of white kings
    wkings: u64,
    /// Bitboard of black kings
    bkings: u64,
}

impl Default for Board {
    /// Creates a new board with the default starting position
    ///
    /// # Returns
    /// A new board with the default starting position
    ///
    fn default() -> Board {
        Board {
            wpawns: 0x000000000000FF00,
            bpawns: 0x00FF000000000000,
            wknights: 0x0000000000000042,
            bknights: 0x4200000000000000,
            wbishops: 0x0000000000000024,
            bbishops: 0x2400000000000000,
            wrooks: 0x0000000000000081,
            brooks: 0x8100000000000000,
            wqueens: 0x0000000000000008,
            bqueens: 0x0800000000000000,
            wkings: 0x0000000000000010,
            bkings: 0x1000000000000000,
        }
    }
}

impl Board {
    /// Creates a new board from a FEN string
    ///
    /// # Arguments
    /// * `fen`: A FEN string representing the board
    ///
    /// # Returns
    /// * `Ok(Board)`: A new board with the position represented by the FEN string
    /// * `Err(FenError)`: If the FEN string is invalid
    ///
    pub fn new(fen: &str) -> Result<Board, FenError> {
        Board::from_fen(fen)
    }

    /// Creates a new empty board
    ///
    /// # Returns
    /// A new empty board
    ///
    pub fn empty() -> Board {
        Board {
            wpawns: 0,
            bpawns: 0,
            wknights: 0,
            bknights: 0,
            wbishops: 0,
            bbishops: 0,
            wrooks: 0,
            brooks: 0,
            wqueens: 0,
            bqueens: 0,
            wkings: 0,
            bkings: 0,
        }
    }

    /// Creates a new board from a FEN string
    ///
    /// # Arguments
    /// * `fen`: A FEN string representing the board
    ///
    /// # Returns
    /// A `Result<Board, FenError>` object
    /// * `Ok(Board)`: A new board with the position represented by the FEN string
    /// * `Err(FenError)`: If the FEN string is invalid
    ///
    pub fn from_fen(fen: &str) -> Result<Board, FenError> {
        parse_simple_fen(fen)
    }

    /// Checks if a position is occupied by a piece
    ///
    /// # Arguments
    /// * `pos`: The position to check
    ///
    /// # Returns
    /// Whether the position is occupied by a piece
    ///
    pub fn is_ocupied(&self, pos: &Position) -> bool {
        let bit = pos.to_bitboard();
        (self.wpawns
            | self.bpawns
            | self.wknights
            | self.bknights
            | self.wbishops
            | self.bbishops
            | self.wrooks
            | self.brooks
            | self.wqueens
            | self.bqueens
            | self.wkings
            | self.bkings)
            & bit
            != 0
    }

    /// Gets the piece at a position
    ///
    /// # Arguments
    /// * `pos`: The position to get the piece
    ///
    /// # Returns
    /// The piece at the position or None if the position is empty
    ///
    pub fn get_piece(&self, pos: &Position) -> Option<Piece> {
        let bit = pos.to_bitboard();

        let piece_data = [
            (self.wpawns, Color::White, PieceType::Pawn),
            (self.bpawns, Color::Black, PieceType::Pawn),
            (self.wknights, Color::White, PieceType::Knight),
            (self.bknights, Color::Black, PieceType::Knight),
            (self.wbishops, Color::White, PieceType::Bishop),
            (self.bbishops, Color::Black, PieceType::Bishop),
            (self.wrooks, Color::White, PieceType::Rook),
            (self.brooks, Color::Black, PieceType::Rook),
            (self.wqueens, Color::White, PieceType::Queen),
            (self.bqueens, Color::Black, PieceType::Queen),
            (self.wkings, Color::White, PieceType::King),
            (self.bkings, Color::Black, PieceType::King),
        ];

        match piece_data.iter().find(|(board, _, _)| *board & bit != 0) {
            Some((_, color, piece_type)) => Some(Piece::new(*color, *piece_type)),
            None => None,
        }
    }

    /// Sets a piece at a position
    ///
    /// # Arguments
    /// * `piece`: The piece to set
    /// * `pos`: The position to set the piece
    ///
    /// # Returns
    /// * `Ok(())`: If the piece was set successfully
    /// * `Err(PositionOccupiedError)`: If the position is already occupied
    ///
    pub fn set_piece(&mut self, piece: Piece, pos: &Position) -> Result<(), PositionOccupiedError> {
        if self.is_ocupied(pos) {
            return Err(PositionOccupiedError::new(pos.clone()));
        }
        let bit = pos.to_bitboard();
        match piece.piece_type {
            PieceType::Pawn => match piece.color {
                Color::White => self.wpawns |= bit,
                Color::Black => self.bpawns |= bit,
            },
            PieceType::Knight => match piece.color {
                Color::White => self.wknights |= bit,
                Color::Black => self.bknights |= bit,
            },
            PieceType::Bishop => match piece.color {
                Color::White => self.wbishops |= bit,
                Color::Black => self.bbishops |= bit,
            },
            PieceType::Rook => match piece.color {
                Color::White => self.wrooks |= bit,
                Color::Black => self.brooks |= bit,
            },
            PieceType::Queen => match piece.color {
                Color::White => self.wqueens |= bit,
                Color::Black => self.bqueens |= bit,
            },
            PieceType::King => match piece.color {
                Color::White => self.wkings |= bit,
                Color::Black => self.bkings |= bit,
            },
        }
        Ok(())
    }

    /// Deletes a piece at a position
    ///
    /// # Arguments
    /// * `pos`: The position to delete the piece
    ///
    /// # Returns
    /// * `Ok(Piece)`: The piece that was deleted
    /// * `Err(PositionEmptyError)`: If the position is empty
    ///
    pub fn delete_piece(&mut self, pos: &Position) -> Result<Piece, PositionEmptyError> {
        let piece = match self.get_piece(&pos) {
            Some(piece) => piece,
            None => return Err(PositionEmptyError::new(pos.clone())),
        };

        let bit = pos.to_bitboard();
        match piece.piece_type {
            PieceType::Pawn => match piece.color {
                Color::White => self.wpawns &= !bit,
                Color::Black => self.bpawns &= !bit,
            },
            PieceType::Knight => match piece.color {
                Color::White => self.wknights &= !bit,
                Color::Black => self.bknights &= !bit,
            },
            PieceType::Bishop => match piece.color {
                Color::White => self.wbishops &= !bit,
                Color::Black => self.bbishops &= !bit,
            },
            PieceType::Rook => match piece.color {
                Color::White => self.wrooks &= !bit,
                Color::Black => self.brooks &= !bit,
            },
            PieceType::Queen => match piece.color {
                Color::White => self.wqueens &= !bit,
                Color::Black => self.bqueens &= !bit,
            },
            PieceType::King => match piece.color {
                Color::White => self.wkings &= !bit,
                Color::Black => self.bkings &= !bit,
            },
        }
        Ok(piece)
    }

    /// Finds all pieces of a certain type and color
    ///
    /// # Arguments
    /// * `piece_type`: The type of the piece
    /// * `color`: The color of the piece
    ///
    /// # Returns
    /// A vector of positions of the pieces
    ///
    pub fn find(&self, piece_type: PieceType, color: Color) -> Vec<Position> {
        let bitboard;
        match piece_type {
            PieceType::Pawn => match color {
                Color::White => bitboard = self.wpawns,
                Color::Black => bitboard = self.bpawns,
            },
            PieceType::Knight => match color {
                Color::White => bitboard = self.wknights,
                Color::Black => bitboard = self.bknights,
            },
            PieceType::Bishop => match color {
                Color::White => bitboard = self.wbishops,
                Color::Black => bitboard = self.bbishops,
            },
            PieceType::Rook => match color {
                Color::White => bitboard = self.wrooks,
                Color::Black => bitboard = self.brooks,
            },
            PieceType::Queen => match color {
                Color::White => bitboard = self.wqueens,
                Color::Black => bitboard = self.bqueens,
            },
            PieceType::King => match color {
                Color::White => bitboard = self.wkings,
                Color::Black => bitboard = self.bkings,
            },
        }
        Position::from_bitboard(bitboard)
    }

    /// Finds all pieces of a certain color
    ///
    /// # Arguments
    /// * `color`: The color of the pieces
    ///
    /// # Returns
    /// A vector of positions of the pieces
    ///
    pub fn find_all(&self, color: Color) -> Vec<Position> {
        let mut pieces = Vec::new();
        pieces.append(&mut self.find(PieceType::Pawn, color));
        pieces.append(&mut self.find(PieceType::Knight, color));
        pieces.append(&mut self.find(PieceType::Bishop, color));
        pieces.append(&mut self.find(PieceType::Rook, color));
        pieces.append(&mut self.find(PieceType::Queen, color));
        pieces.append(&mut self.find(PieceType::King, color));
        pieces
    }

    /// Moves a piece from one position to another
    ///
    /// # Arguments
    /// * `from`: The position to move the piece from
    /// * `to`: The position to move the piece to
    ///
    /// # Returns
    /// * `Ok(())`: If the piece was moved successfully
    /// * `Err(PositionEmptyError)`: If the start position is empty
    ///
    pub fn move_piece(&mut self, from: &Position, to: &Position) -> Result<(), PositionEmptyError> {
        let piece = self.delete_piece(from)?;

        self.delete_piece(to).ok();

        self.set_piece(piece, to).unwrap(); // safe unwrap
        Ok(())
    }

    /// Checks if a position is attacked by a certain color
    ///
    /// # Arguments
    /// * `pos`: The position to check
    /// * `color`: The color of the attacking pieces
    ///
    /// # Returns
    /// Whether the position is attacked or not
    ///
    pub fn is_attacked(&self, pos: Position, color: Color) -> bool {
        let pieces = self.find_all(color);
        for piece in pieces {
            if self.can_capture(&piece, &pos).unwrap() {
                // safe unwrap (depends on find_all method)
                return true;
            }
        }
        false
    }

    /// Checks if a piece can capture another piece
    /// The function does not check if the move is legal
    /// It only checks if the piece can capture the other piece
    ///
    /// # Arguments
    /// * `start_pos`: The position of the piece to move
    /// * `end_pos`: The position of the piece to capture
    ///
    /// # Returns
    /// * `Ok(bool)`: Whether the piece can capture the other piece
    /// * `Err(PositionEmptyError)`: If the start position is empty
    ///
    pub fn can_capture(
        &self,
        start_pos: &Position,
        end_pos: &Position,
    ) -> Result<bool, PositionEmptyError> {
        let piece = self
            .get_piece(start_pos)
            .ok_or(PositionEmptyError::new(start_pos.clone()))?;

        match self.get_piece(end_pos) {
            None => (),
            Some(captured_piece) => {
                if captured_piece.color == piece.color {
                    return Ok(false);
                }
            }
        }

        if piece_movement(&piece, start_pos, end_pos) {
            if piece.piece_type == PieceType::Pawn && start_pos.col == end_pos.col {
                return Ok(false);
            }
            return match piece.piece_type {
                PieceType::Pawn => Ok(diagonal_movement(start_pos, end_pos)),
                PieceType::Knight | PieceType::King => Ok(true),
                PieceType::Bishop | PieceType::Rook | PieceType::Queen => {
                    Ok(!self.piece_between(start_pos, end_pos).unwrap()) // safe unwrap (depends on piece_movement)
                }
            };
        }
        Ok(false)
    }

    /// Checks if there is a piece between two positions
    ///
    /// # Arguments
    /// * `from`: The starting position
    /// * `to`: The ending position
    ///
    /// # Returns
    /// * `Ok(bool)`: Whether there is a piece between the two positions
    /// * `Err(PositionBetweenError)`: If the positions are not aligned
    ///
    pub fn piece_between(
        &self,
        from: &Position,
        to: &Position,
    ) -> Result<bool, PositionBetweenError> {
        if !linear_movement(from, to) && !diagonal_movement(from, to) {
            return Err(PositionBetweenError::from(UnalignedPositionsError::new(
                from.clone(),
                to.clone(),
            )));
        }

        let direction = from.direction(to);
        let mut pos = from.to_owned();

        for _ in 0..7 {
            pos = &pos + direction;
            if pos == *to {
                break;
            }
            if self.is_ocupied(&pos) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for row in (0..8).rev() {
            let mut empty = 0;
            for col in 0..8 {
                let pos = Position::new(col, row).unwrap(); // safe unwrap
                let piece = self.get_piece(&pos);
                match piece {
                    None => {
                        empty += 1;
                    }
                    Some(piece) => {
                        if empty > 0 {
                            board.push_str(&empty.to_string());
                            empty = 0;
                        }
                        board.push_str(&piece.to_string());
                    }
                }
            }
            if empty > 0 {
                board.push_str(&empty.to_string());
            }
            if row > 0 {
                board.push('/');
            }
        }
        write!(f, "{}", board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Color, Piece, PieceType, Position};

    #[test]
    fn test_default() {
        let board = Board::default();
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
    }

    #[test]
    fn from_fen() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
    }

    #[test]
    fn test_to_fen() {
        let board = Board::default();
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
    }

    #[test]
    fn test_set_piece() {
        let mut board = Board::default();
        let pos = Position::new(4, 2).unwrap();
        let piece = Piece::new(Color::White, PieceType::Pawn);
        board.set_piece(piece, &pos).unwrap();
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/pppppppp/8/8/8/4P3/PPPPPPPP/RNBQKBNR"
        );
    }

    #[test]
    fn test_set_piece_occupied() {
        let mut board = Board::default();
        let pos = Position::new(0, 1).unwrap();
        let piece = Piece::new(Color::White, PieceType::Pawn);
        let result = board.set_piece(piece, &pos);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_piece() {
        let mut board = Board::default();
        let pos = Position::new(0, 0).unwrap();
        board.delete_piece(&pos).unwrap();
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/1NBQKBNR"
        );
    }

    #[test]
    fn test_get_piece() {
        let board = Board::default();
        let pos = Position::new(0, 0).unwrap();
        let piece = board.get_piece(&pos).unwrap();
        assert_eq!(piece.to_string(), "R");
    }

    #[test]
    fn test_is_ocupied() {
        let board = Board::default();
        let pos = Position::new(0, 0).unwrap();
        assert!(board.is_ocupied(&pos));

        let pos = Position::new(0, 2).unwrap();
        assert!(!board.is_ocupied(&pos));
    }

    #[test]
    fn test_find() {
        let board = Board::default();
        let pieces = board.find(PieceType::Pawn, Color::White);
        assert_eq!(pieces.len(), 8);
    }

    #[test]
    fn test_find_all() {
        let board = Board::default();
        let pieces = board.find_all(Color::White);
        assert_eq!(pieces.len(), 16);
    }

    #[test]
    fn test_move_piece() {
        let mut board = Board::default();
        let from = Position::new(4, 1).unwrap();
        let to = Position::new(4, 3).unwrap();
        board.move_piece(&from, &to).unwrap();
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR"
        );
    }

    #[test]
    fn test_is_attacked() {
        let board = Board::default();

        let pos = Position::from_string("e3").unwrap();
        assert!(board.is_attacked(pos, Color::White));

        let pos = Position::from_string("e4").unwrap();
        assert!(!board.is_attacked(pos, Color::White));
    }

    #[test]
    fn test_piece_between() {
        let board = Board::default();
        let from = Position::new(0, 0).unwrap();
        let to = Position::new(0, 6).unwrap();
        assert!(board.piece_between(&from, &to).unwrap());
    }
}
