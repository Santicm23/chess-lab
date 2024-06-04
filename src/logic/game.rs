use regex::Regex;

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
    pub fn new(fen: &str) -> Result<Game, &'static str> {
        Ok(Game::from_fen(fen)?)
    }

    pub fn from_fen(fen: &str) -> Result<Game, &'static str> {
        let re = Regex::new(r"^([1-8PpNnBbRrQqKk]{1,8}/){7}[1-8PpNnBbRrQqKk]{1,8} [wb] (-|[KQkq]{1,4}) (-|[a-h][1-8]) \d+ ([1-9]\d*)$").unwrap();
        if !re.is_match(fen) {
            return Err("Invalid FEN");
        }

        let mut game = Game::default();
        game.start_position = String::from(fen);

        let parts = fen.split(' ').collect::<Vec<&str>>();
        game.board = Board::from_fen(parts[0])?;
        game.is_white_turn = parts[1] == "w";
        game.castling_rights = parts[2].chars().fold(0, |acc, c| match c {
            'K' => acc | 0b1000,
            'Q' => acc | 0b0100,
            'k' => acc | 0b0010,
            'q' => acc | 0b0001,
            _ => 0,
        });
        game.en_passant = if parts[3] == "-" {
            None
        } else {
            Some(Position::from_string(parts[3]))
        };
        game.halfmove_clock = parts[4].parse::<u8>().unwrap();
        game.fullmove_number = parts[5].parse::<u8>().unwrap();
        Ok(game)
    }

    pub fn move_piece(&mut self, move_str: &'static str) -> Result<(), &'static str> {
        let re =
            Regex::new(r"^([NBRQK]?[a-h]?[1-8]?x?[a-h][1-8](=[NBRQ])?|O(-O){1,2})[+#]?").unwrap();
        if !re.is_match(move_str) || move_str.starts_with('x') {
            return Err("Invalid move");
        }
        todo!()
    }

    pub fn fen(&self) -> String {
        let mut fen = String::new();
        fen.push_str(&self.board.to_string());
        fen.push(' ');
        fen.push(if self.is_white_turn { 'w' } else { 'b' });
        fen.push(' ');
        if self.castling_rights == 0 {
            fen.push('-');
        } else {
            if self.castling_rights & 0b1000 != 0 {
                fen.push('K');
            }
            if self.castling_rights & 0b0100 != 0 {
                fen.push('Q');
            }
            if self.castling_rights & 0b0010 != 0 {
                fen.push('k');
            }
            if self.castling_rights & 0b0001 != 0 {
                fen.push('q');
            }
        }
        fen.push(' ');
        fen.push_str(
            &self
                .en_passant
                .as_ref()
                .map_or(String::from("-"), |pos| pos.to_string()),
        );
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }

    pub fn undo(&mut self) {
        todo!()
    }

    pub fn redo(&mut self) {
        todo!()
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        self.fen()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_fen() {
        let game = super::Game::default();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_move_piece() {
        let mut game = super::Game::default();
        game.move_piece("e4").unwrap();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
        );
    }
}
