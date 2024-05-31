use crate::constants::{Color, PieceType, Position};

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
            wqueens: 0x0000000000000010,
            bqueens: 0x1000000000000000,
            wkings: 0x0000000000000008,
            bkings: 0x0800000000000000,
        }
    }
}

impl Board {
    pub fn new(fen: &str) -> Result<Board, &'static str> {
        Result::Ok(Board::from_fen(fen)?)
    }

    pub fn from_fen(fen: &str) -> Result<Board, &'static str> {
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

                board.set_piece(piece, &Position::new(col, row))?;

                col += 1;
            }

            row -= 1;
        }
        Result::Ok(board)
    }

    pub fn is_ocupied(&self, pos: &Position) -> bool {
        let bit = pos.to_bitboard();
        self.wpawns
            & self.bpawns
            & self.wknights
            & self.bknights
            & self.wbishops
            & self.bbishops
            & self.wrooks
            & self.brooks
            & self.wqueens
            & self.bqueens
            & self.wkings
            & self.bkings
            & bit
            != 0
    }

    pub fn get_piece(&self, pos: &Position) -> Option<Piece> {
        let bit = pos.to_bitboard();
        if self.wpawns & bit != 0 {
            return Option::Some(Piece::new(Color::WHITE, PieceType::PAWN));
        }
        if self.bpawns & bit != 0 {
            return Option::Some(Piece::new(Color::BLACK, PieceType::PAWN));
        }
        if self.wknights & bit != 0 {
            return Option::Some(Piece::new(Color::WHITE, PieceType::KNIGHT));
        }
        if self.bknights & bit != 0 {
            return Option::Some(Piece::new(Color::BLACK, PieceType::KNIGHT));
        }
        if self.wbishops & bit != 0 {
            return Option::Some(Piece::new(Color::WHITE, PieceType::BISHOP));
        }
        if self.bbishops & bit != 0 {
            return Option::Some(Piece::new(Color::BLACK, PieceType::BISHOP));
        }
        if self.wrooks & bit != 0 {
            return Option::Some(Piece::new(Color::WHITE, PieceType::ROOK));
        }
        if self.brooks & bit != 0 {
            return Option::Some(Piece::new(Color::BLACK, PieceType::ROOK));
        }
        if self.wqueens & bit != 0 {
            return Option::Some(Piece::new(Color::WHITE, PieceType::QUEEN));
        }
        if self.bqueens & bit != 0 {
            return Option::Some(Piece::new(Color::BLACK, PieceType::QUEEN));
        }
        if self.wkings & bit != 0 {
            return Option::Some(Piece::new(Color::WHITE, PieceType::KING));
        }
        if self.bkings & bit != 0 {
            return Option::Some(Piece::new(Color::BLACK, PieceType::KING));
        }
        Option::None
    }

    pub fn set_piece(&mut self, piece: Piece, pos: &Position) -> Result<(), &'static str> {
        if self.is_ocupied(pos) {
            return Result::Err("Position already occupied");
        }
        let bit = pos.to_bitboard();
        match piece.piece_type {
            PieceType::PAWN => match piece.color {
                Color::WHITE => self.wpawns |= bit,
                Color::BLACK => self.bpawns |= bit,
            },
            PieceType::KNIGHT => match piece.color {
                Color::WHITE => self.wknights |= bit,
                Color::BLACK => self.bknights |= bit,
            },
            PieceType::BISHOP => match piece.color {
                Color::WHITE => self.wbishops |= bit,
                Color::BLACK => self.bbishops |= bit,
            },
            PieceType::ROOK => match piece.color {
                Color::WHITE => self.wrooks |= bit,
                Color::BLACK => self.brooks |= bit,
            },
            PieceType::QUEEN => match piece.color {
                Color::WHITE => self.wqueens |= bit,
                Color::BLACK => self.bqueens |= bit,
            },
            PieceType::KING => match piece.color {
                Color::WHITE => self.wkings |= bit,
                Color::BLACK => self.bkings |= bit,
            },
        }
        Result::Ok(())
    }

    pub fn delete_piece(&mut self, pos: &Position) -> Result<(), &'static str> {
        let piece = self.get_piece(&pos);
        if piece.is_none() {
            return Result::Err("No piece at position");
        }
        let piece = piece.unwrap();
        let bit = pos.to_bitboard();
        match piece.piece_type {
            PieceType::PAWN => match piece.color {
                Color::WHITE => self.wpawns &= !bit,
                Color::BLACK => self.bpawns &= !bit,
            },
            PieceType::KNIGHT => match piece.color {
                Color::WHITE => self.wknights &= !bit,
                Color::BLACK => self.bknights &= !bit,
            },
            PieceType::BISHOP => match piece.color {
                Color::WHITE => self.wbishops &= !bit,
                Color::BLACK => self.bbishops &= !bit,
            },
            PieceType::ROOK => match piece.color {
                Color::WHITE => self.wrooks &= !bit,
                Color::BLACK => self.brooks &= !bit,
            },
            PieceType::QUEEN => match piece.color {
                Color::WHITE => self.wqueens &= !bit,
                Color::BLACK => self.bqueens &= !bit,
            },
            PieceType::KING => match piece.color {
                Color::WHITE => self.wkings &= !bit,
                Color::BLACK => self.bkings &= !bit,
            },
        }
        Result::Ok(())
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut board = String::new();
        for row in (0..8).rev() {
            let mut empty = 0;
            for col in 0..8 {
                let pos = Position::new(col, row);
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
