use std::collections::HashMap;

use regex::Regex;

use crate::{
    constants::{
        movements::{diagonal_movement, linear_movement},
        pgn::PgnTree,
        CastleType, Color, DrawReason, GameStatus, Move, MoveType, PieceType, Position, WinReason,
    },
    errors::MoveError,
    logic::pieces::{piece_movement, Piece},
};

use super::board::Board;

/// Represents a game of chess
/// It contains the board, the turn, the halfmove clock, the fullmove number,
/// the en passant square, the castling rights, the start position, the history,
/// a flag to indicate if the king needs to be captured, the previous positions
/// and the game status
///
/// # Example
/// ```
/// use chess_lab::logic::Game;
///
/// let game = Game::default();
/// assert_eq!(game.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
/// ```
///
#[derive(Debug, Clone)]
pub struct Game {
    capture_king: bool,
    pub board: Board,
    pub is_white_turn: bool,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub en_passant: Option<Position>,
    pub castling_rights: u8,
    pub start_position: String,
    pub history: PgnTree<Move>,
    pub prev_positions: HashMap<String, u32>,
    pub game_status: GameStatus,
}

impl Default for Game {
    /// Creates a new game with the default values
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::Game;
    ///
    /// let game = Game::default();
    /// assert_eq!(game.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    fn default() -> Game {
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut map = HashMap::new();
        map.insert(
            String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -"),
            1,
        );

        Game {
            board: Board::default(),
            is_white_turn: true,
            castling_rights: 0b1111,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            start_position: fen,
            history: PgnTree::default(),
            capture_king: false,
            prev_positions: map,
            game_status: GameStatus::InProgress,
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
    /// use chess_lab::logic::Game;
    ///
    /// let game = Game::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", true);
    /// assert_eq!(game.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
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
    /// use chess_lab::logic::Game;
    ///
    /// let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// assert_eq!(game.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    pub fn from_fen(fen: &str) -> Game {
        let re = Regex::new(r"^([1-8PpNnBbRrQqKk]{1,8}/){7}[1-8PpNnBbRrQqKk]{1,8} [wb] (-|[KQkq]{1,4}) (-|[a-h][1-8]) \d+ ([1-9]\d*)$").unwrap();
        assert!(re.is_match(fen), "Invalid FEN");

        let mut game = Game::default();
        game.start_position = fen.to_string();

        game.prev_positions.clear();
        game.prev_positions.insert(game.get_fen_reduced(), 1);

        let parts = fen.split(' ').collect::<Vec<&str>>();
        game.board = Board::new(parts[0]);
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
        game.halfmove_clock = parts[4].parse::<u32>().unwrap();
        game.fullmove_number = parts[5].parse::<u32>().unwrap();
        game
    }

