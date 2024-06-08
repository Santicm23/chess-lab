use regex::Regex;

use crate::{
    common::errors::{board::BoardError, fen::FenError},
    constants::{Color, PieceType, Position},
};

use super::pieces::Piece;

pub struct Board {
    wpawns: u64,
    bpawns: u64,
    wknights: u64,
    bknights: u64,
    wbishops: u64,
    bbishops: u64,
    wrooks: u64,
    brooks: u64,
    wqueens: u64,
    bqueens: u64,
    wkings: u64,
    bkings: u64,
}

impl Default for Board {
    fn default() -> Self {
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
    pub fn new(fen: &str) -> Result<Board, FenError> {
        Ok(Board::from_fen(fen)?)
    }

    pub fn from_fen(fen: &str) -> Result<Board, FenError> {
        let re = Regex::new(r"^([1-8PpNnBbRrQqKk]{1,8}/){7}[1-8PpNnBbRrQqKk]{1,8}$").unwrap();
        if !re.is_match(fen) {
            return Err(FenError::Invalid);
        }

        let mut board = Board::default();
        let ranks = fen.split('/').collect::<Vec<&str>>();

        let mut row = 7;
        for rank in ranks {
            let mut col = 0;
            for c in rank.chars() {
                if c.is_digit(10) {
                    col += c.to_digit(10).unwrap() as u8;
                    continue;
                }

                let piece = Piece::from_fen(c);

                board
                    .set_piece(
                        piece,
                        match &Position::new(col, row) {
                            Ok(pos) => pos,
                            Err(_) => return Err(FenError::Invalid),
                        },
                    )
                    .unwrap();

                col += 1;
            }

            row -= 1;
        }
        Ok(board)
    }

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

    pub fn get_piece(&self, pos: &Position) -> Option<Piece> {
        let bit = pos.to_bitboard();
        if self.wpawns & bit != 0 {
            return Some(Piece::new(Color::White, PieceType::Pawn));
        }
        if self.bpawns & bit != 0 {
            return Some(Piece::new(Color::Black, PieceType::Pawn));
        }
        if self.wknights & bit != 0 {
            return Some(Piece::new(Color::White, PieceType::Knight));
        }
        if self.bknights & bit != 0 {
            return Some(Piece::new(Color::Black, PieceType::Knight));
        }
        if self.wbishops & bit != 0 {
            return Some(Piece::new(Color::White, PieceType::Bishop));
        }
        if self.bbishops & bit != 0 {
            return Some(Piece::new(Color::Black, PieceType::Bishop));
        }
        if self.wrooks & bit != 0 {
            return Some(Piece::new(Color::White, PieceType::Rook));
        }
        if self.brooks & bit != 0 {
            return Some(Piece::new(Color::Black, PieceType::Rook));
        }
        if self.wqueens & bit != 0 {
            return Some(Piece::new(Color::White, PieceType::Queen));
        }
        if self.bqueens & bit != 0 {
            return Some(Piece::new(Color::Black, PieceType::Queen));
        }
        if self.wkings & bit != 0 {
            return Some(Piece::new(Color::White, PieceType::King));
        }
        if self.bkings & bit != 0 {
            return Some(Piece::new(Color::Black, PieceType::King));
        }
        None
    }

    pub fn set_piece(&mut self, piece: Piece, pos: &Position) -> Result<(), BoardError> {
        if self.is_ocupied(pos) {
            return Err(BoardError::Occupied);
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

    pub fn delete_piece(&mut self, pos: &Position) -> Result<Piece, BoardError> {
        let piece = self.get_piece(&pos);
        if piece.is_none() {
            return Err(BoardError::Empty);
        }
        let piece = piece.unwrap();
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
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut board = String::new();
        for row in (0..8).rev() {
            let mut empty = 0;
            for col in 0..8 {
                let pos = Position::new(col, row).unwrap();
                let piece = self.get_piece(&pos);
                if piece.is_none() {
                    empty += 1;
                    continue;
                }
                if empty > 0 {
                    board.push_str(&empty.to_string());
                    empty = 0;
                }
                let piece = piece.unwrap();
                board.push_str(&piece.to_string());
            }
            if empty > 0 {
                board.push_str(&empty.to_string());
            }
            if row > 0 {
                board.push('/');
            }
        }
        board
    }
}

#[cfg(test)]
mod tests {

    use super::Board;
    use crate::constants::{Color, PieceType, Position};
    use crate::logic::pieces::Piece;

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
}
