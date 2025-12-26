use std::collections::HashMap;

use crate::{
    core::{Color, GameStatus, Move, PGNTree, Position, Variant, VariantBuilder},
    errors::{FenError, MoveError, PGNError},
    logic::{Board, Game},
    parsing::pgn::{parse_multiple_pgn, parse_pgn},
    utils::os::{read_file, write_file},
};

/// Standard Chess [Variant]
/// This [Variant] is the most common chess variant played worldwide
/// It is played on an 8x8 board with the following pieces for each player:
/// - 8 Pawns
/// - 2 Rooks
/// - 2 Knights
/// - 2 Bishops
/// - 1 Queen
/// - 1 King
///
/// The game is won by checkmating the opponent's king
/// The game is drawn by stalemate, threefold repetition, the fifty-move rule, or agreement
/// The game is lost by resigning or losing on time
///
/// # Attributes
/// * `game` - The game struct that holds the state of the game
///
#[derive(Debug, Clone)]
pub struct StandardChess {
    game: Game,
}

impl Default for StandardChess {
    /// Creates a new instance of the [StandardChess] [Variant] with default values
    ///
    /// # Returns
    /// A new instance of the [StandardChess] [Variant]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::variants::StandardChess;
    /// let game = StandardChess::default();
    /// ```
    ///
    fn default() -> StandardChess {
        StandardChess {
            game: Game::default(),
        }
    }
}

impl VariantBuilder for StandardChess {
    /// Returns the name of the [Variant]
    ///
    /// # Returns
    /// The name of the [Variant]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::StandardChess;
    /// let name = StandardChess::name();
    /// assert_eq!(name, "Standard");
    /// ```
    ///
    fn name() -> &'static str {
        "Standard"
    }

    /// Creates a new instance of the [StandardChess] [Variant]
    ///
    /// # Returns
    /// A new instance of the [StandardChess] [Variant]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::StandardChess;
    /// use chess_lab::logic::Game;
    ///
    /// let game = StandardChess::new(Game::default());
    /// ```
    ///
    fn new(game: Game) -> StandardChess {
        StandardChess { game }
    }

    /// Creates a new instance of the [StandardChess] [Variant] from a FEN string
    ///
    /// # Arguments
    /// * `fen` - A FEN string
    ///
    /// # Returns
    /// * `Ok(StandardChess)` - A new instance of the [StandardChess] [Variant]
    /// * `Err(FenError)` - An error occurred while parsing the FEN string
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::StandardChess;
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let game = StandardChess::from_fen(fen).unwrap();
    /// ```
    ///
    fn from_fen(fen: &str) -> Result<StandardChess, FenError> {
        Ok(StandardChess {
            game: Game::from_fen(fen)?,
        })
    }

    /// Creates a new instance of the [StandardChess] [Variant] from a PGN string
    ///
    /// # Arguments
    /// * `pgn` - A PGN string
    ///
    /// # Returns
    /// A `Result<StandardChess, PGNError>` object
    /// * `Ok(StandardChess)` - A new instance of the [StandardChess] [Variant]
    /// * `Err(PgnError)` - An error occurred while parsing the PGN string
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::StandardChess;
    /// let pgn = "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6";
    ///
    /// let game = StandardChess::from_pgn(pgn).unwrap();
    /// ```
    ///
    fn from_pgn(pgn: &str) -> Result<StandardChess, PGNError> {
        parse_pgn(pgn)
    }

    /// Loads the [Game] from a file
    ///
    /// # Arguments
    /// * `path` - The path to the file
    ///
    /// # Returns
    /// A `Result<StandardChess, PGNError>` object
    /// * `Ok(StandardChess)` - The [Game] was loaded successfully
    /// * `Err(PgnError)` - An error occurred while loading the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::StandardChess;
    ///
    /// let path = "data/standard/ex1.pgn";
    /// let game = StandardChess::load(path).unwrap();
    /// ```
    ///
    fn load(path: &str) -> Result<StandardChess, PGNError> {
        let pgn = read_file(path)?;
        StandardChess::from_pgn(&pgn)
    }

    /// Loads multiple [Games](Game) from a file
    ///
    /// # Arguments
    /// * `path` - The path to the file
    ///
    /// # Returns
    /// A `Result<Vec<StandardChess>, PGNError>` object
    /// * `Ok(Vec<StandardChess>)` - The games were loaded successfully
    /// * `Err(PgnError)` - An error occurred while loading the games
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::StandardChess;
    /// let path = "data/standard/ex3.pgn";
    /// let games = StandardChess::load_all(path).unwrap();
    /// ```
    ///
    fn load_all(path: &str) -> Result<Vec<Self>, PGNError> {
        let pgn = read_file(path)?;
        parse_multiple_pgn(&pgn)
    }
}

