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
        Game {
            board: Board::default(),
            is_white_turn: true,
            castling_rights: 0b1111,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            start_position: String::from(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ),
            history: Vec::new(),
        }
    }
}

impl Game {
    pub fn new(fen: &str) -> Game {
        Game::from_fen(fen)
    }

    pub fn from_fen(fen: &str) -> Game {
        let mut game = Game::default();
        game.start_position = String::from(fen);

        let parts = fen.split(' ').collect::<Vec<&str>>();
        game.board = Board::from_fen(parts[0]).unwrap();
        game.is_white_turn = parts[1] == "w";
        game.castling_rights = parts[2].chars().fold(0, |acc, c| match c {
            'K' => acc | 0b1000,
            'Q' => acc | 0b0100,
            'k' => acc | 0b0010,
            'q' => acc | 0b0001,
            _ => acc,
        });
        game.en_passant = if parts[3] == "-" {
            None
        } else {
            Some(Position::from_string(parts[3]))
        };
        game.halfmove_clock = parts[4].parse::<u8>().unwrap();
        game.fullmove_number = parts[5].parse::<u8>().unwrap();
        game
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