    /// Moves a piece on the board
    ///
    /// # Arguments
    /// * `move_str`: A string slice that holds the move
    ///
    /// # Returns
    /// The game status if the move was successful, otherwise an error
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::Game;
    ///
    /// let mut game = Game::default();
    /// game.move_piece("e4").unwrap();
    /// assert_eq!(game.to_string(), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
    /// ```
    ///
    pub fn move_piece(&mut self, move_str: &str) -> Result<GameStatus, MoveError> {
        if self.game_status != GameStatus::InProgress {
            return Ok(self.game_status);
        }

        let (piece_type, start_pos_info, end_pos, move_type) = self.parse_move(move_str)?;
        let color = if self.is_white_turn {
            Color::White
        } else {
            Color::Black
        };

        let start_pos = self.find_piece(piece_type, color, start_pos_info, &end_pos, &move_type)?;

        let mut rook_start: Option<Position> = None;
        let mut captured_piece: Option<PieceType> =
            self.board.get_piece(&end_pos).map(|p| p.piece_type);

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
                                        rook_start = Some(rook);
                                        self.board.move_piece(&rook, &rook_end).unwrap();
                                        break;
                                    }
                                }
                                CastleType::QueenSide => {
                                    if rook.col < start_pos.col && rook.row == start_pos.row {
                                        rook_start = Some(rook);
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
                        captured_piece =
                            Some(self.board.delete_piece(&captured_pos).unwrap().piece_type);
                    }
                    _ => {}
                }
                if let MoveType::Normal {
                    capture: _,
                    promotion: Some(piece_type),
                } = move_type
                {
                    self.board.delete_piece(&end_pos).unwrap();
                    self.board
                        .set_piece(Piece::new(color, piece_type), &end_pos)
                        .unwrap();
                }
                let ambiguity =
                    self.move_ambiguity(piece_type, color, start_pos_info, &end_pos, &move_type);

                self.update_rules(Move::new(
                    Piece::new(color, piece_type),
                    start_pos,
                    end_pos,
                    move_type,
                    captured_piece,
                    rook_start,
                    ambiguity,
                    false,
                    false,
                ));

                Ok(self.game_status)
            }
            Err(_) => Err(MoveError::Illegal),
        }
    }

    /// Parses a move string
    ///
    /// # Arguments
    /// * `mov`: A move that holds the piece type, start and end position, the move type, the captured piece and the rook start position
    ///
    fn update_rules(&mut self, mut mov: Move) {
        self.is_white_turn = !self.is_white_turn;

        mov.check = self.check();
        mov.checkmate = self.checkmate();

        self.history.add_move(
            mov.clone(),
            self.halfmove_clock,
            self.fullmove_number,
            self.en_passant,
            self.castling_rights,
            self.game_status,
        );

        if matches!(mov.move_type, MoveType::Castle { .. })
            || mov.piece.piece_type == PieceType::King
        {
            self.castling_rights &= match mov.piece.color {
                Color::White => 0b0011,
                Color::Black => 0b1100,
            };
        }
        if mov.piece.piece_type == PieceType::Rook {
            let king = self.board.find(PieceType::King, mov.piece.color)[0];
            match mov.piece.color {
                Color::White => {
                    if mov.from.col < king.col {
                        self.castling_rights &= 0b1011;
                    } else if mov.from.col > king.col {
                        self.castling_rights &= 0b0111;
                    }
                }
                Color::Black => {
                    if mov.from.col < king.col {
                        self.castling_rights &= 0b1110;
                    } else if mov.from.col > king.col {
                        self.castling_rights &= 0b1101;
                    }
                }
            }
        }
        if let Some(PieceType::Rook) = mov.captured_piece {
            let king = self.board.find(PieceType::King, mov.piece.color)[0];
            match mov.piece.color.opposite() {
                Color::White => {
                    if mov.to.col < king.col {
                        self.castling_rights &= 0b1011;
                    } else if mov.to.col > king.col {
                        self.castling_rights &= 0b0111;
                    }
                }
                Color::Black => {
                    if mov.to.col < king.col {
                        self.castling_rights &= 0b1110;
                    } else if mov.to.col > king.col {
                        self.castling_rights &= 0b1101;
                    }
                }
            }
        }
        if matches!(mov.move_type, MoveType::Normal { capture: true, .. })
            || mov.piece.piece_type == PieceType::Pawn
        {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }
        if mov.piece.piece_type == PieceType::Pawn && (&mov.from - &mov.to).1.abs() == 2 {
            let positions = self.board.find(PieceType::Pawn, mov.piece.color.opposite());
            let en_passant_pos = Position {
                col: mov.from.col,
                row: (mov.from.row + mov.to.row) / 2,
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
        if self.is_white_turn {
            self.fullmove_number += 1;
        }

        let current_pos = self.get_fen_reduced();
        let posistions = *self.prev_positions.get(&current_pos).unwrap_or(&0);

        self.prev_positions.insert(current_pos, posistions + 1);

        if mov.checkmate {
            self.game_status = if self.is_white_turn {
                GameStatus::BlackWins(WinReason::Checkmate)
            } else {
                GameStatus::WhiteWins(WinReason::Checkmate)
            };
        } else if self.stalemate() {
            self.game_status = GameStatus::Draw(DrawReason::Stalemate);
        } else if posistions == 2 {
            self.game_status = GameStatus::Draw(DrawReason::ThreefoldRepetition);
        } else if self.halfmove_clock >= 100 {
            self.game_status = GameStatus::Draw(DrawReason::FiftyMoveRule);
        } else {
            self.game_status = GameStatus::InProgress;
        };

        if self.game_status != GameStatus::InProgress {
            self.history.game_over(self.game_status);
        }
    }

    /// Returns the FEN representation of the game
    ///
    /// # Returns
    /// A string that holds the FEN representation of the game
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::Game;
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

    /// Undoes the last move
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::Game;
    ///
    /// let mut game = Game::default();
    /// game.move_piece("e4").unwrap();
    /// game.undo();
    /// assert_eq!(game.fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    pub fn undo(&mut self) {
        let mov = self.history.get_move();
        let info = self.history.get_prev_move_info();

        if let None = mov {
            return;
        }

        let mov = mov.unwrap();

        self.board.move_piece(&mov.to, &mov.from).unwrap();

        match mov.move_type {
            MoveType::Normal {
                capture: true,
                promotion,
            } => {
                self.board
                    .set_piece(
                        Piece::new(mov.piece.color.opposite(), mov.captured_piece.unwrap()),
                        &mov.to,
                    )
                    .unwrap();
                if let Some(_) = promotion {
                    self.board.delete_piece(&mov.from).unwrap();
                    self.board
                        .set_piece(Piece::new(mov.piece.color, PieceType::Pawn), &mov.from)
                        .unwrap();
                }
            }
            MoveType::EnPassant => {
                let captured_pos = Position {
                    col: mov.to.col,
                    row: mov.from.row,
                };
                self.board
                    .set_piece(
                        Piece::new(mov.piece.color.opposite(), mov.captured_piece.unwrap()),
                        &captured_pos,
                    )
                    .unwrap();
            }
            MoveType::Castle { side } => {
                let rook_from = mov.rook_from.unwrap();
                let rook_to = match side {
                    CastleType::KingSide => Position {
                        col: 5,
                        row: mov.to.row,
                    },
                    CastleType::QueenSide => Position {
                        col: 3,
                        row: mov.to.row,
                    },
                };
                self.board.move_piece(&rook_to, &rook_from).unwrap();
            }
            _ => {}
        }

        self.is_white_turn = !self.is_white_turn;

        self.halfmove_clock = info.0;
        self.fullmove_number = info.1;
        self.en_passant = info.2;
        self.castling_rights = info.3;
        self.game_status = info.4;

        self.history.prev_move();
    }

    /// Redoes the last undone move
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::Game;
    ///
    /// let mut game = Game::default();
    /// game.move_piece("e4").unwrap();
    /// game.undo();
    /// game.redo();
    ///
    /// assert_eq!(game.fen(), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
    /// ```
    ///
    pub fn redo(&mut self) {
        let mov = self.history.next_move();

        if let None = mov {
            return;
        }

        let mov = mov.unwrap();
        self.history.prev_move();

        self.move_piece(mov.to_string().as_str()).unwrap();
    }

    /// Redoes the nth variation of the last undone move
    ///
    /// # Arguments
    /// * `n` - The number of the variation to redo
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::Game;
    ///
    /// let mut game = Game::default();
    /// game.move_piece("e4").unwrap();
    /// game.undo();
    /// game.move_piece("d4").unwrap();
    /// game.undo();
    /// game.redo_nth(1);
    /// assert_eq!(
    ///     game.fen(),
    ///     "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1"
    /// );
    /// ```
    ///
    pub fn redo_nth(&mut self, n: u32) {
        let mov = self.history.next_move_variant(n);

        if let None = mov {
            return;
        }

        let mov = mov.unwrap();

        self.move_piece(mov.to_string().as_str()).unwrap();
    }

    /// Undoes all moves until the starting position
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::Game;
    ///
    /// let mut game = Game::default();
    /// game.move_piece("e4").unwrap();
    /// game.move_piece("e5").unwrap();
    /// game.start();
    ///
    /// assert_eq!(game.fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    pub fn start(&mut self) {
        while self.history.has_prev_move() {
            self.undo();
        }
    }

    /// Redoes all moves until the last move
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::Game;
    ///
    /// let mut game = Game::default();
    /// game.move_piece("e4").unwrap();
    /// game.move_piece("e5").unwrap();
    ///
    /// game.start();
    /// game.end();
    ///
    /// assert_eq!(game.fen(), "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
    /// ```
    ///
    pub fn end(&mut self) {
        while self.history.has_next_move() {
            self.redo();
        }
    }

    /// Returns the PGN of the game
    ///
    /// # Returns
    /// A string containing the PGN of the game
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::Game;
    ///
    /// let mut game = Game::default();
    /// game.move_piece("e4").unwrap();
    /// game.move_piece("e5").unwrap();
    /// println!("{}", game.pgn());
    /// ```
    ///
    pub fn pgn(&self) -> String {
        self.history.pgn()
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
    pub fn parse_move(
        &self,
        move_str: &str,
    ) -> Result<(PieceType, (Option<u8>, Option<u8>), Position, MoveType), MoveError> {
        let mut move_str = move_str.to_string();
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
    pub fn is_legal(
        &self,
        piece: &Piece,
        start_pos: &Position,
        end_pos: &Position,
        move_type: &MoveType,
    ) -> bool {
        if piece.piece_type != PieceType::Knight && piece.piece_type != PieceType::King {
            if !linear_movement(start_pos, end_pos) && !diagonal_movement(start_pos, end_pos) {
                return false;
            }
            if self.board.piece_between(start_pos, end_pos) {
                return false;
            }
        }

        if let MoveType::Castle { side } = move_type {
            return self.is_castle_legal(piece, start_pos, end_pos, side);
        }
        if !piece_movement(piece, start_pos, end_pos) {
            return false;
        }
        if let MoveType::Normal {
            capture: true,
            promotion: _,
        } = move_type
        {
            if !self.board.is_ocupied(end_pos)
                || self.board.get_piece(end_pos).unwrap().color == piece.color
                || (piece.piece_type == PieceType::Pawn && start_pos.col == end_pos.col)
            {
                return false;
            }
        }
        if piece.piece_type == PieceType::Pawn
            && matches!(
                move_type,
                MoveType::Normal {
                    capture: false,
                    promotion: _
                }
            )
        {
            if self.board.get_piece(end_pos).is_some() || start_pos.col != end_pos.col {
                return false;
            }
        }

        if self.capture_king {
            return true;
        }

        let mut board = self.board.clone();
        board.move_piece(start_pos, end_pos).unwrap();

        let king = self.board.find(PieceType::King, piece.color)[0];
        return !board.is_attacked(king, piece.color.opposite());
    }

    /// Returns whether the king is in check
    ///
    /// # Returns
    /// Whether the king is in check
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::Game;
    ///
    /// let mut game = Game::default();
    ///
    /// game.move_piece("c4").unwrap();
    /// game.move_piece("d6").unwrap();
    /// game.move_piece("Qa4+").unwrap();
    ///
    /// assert!(game.check());
    /// ```
    ///
    pub fn check(&self) -> bool {
        if self.capture_king {
            return false;
        }
        let color = if self.is_white_turn {
            Color::White
        } else {
            Color::Black
        };

        if self
            .board
            .is_attacked(self.board.find(PieceType::King, color)[0], color.opposite())
        {
            return true;
        }
        false
    }

    /// Returns whether the king is in checkmate
    ///
    /// # Returns
    /// Whether the king is in checkmate
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::Game;
    ///
    /// let mut game = Game::default();
    ///
    /// game.move_piece("e4").unwrap();
    /// game.move_piece("e5").unwrap();
    /// game.move_piece("Qh5").unwrap();
    /// game.move_piece("Nc6").unwrap();
    /// game.move_piece("Bc4").unwrap();
    /// game.move_piece("Nf6").unwrap();
    /// game.move_piece("Qxf7#").unwrap();
    ///
    /// assert!(game.checkmate());
    /// ```
    ///
    pub fn checkmate(&self) -> bool {
        if self.capture_king {
            let color = if self.is_white_turn {
                Color::White
            } else {
                Color::Black
            };
            let kings = self.board.find(PieceType::King, color);
            return !kings.is_empty();
        }
        if !self.check() {
            return false;
        }

        !self.has_legal_moves()
    }

    /// Returns whether the game is in stalemate
    ///
    /// # Returns
    /// Whether the game is in stalemate
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::Game;
    ///
    /// let game = Game::from_fen("8/8/8/8/8/4KQ2/8/4k3 b - - 0 1");
    ///
    /// assert!(game.stalemate());
    /// ```
    ///
    pub fn stalemate(&self) -> bool {
        if self.capture_king {
            return false;
        }
        if self.check() {
            return false;
        }

        !self.has_legal_moves()
    }

    /// Ends the game and sets the winner to the opposite of the color that resigned
    ///
    /// # Arguments
    /// * `color`: The color of the player that resigned
    ///
    /// # Example
    /// ```
    /// use chess_lab::logic::{Game};
    /// use chess_lab::constants::{Color, GameStatus, WinReason};
    ///
    /// let mut game = Game::default();
    /// game.resign(Color::White);
    ///
    /// assert_eq!(game.game_status, GameStatus::BlackWins(WinReason::Resignation));
    /// ```
    ///
    pub fn resign(&mut self, color: Color) {
        self.game_status = if color == Color::White {
            GameStatus::BlackWins(WinReason::Resignation)
        } else {
            GameStatus::WhiteWins(WinReason::Resignation)
        };
    }

    /// Ends the game and sets the winner to the opposite of the color that lost on time
    ///
    /// # Arguments
    /// * `color`: The color of the player that lost on time
    ///
    /// # Example
    /// ```
    /// use chess_lab::constants::{Color, GameStatus, WinReason};
    /// use chess_lab::logic::Game;
    ///
    /// let mut game = Game::default();
    /// game.set_lost_in_time(Color::White);
    ///
    /// assert_eq!(game.game_status, GameStatus::BlackWins(WinReason::Time));
    /// ```
    ///
    pub fn set_lost_in_time(&mut self, color: Color) {
        self.game_status = if color == Color::White {
            GameStatus::BlackWins(WinReason::Time)
        } else {
            GameStatus::WhiteWins(WinReason::Time)
        };
    }

    /// Ends the game by a draw due to agreement
    ///
    /// # Example
    /// ```
    /// use chess_lab::constants::{GameStatus, DrawReason};
    /// use chess_lab::logic::Game;
    ///
    /// let mut game = Game::default();
    /// game.set_draw_by_agreement();
    ///
    /// assert_eq!(game.game_status, GameStatus::Draw(DrawReason::Agreement));
    ///
    pub fn set_draw_by_agreement(&mut self) {
        self.game_status = GameStatus::Draw(DrawReason::Agreement);
    }

    /// Finds the position of a piece that matches the given criteria to move
    ///
    /// # Arguments
    /// * `piece`: The type of piece to find
    /// * `color`: The color of the piece to find
    /// * `start_pos`: The criteria for the starting position of the piece to find
    /// * `end_pos`: The ending position of the piece to find
    /// * `move_type`: The type of move to find
    ///
    /// # Returns
    /// The position of the piece on the board
    /// If the piece is not found or there are multiple pieces that match the criteria, an error is returned
    ///
    fn find_piece(
        &self,
        piece: PieceType,
        color: Color,
        start_pos: (Option<u8>, Option<u8>),
        end_pos: &Position,
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

        let mut valid_positions = Vec::new();
        for pos in positions {
            if self.is_legal(
                &Piece {
                    color,
                    piece_type: piece,
                },
                &pos,
                &end_pos,
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

    /// Checks if the move representation has to contain the column or row of the piece to move
    /// asuming that the move is legal
    ///
    /// # Arguments
    /// * `piece`: The type of piece to move
    /// * `color`: The color of the piece to move
    /// * `start_pos`: The starting position of the piece to move
    /// * `end_pos`: The ending position of the piece to move
    /// * `move_type`: The type of move to make
    ///
    /// # Returns
    /// A tuple containing two booleans:
    /// * The first boolean indicates if the column of the piece to move has to be included in the move representation
    /// * The second boolean indicates if the row of the piece to move has to be included in the move representation
    ///
    fn move_ambiguity(
        &self,
        piece: PieceType,
        color: Color,
        start_pos: (Option<u8>, Option<u8>),
        end_pos: &Position,
        move_type: &MoveType,
    ) -> (bool, bool) {
        match start_pos {
            (None, None) => (false, false),
            (Some(_), None) => {
                let positions = self.board.find(piece, color);

                let valid_positions = positions
                    .iter()
                    .filter(|pos| {
                        self.is_legal(
                            &Piece {
                                color,
                                piece_type: piece,
                            },
                            &pos,
                            &end_pos,
                            &move_type,
                        )
                    })
                    .count();
                (valid_positions > 1, false)
            }
            (None, Some(_)) => {
                let positions = self.board.find(piece, color);
                let valid_positions = positions
                    .iter()
                    .filter(|pos| {
                        self.is_legal(
                            &Piece {
                                color,
                                piece_type: piece,
                            },
                            &pos,
                            &end_pos,
                            &move_type,
                        )
                    })
                    .count();
                (false, valid_positions > 1)
            }
            (Some(col), Some(row)) => {
                let positions = self.board.find(piece, color);
                let col_ambiguity = positions
                    .iter()
                    .filter(|pos| pos.row == row)
                    .filter(|pos| {
                        self.is_legal(
                            &Piece {
                                color,
                                piece_type: piece,
                            },
                            &pos,
                            &end_pos,
                            &move_type,
                        )
                    })
                    .count()
                    > 1;
                let row_ambiguity = positions
                    .iter()
                    .filter(|pos| pos.col == col)
                    .filter(|pos| {
                        self.is_legal(
                            &Piece {
                                color,
                                piece_type: piece,
                            },
                            &pos,
                            &end_pos,
                            &move_type,
                        )
                    })
                    .count()
                    > 1;
                (col_ambiguity, row_ambiguity)
            }
        }
    }

    /// Checks if castling is legal
    ///
    /// # Arguments
    /// * `piece`: The king piece to castle
    /// * `start_pos`: The starting position of the king piece
    /// * `end_pos`: The ending position of the king piece
    /// * `side`: The side to castle
    ///
    /// # Returns
    /// A boolean indicating if the castling is legal
    ///
    fn is_castle_legal(
        &self,
        piece: &Piece,
        start_pos: &Position,
        end_pos: &Position,
        side: &CastleType,
    ) -> bool {
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
                    let new_pos = Position::new(col, start_pos.row);
                    if (&new_pos != start_pos && self.board.is_ocupied(&new_pos))
                        || self
                            .board
                            .is_attacked(Position::new(col, start_pos.row), piece.color.opposite())
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
                        || self
                            .board
                            .is_attacked(Position::new(col, start_pos.row), piece.color.opposite())
                    {
                        return false;
                    }
                }
                return true;
            }
        }
    }

    /// Checks if there are legal moves for the current player
    ///
    /// # Returns
    /// A boolean indicating if there are legal moves for the current player
    ///
    fn has_legal_moves(&self) -> bool {
        let color = if self.is_white_turn {
            Color::White
        } else {
            Color::Black
        };

        for piece_pos in self.board.find_all(color) {
            let piece = self.board.get_piece(&piece_pos).unwrap();
            for col in 0..8 {
                for row in 0..8 {
                    let end_pos = Position::new(col, row);

                    let mut board = self.board.clone();
                    if !board.can_capture(&piece_pos, &end_pos) {
                        continue;
                    }

                    board.move_piece(&piece_pos, &end_pos).unwrap();

                    let king = board.find(PieceType::King, piece.color)[0];
                    if !board.is_attacked(king, piece.color.opposite()) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Gives the FEN string of the position withouth the halfmove clock and fullmove number
    /// to be used as position identifier
    ///
    /// # Returns
    /// The FEN string of the position withouth the halfmove clock and fullmove number
    ///
    fn get_fen_reduced(&self) -> String {
        let fen = self.fen();
        let mut fen_parts: Vec<&str> = fen.split_whitespace().collect();
        fen_parts.pop();
        fen_parts.pop();
        fen_parts.join(" ")
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
    /// use chess_lab::logic::Game;
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
    use super::Game;

    #[test]
    fn test_fen() {
        let game = Game::default();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_from_fen() {
        let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_move_piece() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn test_castle() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("Nf3").unwrap();
        game.move_piece("Nc6").unwrap();
        game.move_piece("Bb5").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("O-O").unwrap();
        assert_eq!(
            game.fen(),
            "r1bqkbnr/1ppp1ppp/p1n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 1 4"
        );
    }

    #[test]
    fn test_en_passant() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("d5").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("f5").unwrap();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3"
        );
        game.move_piece("exf6").unwrap();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/ppp1p1pp/5P2/3p4/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3"
        );
    }

    #[test]
    fn test_castle_rights() {
        let mut game = Game::default();
        game.move_piece("a3").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("Ra2").unwrap();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/1ppppppp/p7/8/8/P7/RPPPPPPP/1NBQKBNR b Kkq - 1 2"
        );
        game.move_piece("Ra7").unwrap();
        assert_eq!(
            game.fen(),
            "1nbqkbnr/rppppppp/p7/8/8/P7/RPPPPPPP/1NBQKBNR w Kk - 2 3"
        );

        game.move_piece("h3").unwrap();
        game.move_piece("h6").unwrap();
        game.move_piece("Rh2").unwrap();
        assert_eq!(
            game.fen(),
            "1nbqkbnr/rpppppp1/p6p/8/8/P6P/RPPPPPPR/1NBQKBN1 b k - 1 4"
        );
        game.move_piece("Rh7").unwrap();
        assert_eq!(
            game.fen(),
            "1nbqkbn1/rppppppr/p6p/8/8/P6P/RPPPPPPR/1NBQKBN1 w - - 2 5"
        );
    }

    #[test]
    fn test_promotion() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("d5").unwrap();
        game.move_piece("exd5").unwrap();
        game.move_piece("c6").unwrap();
        game.move_piece("dxc6").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("cxb7").unwrap();
        game.move_piece("a5").unwrap();
        game.move_piece("bxa8=Q").unwrap();
        assert_eq!(
            game.fen(),
            "Qnbqkbnr/4pppp/8/p7/8/8/PPPP1PPP/RNBQKBNR b KQk - 0 5"
        );
    }

    #[test]
    fn test_undo() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.undo();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_undo_castle() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("Nf3").unwrap();
        game.move_piece("Nc6").unwrap();
        game.move_piece("Bb5").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("O-O").unwrap();
        game.undo();
        assert_eq!(
            game.fen(),
            "r1bqkbnr/1ppp1ppp/p1n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 4"
        );
    }

    #[test]
    fn test_undo_en_passant() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("d5").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("f5").unwrap();
        game.move_piece("exf6").unwrap();
        game.undo();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3"
        );
    }

    #[test]
    fn test_undo_promotion() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("d5").unwrap();
        game.move_piece("exd5").unwrap();
        game.move_piece("c6").unwrap();
        game.move_piece("dxc6").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("cxb7").unwrap();
        game.move_piece("a5").unwrap();
        game.move_piece("bxa8=Q").unwrap();
        game.undo();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/1P2pppp/8/p7/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 5"
        );
    }

    #[test]
    fn test_undo_castle_rights() {
        let mut game = Game::default();
        game.move_piece("a3").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("Ra2").unwrap();
        game.move_piece("Ra7").unwrap();
        game.move_piece("h3").unwrap();
        game.move_piece("h6").unwrap();
        game.move_piece("Rh2").unwrap();
        game.move_piece("Rh7").unwrap();
        game.undo();
        assert_eq!(
            game.fen(),
            "1nbqkbnr/rpppppp1/p6p/8/8/P6P/RPPPPPPR/1NBQKBN1 b k - 1 4"
        );
    }

    #[test]
    fn test_redo() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.undo();
        game.redo();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );
        game.undo();
    }

    #[test]
    fn test_redo_castle() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("Nf3").unwrap();
        game.move_piece("Nc6").unwrap();
        game.move_piece("Bb5").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("O-O").unwrap();
        game.undo();
        game.redo();
        assert_eq!(
            game.fen(),
            "r1bqkbnr/1ppp1ppp/p1n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 1 4"
        );
    }

    #[test]
    fn test_redo_en_passant() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("d5").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("f5").unwrap();
        game.move_piece("exf6").unwrap();
        game.undo();
        game.redo();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/ppp1p1pp/5P2/3p4/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3"
        );
    }

    #[test]
    fn test_redo_promotion() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("d5").unwrap();
        game.move_piece("exd5").unwrap();
        game.move_piece("c6").unwrap();
        game.move_piece("dxc6").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("cxb7").unwrap();
        game.move_piece("a5").unwrap();
        game.move_piece("bxa8=Q").unwrap();
        game.undo();
        game.redo();
        assert_eq!(
            game.fen(),
            "Qnbqkbnr/4pppp/8/p7/8/8/PPPP1PPP/RNBQKBNR b KQk - 0 5"
        );
    }

    #[test]
    fn test_redo_castle_rights() {
        let mut game = Game::default();
        game.move_piece("a3").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("Ra2").unwrap();
        game.move_piece("Ra7").unwrap();
        game.move_piece("h3").unwrap();
        game.move_piece("h6").unwrap();
        game.move_piece("Rh2").unwrap();
        game.move_piece("Rh7").unwrap();
        game.undo();
        game.redo();
        assert_eq!(
            game.fen(),
            "1nbqkbn1/rppppppr/p6p/8/8/P6P/RPPPPPPR/1NBQKBN1 w - - 2 5"
        );
    }

    #[test]
    fn test_redo_principal_line() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.undo();
        game.move_piece("d4").unwrap();
        game.undo();

        assert_eq!(game.history.all_next_moves().len(), 2);

        game.redo();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn test_multiple_redos() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("Nf3").unwrap();
        game.move_piece("Nc6").unwrap();
        game.move_piece("Bb5").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("O-O").unwrap();

        game.start();

        game.redo();
        game.redo();
        game.redo();
        game.redo();
        game.redo();
        game.redo();
        game.redo();

        assert_eq!(
            game.fen(),
            "r1bqkbnr/1ppp1ppp/p1n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 1 4"
        );
    }

    #[test]
    fn test_auto_redo() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("e5").unwrap();
        game.undo();
        game.undo();
        game.move_piece("e4").unwrap();
        game.redo();

        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2"
        );
    }

    #[test]
    fn test_play_other_move() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.undo();
        game.move_piece("d4").unwrap();

        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1"
        );

        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("Nf3").unwrap();
        game.undo();
        game.move_piece("Nc3").unwrap();

        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppp1ppp/8/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 1 2"
        );
    }

    #[test]
    fn redo_nth() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.undo();
        game.move_piece("d4").unwrap();
        game.undo();
        game.redo_nth(1);
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn test_start() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("Nf3").unwrap();
        game.move_piece("Nc6").unwrap();
        game.move_piece("Bb5").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("O-O").unwrap();

        game.start();

        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_end() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("Nf3").unwrap();
        game.move_piece("Nc6").unwrap();
        game.move_piece("Bb5").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("O-O").unwrap();

        game.start();
        game.end();

        assert_eq!(
            game.fen(),
            "r1bqkbnr/1ppp1ppp/p1n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 1 4"
        );
    }

    #[test]
    fn test_pgn() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("Nf3").unwrap();
        game.move_piece("Nc6").unwrap();
        game.move_piece("Bb5").unwrap();
        game.move_piece("a6").unwrap();
        game.move_piece("Ba4").unwrap();
        game.move_piece("Nf6").unwrap();
        game.move_piece("O-O").unwrap();
        game.move_piece("Be7").unwrap();
        game.move_piece("Re1").unwrap();
        game.move_piece("b5").unwrap();
        game.move_piece("Bb3").unwrap();
        game.move_piece("O-O").unwrap();
        game.move_piece("c3").unwrap();
        game.move_piece("d5").unwrap();
        game.undo();
        game.undo();
        game.undo();
        game.undo();
        game.undo();
        game.move_piece("O-O").unwrap();
        game.move_piece("c3").unwrap();
        game.move_piece("b5").unwrap();
        game.move_piece("Bc2").unwrap();

        let pgn = game.pgn();
        assert!(
            pgn.contains(
                "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 (6... O-O 7. c3 b5 8. Bc2) 7. Bb3 O-O 8. c3 d5"
            )
        );
    }

    #[test]
    fn test_check() {
        let mut game = Game::default();
        game.move_piece("c4").unwrap();
        game.move_piece("d6").unwrap();
        game.move_piece("Qa4+").unwrap();
        assert!(game.check());
    }

    #[test]
    fn test_check_pgn() {
        let mut game = Game::default();
        game.move_piece("c4").unwrap();
        game.move_piece("d6").unwrap();
        game.move_piece("Qa4+").unwrap();
        assert!(game.pgn().contains("1. c4 d6 2. Qa4+"));
    }

    #[test]
    fn test_checkmate() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("Qh5").unwrap();
        game.move_piece("Nc6").unwrap();
        game.move_piece("Bc4").unwrap();
        game.move_piece("Nf6").unwrap();
        game.move_piece("Qxf7#").unwrap();
        assert!(game.checkmate());
    }

    #[test]
    fn test_checkmate_pgn() {
        let mut game = Game::default();
        game.move_piece("e4").unwrap();
        game.move_piece("e5").unwrap();
        game.move_piece("Qh5").unwrap();
        game.move_piece("Nc6").unwrap();
        game.move_piece("Bc4").unwrap();
        game.move_piece("Nf6").unwrap();
        game.move_piece("Qxf7#").unwrap();
        assert!(game
            .pgn()
            .contains("[Result \"1-0\"]\n1. e4 e5 2. Qh5 Nc6 3. Bc4 Nf6 4. Qxf7# 1-0\n"));
    }

    #[test]
    fn test_stalemate() {
        let game = Game::from_fen("8/8/8/8/8/4KQ2/8/4k3 b - - 0 1");
        assert!(game.stalemate());
    }
}
