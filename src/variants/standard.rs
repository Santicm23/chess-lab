use std::collections::HashMap;

use crate::{
    constants::{pgn::PgnTree, Color, GameStatus, Move, Position, Variant},
    errors::{FenError, MoveError, PgnError},
    logic::{Board, Game},
    utils::{
        os::{read_file, write_file},
        pest::pgn_parser::parse_standard_pgn,
    },
};

/// Standard Chess variant
/// This variant is the most common chess variant played worldwide.
/// It is played on an 8x8 board with the following pieces for each player:
/// - 8 Pawns
/// - 2 Rooks
/// - 2 Knights
/// - 2 Bishops
/// - 1 Queen
/// - 1 King
///
/// The game is won by checkmating the opponent's king.
/// The game is drawn by stalemate, threefold repetition, the fifty-move rule, or agreement.
/// The game is lost by resigning or losing on time.
///
/// # Attributes
/// * `game` - The game struct that holds the state of the game.
///
/// # Methods
/// * `new` - Creates a new instance of the StandardChess variant.
/// * `from_fen` - Creates a new instance of the StandardChess variant from a FEN string.
/// * `from_pgn` - Creates a new instance of the StandardChess variant from a PGN string.
/// * `move_piece` - Moves a piece on the board.
/// * `undo` - Undoes the last move.
/// * `redo` - Redoes the last undone move.
/// * `pgn` - Returns the PGN string of the game.
/// * `fen` - Returns the FEN string of the game.
/// * `save` - Saves the game to a PGN file.
/// * `load` - Loads the game from a PGN file.
/// * `resign` - Resigns the game for a player.
/// * `draw` - Sets the game as a draw by agreement.
/// * `set_lost_in_time` - Sets the game as lost in time for a player.
///
#[derive(Debug, Clone)]
pub struct StandardChess {
    game: Game,
}

impl Variant for StandardChess {
    /// Creates a new instance of the StandardChess variant.
    ///
    /// # Returns
    /// A new instance of the StandardChess variant.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::Variant;
    /// use chess_lab::variants::StandardChess;
    ///
    /// let game = StandardChess::new();
    /// ```
    ///
    fn new() -> StandardChess {
        StandardChess {
            game: Game::default(),
        }
    }

    /// Creates a new instance of the StandardChess variant from a FEN string.
    ///
    /// # Arguments
    /// * `fen` - A FEN string.
    ///
    /// # Returns
    /// * `Ok(StandardChess)` - A new instance of the StandardChess variant.
    /// * `Err(FenError)` - An error occurred while parsing the FEN string.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::Variant;
    /// use chess_lab::variants::StandardChess;
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let game = StandardChess::from_fen(fen).unwrap();
    /// ```
    ///
    fn from_fen(fen: &str) -> Result<StandardChess, FenError> {
        Ok(StandardChess {
            game: Game::from_fen(fen)?,
        })
    }

    /// Creates a new instance of the StandardChess variant from a PGN string.
    ///
    /// # Arguments
    /// * `pgn` - A PGN string.
    ///
    /// # Returns
    /// * `Ok(StandardChess)` - A new instance of the StandardChess variant.
    /// * `Err(PgnError)` - An error occurred while parsing the PGN string.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::Variant;
    /// use chess_lab::variants::StandardChess;
    ///
    /// let pgn = "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6";
    ///
    /// let game = StandardChess::from_pgn(pgn).unwrap();
    /// ```
    ///
    fn from_pgn(pgn: &str) -> Result<StandardChess, PgnError> {
        Ok(StandardChess {
            game: parse_standard_pgn(pgn)?,
        })
    }

    /// Loads the game from a file.
    ///
    /// # Arguments
    /// * `path` - The path to the file.
    ///
    /// # Returns
    /// * `Ok(StandardChess)` - The game was loaded successfully.
    /// * `Err(PgnError)` - An error occurred while loading the game.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::Variant;
    /// use chess_lab::variants::StandardChess;
    ///
    /// let path = "data/ex1.pgn";
    /// let game = StandardChess::load(path).unwrap();
    /// ```
    ///
    fn load(path: &str) -> Result<StandardChess, PgnError> {
        let pgn = read_file(path)?;
        Ok(StandardChess {
            game: Self::from_pgn(pgn.as_str())?.game,
        })
    }

    /// Moves a piece on the board.
    ///
    /// # Arguments
    /// * `move_str` - A move string in algebraic notation.
    ///
    /// # Returns
    /// * `Ok(GameStatus)` - The status of the game after the move.
    /// * `Err(MoveError)` - An error occurred while moving the piece.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::{Variant, GameStatus};
    /// use chess_lab::variants::StandardChess;
    ///
    /// let mut game = StandardChess::new();
    /// let status = game.move_piece("e4");
    ///
    /// assert_eq!(status, Ok(GameStatus::InProgress));
    /// ```
    ///
    fn move_piece(&mut self, move_str: &str) -> Result<GameStatus, MoveError> {
        self.game.move_piece(move_str)
    }

