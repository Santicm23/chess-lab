use regex::Regex;

use crate::{
    common::errors::{fen::FenError, movements::MoveError},
    constants::{CastleType, MoveType, PieceType, Position},
};

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
    pub fn new(fen: &str) -> Result<Game, FenError> {
        Ok(Game::from_fen(fen)?)
    }

    pub fn from_fen(fen: &str) -> Result<Game, FenError> {
        let re = Regex::new(r"^([1-8PpNnBbRrQqKk]{1,8}/){7}[1-8PpNnBbRrQqKk]{1,8} [wb] (-|[KQkq]{1,4}) (-|[a-h][1-8]) \d+ ([1-9]\d*)$").unwrap();
        if !re.is_match(fen) {
            return Err(FenError::Invalid);
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
            Some(Position::from_string(parts[3]).unwrap())
        };
        game.halfmove_clock = parts[4].parse::<u8>().unwrap();
        game.fullmove_number = parts[5].parse::<u8>().unwrap();
        Ok(game)
    }

    pub fn move_piece(&mut self, move_str: String) -> Result<(), MoveError> {
        let res = self.parse_move(move_str)?;
        println!("{:?}", res);
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

    /// Parse a move string and return the start and end positions
    ///
    /// # Arguments
    ///
    /// * `move_str` - A string slice that holds the move to be parsed
    ///
    /// # Returns
    ///
    /// * A result type with in a tuple containing the start position
    ///   of the move and the type of move with additional information
    /// * If the move is invalid, an error is returned
    fn parse_move(&self, mut move_str: String) -> Result<MoveType, MoveError> {
        let re =
            Regex::new(r"^([NBRQK]?[a-h]?[1-8]?x?[a-h][1-8](=[NBRQ])?|O(-O){1,2})[+#]?").unwrap();
        if !re.is_match(move_str.as_str()) || move_str.starts_with('x') {
            return Err(MoveError::Invalid);
        }

        if move_str.chars().last().unwrap() == '+' || move_str.chars().last().unwrap() == '#' {
            move_str.remove(move_str.len() - 1);
        }

        if move_str.starts_with('O') {
            let castle_side;
            if move_str == "O-O" {
                if (self.castling_rights & 0b1000 == 0 || !self.is_white_turn)
                    && (self.castling_rights & 0b0010 == 0 || self.is_white_turn)
                {
                    return Err(MoveError::Invalid);
                }
                castle_side = CastleType::KingSide;
            } else if move_str == "O-O-O" {
                if (self.castling_rights & 0b0100 == 0 || !self.is_white_turn)
                    && (self.castling_rights & 0b0001 == 0 || self.is_white_turn)
                {
                    return Err(MoveError::Invalid);
                }
                castle_side = CastleType::QueenSide;
            } else {
                return Err(MoveError::Invalid);
            }
            return Ok(MoveType::Castle { side: castle_side });
        } else {
            let start_col;
            let start_row;
            let end_pos;
            let capture = move_str.contains("x");
            let promotion;
            let end_pos_index;
            let piece = match move_str.chars().next().unwrap() {
                'N' => PieceType::Knight,
                'B' => PieceType::Bishop,
                'R' => PieceType::Rook,
                'Q' => PieceType::Queen,
                'K' => PieceType::King,
                _ => {
                    move_str = format!("P{}", move_str);
                    PieceType::Pawn
                }
            };

            if move_str.contains('=') {
                if "NBRQK".contains(move_str.chars().next().unwrap()) {
                    return Err(MoveError::Invalid);
                }

                promotion = Some(PieceType::from_char(move_str.chars().last().unwrap()).unwrap());
                end_pos = Position::from_string(&move_str[move_str.len() - 4..move_str.len() - 2])
                    .unwrap();
                end_pos_index = move_str.len() - 4;

                if end_pos.row != 0 && end_pos.row != 7 {
                    return Err(MoveError::Invalid);
                }
            } else {
                end_pos = Position::from_string(&move_str[move_str.len() - 2..]).unwrap();
                end_pos_index = move_str.len() - 2;
                promotion = None;
            }

            if end_pos_index > 1 {
                if "abcdefgh".contains(move_str.chars().nth(1).unwrap()) {
                    start_col = Some(move_str.chars().nth(1).unwrap() as u8 - 'a' as u8);
                    if "12345678".contains(move_str.chars().nth(2).unwrap()) {
                        start_row =
                            Some(move_str.chars().nth(2).unwrap().to_digit(10).unwrap() as u8);
                    } else {
                        start_row = None;
                    }
                } else if "12345678".contains(move_str.chars().nth(1).unwrap()) {
                    start_col = None;
                    start_row = Some(move_str.chars().nth(1).unwrap().to_digit(10).unwrap() as u8);
                } else {
                    start_col = None;
                    start_row = None;
                }
            } else {
                start_col = None;
                start_row = None;
            }

            if capture && self.en_passant.is_some() {
                if piece == PieceType::Pawn && end_pos == self.en_passant.unwrap() {
                    return Ok(MoveType::EnPassant {
                        start: (start_col, start_row),
                        end: end_pos,
                    });
                }
            }

            return Ok(MoveType::Normal {
                piece,
                start: (start_col, start_row),
                end: end_pos,
                capture,
                promotion,
            });
        }
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
        game.move_piece(String::from("e2xe8=Q")).unwrap();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
        );
    }
}
