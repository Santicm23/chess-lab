use regex::Regex;

use crate::{
    constants::{CastleType, Color, MoveType, PieceType, Position},
    errors::MoveError,
    logic::pieces::{piece_movement, Piece},
};

use super::board::Board;

/// Represents a game of chess
/// It contains the board, the turn, the halfmove clock, the fullmove number,
/// the en passant square, the castling rights, the start position, the history
/// and a flag to indicate if the king needs to be captured
///
/// # Example
/// ```
/// use chess_lib::logic::Game;
///
/// let game = Game::default();
/// assert_eq!(game.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
/// ```
///
#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub is_white_turn: bool,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
    pub en_passant: Option<Position>,
    pub castling_rights: u8,
    pub start_position: String,
    pub history: Vec<&'static str>,
    pub capture_king: bool,
}

impl Default for Game {
    /// Creates a new game with the default values
    ///
    /// # Example
    /// ```
    /// use chess_lib::logic::Game;
    ///
    /// let game = Game::default();
    /// assert_eq!(game.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    fn default() -> Game {
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
            capture_king: false,
        }
    }
}

impl Game {
    /// Creates a new game
    ///
    /// # Arguments
    /// * `fen`: A string slice that holds the FEN representation of the game
    /// * `capture_king`: A boolean that indicates if the king needs to be captured
    ///
    /// # Returns
    /// A new game
    ///
    /// # Panics
    /// Panics if the FEN is invalid
    ///
    /// # Example
    /// ```
    /// use chess_lib::logic::Game;
    ///
    /// let game = Game::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", true);
    /// assert_eq!(game.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// assert_eq!(game.capture_king, true);
    /// ```
    ///
    pub fn new(fen: &str, capture_king: bool) -> Game {
        let mut game = Game::from_fen(fen);

        game.capture_king = capture_king;

        game
    }

    /// Creates a new game from a FEN string
    ///
    /// # Arguments
    /// * `fen`: A string slice that holds the FEN representation of the game
    ///
    /// # Returns
    /// A new game
    ///
    /// # Panics
    /// Panics if the FEN is invalid
    ///
    /// # Example
    /// ```
    /// use chess_lib::logic::Game;
    ///
    /// let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// assert_eq!(game.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    pub fn from_fen(fen: &str) -> Game {
        let re = Regex::new(r"^([1-8PpNnBbRrQqKk]{1,8}/){7}[1-8PpNnBbRrQqKk]{1,8} [wb] (-|[KQkq]{1,4}) (-|[a-h][1-8]) \d+ ([1-9]\d*)$").unwrap();
        assert!(re.is_match(fen), "Invalid FEN");

        let mut game = Game::default();
        game.start_position = String::from(fen);