    /// Undoes the last move.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::Variant;
    /// use chess_lab::variants::StandardChess;
    ///
    /// let mut game = StandardChess::new();
    /// game.move_piece("e4").unwrap();
    /// game.undo();
    ///
    /// assert_eq!(game.fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    fn undo(&mut self) {
        self.game.undo()
    }

    /// Redoes the last undone move.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::Variant;
    /// use chess_lab::variants::StandardChess;
    ///
    /// let mut game = StandardChess::new();
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

    /// Returns the PGN string of the game.
    ///
    /// # Returns
    /// The PGN string of the game.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::Variant;
    /// use chess_lab::variants::StandardChess;
    ///
    /// let mut game = StandardChess::new();
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

    /// Returns the FEN string of the game.
    ///
    /// # Returns
    /// The FEN string of the game.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::Variant;
    /// use chess_lab::variants::StandardChess;
    ///
    /// let game = StandardChess::new();
    /// let fen = game.fen();
    ///
    /// assert_eq!(fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    fn fen(&self) -> String {
        self.game.fen()
    }

    /// Saves the game to a file.
    ///
    /// # Arguments
    /// * `path` - The path to the file.
    ///
    /// # Returns
    /// * `Ok(())` - The game was saved successfully.
    /// * `Err(std::io::Error)` - An error occurred while saving the game.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::Variant;
    /// use chess_lab::variants::StandardChess;
    ///
    /// let game = StandardChess::new();
    /// let path = "data/ex.pgn";
    /// game.save(path).unwrap();
    /// ```
    ///
    fn save(&self, path: &str) -> Result<(), std::io::Error> {
        write_file(path, self.pgn().as_str())?;
        Ok(())
    }

    /// Resigns the game for a player.
    ///
    /// # Arguments
    /// * `color` - The color of the player who resigns.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::{Variant, Color};
    /// use chess_lab::variants::StandardChess;
    ///
    /// let mut game = StandardChess::new();
    /// game.resign(Color::White);
    /// ```
    ///
    fn resign(&mut self, color: Color) {
        self.game.resign(color)
    }

    /// Sets the game as a draw by agreement.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::Variant;
    /// use chess_lab::variants::StandardChess;
    ///
    /// let mut game = StandardChess::new();
    /// game.draw();
    /// ```
    ///
    fn draw(&mut self) {
        self.game.set_draw_by_agreement()
    }

    /// Sets the game as lost in time for a player.
    ///
    /// # Arguments
    /// * `color` - The color of the player who lost in time.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::{Variant, Color};
    /// use chess_lab::variants::StandardChess;
    ///
    /// let mut game = StandardChess::new();
    /// game.set_lost_in_time(Color::Black);
    /// ```
    ///
    fn set_lost_in_time(&mut self, color: Color) {
        self.game.set_lost_in_time(color)
    }

    /// Returns the board of the game.
    ///
    /// # Returns
    /// A copy of the board of the game.
    ///
    fn get_board(&self) -> Board {
        self.game.board.clone()
    }

    /// Returns the color of the player whose turn it is.
    ///
    /// # Returns
    /// The color of the player whose turn it is.
    ///
    fn is_white_turn(&self) -> bool {
        self.game.is_white_turn
    }

    /// Returns the halfmove clock of the game.
    ///
    /// # Returns
    /// The halfmove clock of the game.
    ///
    fn get_halfmove_clock(&self) -> u32 {
        self.game.halfmove_clock
    }

    /// Returns the fullmove number of the game.
    ///
    /// # Returns
    /// The fullmove number of the game.
    ///
    fn get_fullmove_number(&self) -> u32 {
        self.game.fullmove_number
    }

    /// Returns the castling rights of the game.
    ///
    /// # Returns
    /// The castling rights of the game.
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

    /// Returns the en passant square of the game.
    ///
    /// # Returns
    /// The en passant square of the game.
    ///
    fn get_en_passant(&self) -> Option<Position> {
        self.game.en_passant
    }

    /// Returns the starting FEN of the game.
    ///
    /// # Returns
    /// A copy of the starting FEN of the game.
    ///
    fn get_starting_fen(&self) -> String {
        self.game.starting_fen.clone()
    }

    /// Returns the history of the game.
    ///
    /// # Returns
    /// A copy of the history of the game.
    ///
    fn get_history(&self) -> PgnTree<Move> {
        self.game.history.clone()
    }

    /// Returns the previous positions of the game.
    ///
    /// # Returns
    /// A copy of the previous positions of the game.
    ///
    fn get_prev_positions(&self) -> HashMap<String, u32> {
        self.game.prev_positions.clone()
    }

    /// Returns the status of the game.
    ///
    /// # Returns
    /// The status of the game.
    ///
    fn get_status(&self) -> GameStatus {
        self.game.status
    }
}