impl Variant for StandardChess {
    /// Moves a [Piece] on the [Board]
    ///
    /// # Arguments
    /// * `move_str` - A move string in algebraic notation.
    ///
    /// # Returns
    /// A `Result<GameStatus, MoveError>` object
    /// * `Ok(GameStatus)` - The status of the game after the move.
    /// * `Err(MoveError)` - An error occurred while moving the piece.
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::GameStatus;
    /// # use chess_lab::variants::StandardChess;
    /// let mut game = StandardChess::default();
    /// let status = game.move_piece("e4");
    ///
    /// assert_eq!(status, Ok(GameStatus::InProgress));
    /// ```
    ///
    fn move_piece(&mut self, move_str: &str) -> Result<GameStatus, MoveError> {
        self.game.move_piece(move_str)
    }

    /// Undoes the last [Move]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    ///
    /// let mut game = StandardChess::default();
    /// game.move_piece("e4").unwrap();
    /// game.undo();
    ///
    /// assert_eq!(game.fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    fn undo(&mut self) {
        self.game.undo()
    }

    /// Redoes the last undone [Move]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    /// let mut game = StandardChess::default();
    /// game.move_piece("e4").unwrap();
    /// game.undo();
    /// game.redo();
    ///
    /// assert_eq!(game.fen(), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
    /// ```
    ///
    fn redo(&mut self) {
        self.game.redo()
    }

    /// Returns the PGN string of the [Game]
    ///
    /// # Returns
    /// The PGN string of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    /// let mut game = StandardChess::default();
    /// game.move_piece("e4").unwrap();
    /// game.move_piece("e5").unwrap();
    /// let pgn = game.pgn();
    ///
    /// assert!(pgn.contains("1. e4 e5"));
    /// ```
    ///
    fn pgn(&self) -> String {
        self.game.pgn()
    }

    /// Returns the FEN string of the game
    ///
    /// # Returns
    /// The FEN string of the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    /// let game = StandardChess::default();
    /// let fen = game.fen();
    ///
    /// assert_eq!(fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    fn fen(&self) -> String {
        self.game.fen()
    }

    /// Saves the [Game] to a file
    ///
    /// # Arguments
    /// * `path` - The path to the file
    /// * `overwrite` - Whether to overwrite the file if it already exists
    ///
    /// # Returns
    /// A `Result<(), std::io::Error>` object
    /// * `Ok(())` - The game was saved successfully
    /// * `Err(std::io::Error)` - An error occurred while saving the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    /// let game = StandardChess::default();
    /// let path = "data/standard/ex.pgn";
    ///
    /// game.save(path, true).unwrap();
    /// ```
    ///
    fn save(&self, path: &str, overwrite: bool) -> Result<(), std::io::Error> {
        write_file(path, self.pgn().as_str(), !overwrite)?;
        Ok(())
    }

    /// Resigns the [Game] for a player
    ///
    /// # Arguments
    /// * `color` - The [Color] of the player who resigns
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    /// use chess_lab::core::Color;
    /// let mut game = StandardChess::default();
    /// game.resign(Color::White);
    /// ```
    ///
    fn resign(&mut self, color: Color) {
        self.game.resign(color)
    }

    /// Sets the [Game] as a draw by agreement
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    /// let mut game = StandardChess::default();
    /// game.draw();
    /// ```
    ///
    fn draw(&mut self) {
        self.game.set_draw_by_agreement()
    }

    /// Sets the [Game] as lost in time for a player
    ///
    /// # Arguments
    /// * `color` - The [Color] of the player who lost in time
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    /// use chess_lab::core::Color;
    ///
    /// let mut game = StandardChess::default();
    /// game.set_lost_in_time(Color::Black);
    /// ```
    ///
    fn set_lost_in_time(&mut self, color: Color) {
        self.game.set_lost_in_time(color)
    }

    /// Returns the [Board] of the game
    ///
    /// # Returns
    /// A copy of the [Board] of the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    /// let game = StandardChess::default();
    /// let board = game.get_board();
    /// ```
    ///
    fn get_board(&self) -> Board {
        self.game.board.clone()
    }

    /// Returns whether it is white's turn to [Move]
    ///
    /// # Returns
    /// Whether it is white's turn to [Move]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    /// let game = StandardChess::default();
    /// let color = game.is_white_turn();
    /// ```
    ///
    fn is_white_turn(&self) -> bool {
        self.game.is_white_turn
    }

    /// Returns the halfmove clock of the [Game]
    ///
    /// # Returns
    /// The halfmove clock of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    ///
    /// let game = StandardChess::default();
    /// let halfmove_clock = game.get_halfmove_clock();
    /// ```
    ///
    fn get_halfmove_clock(&self) -> u32 {
        self.game.halfmove_clock
    }

    /// Returns the fullmove number of the [Game]
    ///
    /// # Returns
    /// The fullmove number of the [Game]
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Variant;
    /// use chess_lab::variants::StandardChess;
    ///
    /// let game = StandardChess::default();
    /// let fullmove_number = game.get_fullmove_number();
    /// ```
    ///
    fn get_fullmove_number(&self) -> u32 {
        self.game.fullmove_number
    }

    /// Returns the castling rights of the [Game]
    ///
    /// # Returns
    /// The castling rights of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    ///
    /// let game = StandardChess::default();
    /// let castling_rights = game.get_castling_rights();
    /// ```
    ///
    fn get_castling_rights(&self) -> String {
        let mut castling_rights = String::new();

        if self.game.castling_rights == 0 {
            castling_rights.push('-');
        } else {
            if self.game.castling_rights & 0b1000 != 0 {
                castling_rights.push('K');
            }
            if self.game.castling_rights & 0b0100 != 0 {
                castling_rights.push('Q');
            }
            if self.game.castling_rights & 0b0010 != 0 {
                castling_rights.push('k');
            }
            if self.game.castling_rights & 0b0001 != 0 {
                castling_rights.push('q');
            }
        }
        castling_rights
    }

    /// Returns the en passant square of the [Game]
    ///
    /// # Returns
    /// The en passant square of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    ///
    /// let game = StandardChess::default();
    /// let en_passant = game.get_en_passant();
    /// ```
    ///
    fn get_en_passant(&self) -> Option<Position> {
        self.game.en_passant
    }

    /// Returns the starting FEN of the [game]
    ///
    /// # Returns
    /// A copy of the starting FEN of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    ///
    /// let game = StandardChess::default();
    /// let starting_fen = game.get_starting_fen();
    /// ```
    ///
    fn get_starting_fen(&self) -> String {
        self.game.starting_fen.clone()
    }

    /// Returns the history of the [Game]
    ///
    /// # Returns
    /// A copy of the history of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    ///
    /// let game = StandardChess::default();
    /// let history = game.get_history();
    /// ```
    ///
    fn get_history(&self) -> PGNTree<Move> {
        self.game.history.clone()
    }

    /// Returns the previous positions of the [Game]
    ///
    /// # Returns
    /// A copy of the previous positions of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    ///
    /// let game = StandardChess::default();
    /// let prev_positions = game.get_prev_positions();
    /// ```
    ///
    fn get_prev_positions(&self) -> HashMap<String, u32> {
        self.game.prev_positions.clone()
    }

    /// Returns the status of the [Game]
    ///
    /// # Returns
    /// The status of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::{Variant, GameStatus};
    /// # use chess_lab::variants::StandardChess;
    ///
    /// let game = StandardChess::default();
    /// let status = game.get_status();
    /// ```
    ///
    fn get_status(&self) -> GameStatus {
        self.game.status
    }
}

// TODO: Implement tests for StandardChess variant
