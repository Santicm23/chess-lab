use crate::{
    errors::{FenError, MoveError, PGNError},
    logic::Game,
};

use super::{Color, GameStatus, Position};

/// A trait for a chess variant.
///
/// A chess variant is a game that is derived from chess, but has different rules.
///
pub trait Variant {
    /// Moves a piece on the board.
    ///
    /// # Arguments
    /// * `move_str` - A move string in algebraic notation.
    ///
    /// # Returns
    /// A `Result<GameStatus, MoveError>` object
    /// * `Ok(GameStatus)` - The status of the game after the move.
    /// * `Err(MoveError)` - An error occurred while moving the piece.
    ///
    fn move_piece(&mut self, move_str: &str) -> Result<GameStatus, MoveError>;

    /// Undoes the last move.
    ///
    fn undo(&mut self);

    /// Redoes the last undone move.
    ///
    fn redo(&mut self);

    /// Returns the PGN string of the game.
    ///
    /// # Returns
    /// The PGN string of the game.
    ///
    fn pgn(&self) -> String;

    /// Returns the FEN string of the game.
    ///
    /// # Returns
    /// The FEN string of the game.
    ///
    fn fen(&self) -> String;

    /// Saves the game to a file.
    ///
    /// # Arguments
    /// * `path` - The path to the file.
    /// * `overwrite` - Whether to overwrite the file if it already exists.
    ///
    /// # Returns
    /// A `Result<(), std::io::Error>` object
    /// * `Ok(())` - The game was saved successfully.
    /// * `Err(std::io::Error)` - An error occurred while saving the game.
    ///
    fn save(&self, path: &str, overwrite: bool) -> Result<(), std::io::Error>;

    /// Resigns the game for a player.
    ///
    /// # Arguments
    /// * `color` - The color of the player that is resigning.
    ///
    fn resign(&mut self, color: Color);

    /// Sets the game as a draw by agreement.
    ///
    fn draw(&mut self);

    /// Sets the game as lost in time for a player.
    ///
    /// # Arguments
    /// * `color` - The color of the player that lost in time.
    ///
    fn lost_on_time(&mut self, color: Color);

    /// Gets the minified fen of the game.
    ///
    /// # Returns
    /// The minified fen of the game.
    ///
    fn get_minified_fen(&self) -> String;

    /// Returns whether it is white's turn to move.
    ///
    /// # Returns
    /// Whether it is white's turn to move.
    ///
    fn is_white_turn(&self) -> bool;

    /// Returns the halfmove clock of the game.
    ///
    /// # Returns
    /// The halfmove clock of the game.
    ///
    fn get_halfmove_clock(&self) -> u32;

    /// Returns the fullmove number of the game.
    ///
    /// # Returns
    /// The fullmove number of the game.
    ///
    fn get_fullmove_number(&self) -> u32;

    /// Returns the castling rights of the game.
    ///
    /// # Returns
    /// The castling rights of the game.
    ///
    fn get_castling_rights(&self) -> String;

    /// Returns the en passant square of the game.
    ///
    /// # Returns
    /// The en passant square of the game.
    ///
    fn get_en_passant(&self) -> Option<Position>;

    /// Returns the starting FEN of the game.
    ///
    /// # Returns
    /// The starting FEN of the game.
    ///
    fn get_starting_fen(&self) -> String;

    /// Returns the status of the game.
    ///
    /// # Returns
    /// The status of the game.
    ///
    fn get_status(&self) -> GameStatus;
}

pub trait VariantBuilder: Sized + Default {
    /// The name of the variant.
    ///
    /// # Returns
    /// The name of the variant.
    ///
    fn name() -> &'static str;

    /// Creates a new instance of the variant.
    ///
    /// # Returns
    /// A new instance of the variant.
    ///
    fn new(game: Game) -> Self;

    /// Creates a new instance of the variant from a FEN string.
    ///
    /// # Arguments
    /// * `fen` - A FEN string.
    ///
    /// # Returns
    /// A `Result<Self, FenError>` object
    /// * `Ok(Self)` - A new instance of the variant.
    /// * `Err(FenError)` - An error occurred while parsing the FEN string.
    ///
    fn from_fen(fen: &str) -> Result<Self, FenError>;

    /// Creates a new instance of the variant from a PGN string.
    ///
    /// # Arguments
    /// * `pgn` - A PGN string.
    ///
    /// # Returns
    /// A `Result<Self, PgnError>` object
    /// * `Ok(Self)` - A new instance of the variant.
    /// * `Err(PgnError)` - An error occurred while parsing the PGN string.
    ///
    fn from_pgn(pgn: &str) -> Result<Self, PGNError>;

    /// Loads the game from a file.
    ///
    /// # Arguments
    /// * `path` - The path to the file.
    ///
    /// # Returns
    /// A `Result<Self, PgnError>` object
    /// * `Ok(Self)` - The game was loaded successfully.
    /// * `Err(PgnError)` - An error occurred while loading the game.
    ///
    fn load(path: &str) -> Result<Self, PGNError>;

    /// Loads multiple games from a file.
    ///
    /// # Arguments
    /// * `path` - The path to the file.
    ///
    /// # Returns
    /// A `Result<Vec<Self>, PgnError>` object
    /// * `Ok(Vec<Self>)` - The games were loaded successfully.
    /// * `Err(PgnError)` - An error occurred while loading the games.
    ///
    fn load_all(path: &str) -> Result<Vec<Self>, PGNError>;
}
