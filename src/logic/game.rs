use crate::constants::Position;

use super::board::Board;

pub struct Game {
    pub board: Board,
    pub is_white_turn: bool,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
    pub en_passant: Option<Position>,
    pub castling_rights: u8,
    pub start_position: String,
    pub history: Vec<&'static str>,
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            is_white_turn: true,
            halfmove_clock: 0,
            fullmove_number: 1,
            en_passant: None,
            castling_rights: 0b1111,
            start_position: String::from(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ),
            history: Vec::new(),
        }
    }

    pub fn move_piece(&mut self, move_str: &'static str) {}

    pub fn fen(&self) -> String {
        let mut fen = String::new();
        let mut empty_count = 0;

        for rank in (0..8).rev() {
            for file in 0..8 {
                let piece = self.board.get_piece(&Position::new(file, rank));
                if piece.is_some() {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    fen.push_str(&piece.unwrap().to_string());
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
                empty_count = 0;
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        fen
    }
}