        let parts = fen.split(' ').collect::<Vec<&str>>();
        game.board = Board::from_fen(parts[0]);
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
        game
    }

    /// Moves a piece on the board
    ///
    /// # Arguments
    /// * `move_str`: A string slice that holds the move
    ///
    /// # Returns
    /// Nothing if the move was successful, otherwise an error
    ///
    /// # Example
    /// ```
    /// use chess_lib::logic::Game;
    ///
    /// let mut game = Game::default();
    /// game.move_piece("e4".to_string()).unwrap();
    /// assert_eq!(game.to_string(), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
    /// ```
    ///
    pub fn move_piece(&mut self, move_str: String) -> Result<(), MoveError> {
        let (piece_type, start_pos, end_pos, move_type) = self.parse_move(move_str)?;
        let color = if self.is_white_turn {
            Color::White
        } else {
            Color::Black
        };

        let start_pos = self.find_piece(piece_type, color, start_pos, end_pos, &move_type)?;

        match self.board.move_piece(&start_pos, &end_pos) {
            Ok(_) => {
                match &move_type {
                    MoveType::Castle { side } => {
                        let rook_end = match side {
                            CastleType::KingSide => Position {
                                col: 5,
                                row: start_pos.row,
                            },
                            CastleType::QueenSide => Position {
                                col: 3,
                                row: start_pos.row,
                            },
                        };

                        let rooks = self.board.find(PieceType::Rook, color);

                        for rook in rooks {
                            match side {
                                CastleType::KingSide => {
                                    if rook.col > start_pos.col && rook.row == start_pos.row {
                                        self.board.move_piece(&rook, &rook_end).unwrap();
                                        break;
                                    }
                                }
                                CastleType::QueenSide => {
                                    if rook.col < start_pos.col && rook.row == start_pos.row {
                                        self.board.move_piece(&rook, &rook_end).unwrap();
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    MoveType::EnPassant => {
                        let captured_pos = Position {
                            col: end_pos.col,
                            row: start_pos.row,
                        };
                        self.board.delete_piece(&captured_pos).unwrap();
                    }
                    _ => {}
                }
                if piece_type == PieceType::Pawn && (&start_pos - &end_pos).1.abs() == 2 {
                    let positions = self.board.find(PieceType::Pawn, color.opposite());
                    let en_passant_pos = Position {
                        col: start_pos.col,
                        row: (start_pos.row + end_pos.row) / 2,
                    };

                    let can_en_passant = positions.iter().any(|pos| {
                        let piece = self.board.get_piece(&pos).unwrap();
                        piece_movement(&piece, &pos, &en_passant_pos)
                    });

                    if can_en_passant {
                        self.en_passant = Some(en_passant_pos);
                    } else {
                        self.en_passant = None;
                    }
                } else {
                    self.en_passant = None;
                }

                self.is_white_turn = !self.is_white_turn;
                self.halfmove_clock += 1;
                if self.is_white_turn {
                    self.fullmove_number += 1;
                }
                if PieceType::Pawn == piece_type
                    || matches!(
                        move_type,
                        MoveType::Normal {
                            capture: true,
                            promotion: _
                        }
                    )
                {
                    self.halfmove_clock = 0;
                }
                Ok(())
            }
            Err(_) => Err(MoveError::Illegal),
        }
    }

    /// Returns the FEN representation of the game
    ///
    /// # Returns
    /// A string that holds the FEN representation of the game
    ///
    /// # Example
    /// ```
    /// use chess_lib::logic::Game;
    ///
    /// let game = Game::default();
    /// assert_eq!(game.fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
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
    /// * `move_str`: A string slice that holds the move to be parsed
    ///
    /// # Returns
    /// A tuple containing the piece type, start position, end position and the move type
    /// If the move is invalid, a MoveError is returned
    ///
    fn parse_move(
        &self,
        mut move_str: String,
    ) -> Result<(PieceType, (Option<u8>, Option<u8>), Position, MoveType), MoveError> {
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
            let end_pos;
            if move_str == "O-O" {
                if (self.castling_rights & 0b1000 == 0 || !self.is_white_turn)
                    && (self.castling_rights & 0b0010 == 0 || self.is_white_turn)
                {
                    return Err(MoveError::Invalid);
                }
                castle_side = CastleType::KingSide;
                end_pos = if self.is_white_turn {
                    Position::from_string("g1")
                } else {
                    Position::from_string("g8")
                };
            } else if move_str == "O-O-O" {
                if (self.castling_rights & 0b0100 == 0 || !self.is_white_turn)
                    && (self.castling_rights & 0b0001 == 0 || self.is_white_turn)
                {
                    return Err(MoveError::Invalid);
                }
                castle_side = CastleType::QueenSide;

                end_pos = if self.is_white_turn {
                    Position::from_string("c1")
                } else {
                    Position::from_string("c8")
                };
            } else {
                return Err(MoveError::Invalid);
            }
            return Ok((
                PieceType::King,
                (None, None),
                end_pos,
                MoveType::Castle { side: castle_side },
            ));
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
                end_pos = Position::from_string(&move_str[move_str.len() - 4..move_str.len() - 2]);
                end_pos_index = move_str.len() - 4;

                if end_pos.row != 0 && end_pos.row != 7 {
                    return Err(MoveError::Invalid);
                }
            } else {
                end_pos = Position::from_string(&move_str[move_str.len() - 2..]);
                end_pos_index = move_str.len() - 2;
                promotion = None;
            }

            if end_pos_index > 1 {
                if "abcdefgh".contains(move_str.chars().nth(1).unwrap()) {
                    start_col = Some(move_str.chars().nth(1).unwrap() as u8 - 'a' as u8);
                    if "12345678".contains(move_str.chars().nth(2).unwrap()) {
                        start_row = Some(move_str.chars().nth(2).unwrap() as u8 - '1' as u8);
                    } else {
                        start_row = None;
                    }
                } else if "12345678".contains(move_str.chars().nth(1).unwrap()) {
                    start_col = None;
                    start_row = Some(move_str.chars().nth(1).unwrap() as u8 - '1' as u8);
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
                    return Ok((
                        PieceType::Pawn,
                        (start_col, start_row),
                        end_pos,
                        MoveType::EnPassant,
                    ));
                }
            }

            return Ok((
                piece,
                (start_col, start_row),
                end_pos,
                MoveType::Normal { capture, promotion },
            ));
        }
    }

    fn find_piece(
        &self,
        piece: PieceType,
        color: Color,
        start_pos: (Option<u8>, Option<u8>),
        end_pos: Position,
        move_type: &MoveType,
    ) -> Result<Position, MoveError> {
        let mut positions = match move_type {
            MoveType::Normal {
                capture: _,
                promotion: _,
            } => self.board.find(piece, color),
            MoveType::EnPassant => self.board.find(PieceType::Pawn, color),
            MoveType::Castle { side: _ } => self.board.find(PieceType::King, color),
        };

        positions = positions
            .iter()
            .filter(|pos| match start_pos {
                (Some(col), Some(row)) => pos.col == col && pos.row == row,
                (Some(col), None) => pos.col == col,
                (None, Some(row)) => pos.row == row,
                (None, None) => true,
            })
            .cloned()
            .collect();

        if positions.len() == 0 {
            return Err(MoveError::Illegal);
        } else if positions.len() == 1 {
            return Ok(positions[0]);
        } else {
            let mut valid_positions = Vec::new();
            for pos in positions {
                if self.is_legal(
                    Piece {
                        color,
                        piece_type: piece,
                    },
                    pos,
                    end_pos,
                    &move_type,
                ) {
                    valid_positions.push(pos);
                }
            }
            if valid_positions.len() == 0 {
                return Err(MoveError::Illegal);
            } else if valid_positions.len() == 1 {
                return Ok(valid_positions[0]);
            } else {
                return Err(MoveError::Ambiguous);
            }
        }
    }

    /// Check if a move is legal
    ///
    /// # Arguments
    /// * `piece`: The piece being moved
    /// * `start_pos`: The starting position of the piece
    /// * `end_pos`: The ending position of the piece
    /// * `move_type`: The type of move being made
    ///
    /// # Returns
    /// Whether the move is legal
    ///
    fn is_legal(
        &self,
        piece: Piece,
        start_pos: Position,
        end_pos: Position,
        move_type: &MoveType,
    ) -> bool {
        if self.capture_king {
            return true;
        }

        if let MoveType::Castle { side } = move_type {
            assert!(piece.piece_type == PieceType::King);
            if start_pos.row != end_pos.row {
                return false;
            }

            match side {
                CastleType::KingSide => {
                    if piece.color == Color::White && self.castling_rights & 0b1000 == 0 {
                        return false;
                    } else if piece.color == Color::Black && self.castling_rights & 0b0010 == 0 {
                        return false;
                    }

                    for col in start_pos.col + 0..end_pos.col + 1 {
                        if self.board.is_ocupied(&Position::new(col, start_pos.row))
                            || self.board.is_attacked(
                                Position::new(col, start_pos.row),
                                piece.color.opposite(),
                            )
                        {
                            return false;
                        }
                    }
                    return true;
                }
                CastleType::QueenSide => {
                    if piece.color == Color::White && self.castling_rights & 0b0100 == 0 {
                        return false;
                    } else if piece.color == Color::Black && self.castling_rights & 0b0001 == 0 {
                        return false;
                    }

                    for col in start_pos.col - 0..end_pos.col + 1 {
                        if self.board.is_ocupied(&Position::new(col, start_pos.row))
                            || self.board.is_attacked(
                                Position::new(col, start_pos.row),
                                piece.color.opposite(),
                            )
                        {
                            return false;
                        }
                    }
                    return true;
                }
            }
        }
        if !piece_movement(&piece, &start_pos, &end_pos) {
            return false;
        }

        let mut board = self.board.clone();
        board.move_piece(&start_pos, &end_pos).unwrap();

        let king = self.board.find(PieceType::King, piece.color)[0];
        return !board.is_attacked(king, piece.color.opposite());
    }
}

impl ToString for Game {
    /// Convert the game to a FEN string
    ///
    /// # Returns
    /// The FEN string
    ///
    /// # Example
    /// ```
    /// use chess_lib::logic::Game;
    ///
    /// let game = Game::default();
    /// assert_eq!(game.fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
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
        game.move_piece(String::from("e4")).unwrap();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn test_castle() {
        let mut game = super::Game::default();
        game.move_piece(String::from("e4")).unwrap();
        game.move_piece(String::from("e5")).unwrap();
        game.move_piece(String::from("Nf3")).unwrap();
        game.move_piece(String::from("Nc6")).unwrap();
        game.move_piece(String::from("Bb5")).unwrap();
        game.move_piece(String::from("a6")).unwrap();
        game.move_piece(String::from("O-O")).unwrap();
        assert_eq!(
            game.fen(),
            "r1bqkbnr/1ppp1ppp/p1n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 1 4"
        );
    }

    #[test]
    fn test_en_passant() {
        let mut game = super::Game::default();
        game.move_piece(String::from("e4")).unwrap();
        game.move_piece(String::from("d5")).unwrap();
        game.move_piece(String::from("e5")).unwrap();
        game.move_piece(String::from("f5")).unwrap();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3"
        );
        game.move_piece(String::from("exf6")).unwrap();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/ppp1p1pp/5P2/3p4/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3"
        );
    }
}
