use std::collections::HashMap;

use crate::{
    errors::{FenError, MoveError, PgnError},
    logic::{Board, Game},
};

use super::{Color, GameStatus, Move, PgnTree, Position};

/// A trait for a chess variant.
///
/// A chess variant is a game that is derived from chess, but has different rules.
///
/// Methods:
/// * `new` - creates a new instance of the variant.
/// * `from_fen` - creates a new instance of the variant from a FEN string.
/// * `from_pgn` - creates a new instance of the variant from a PGN string.
/// * `move_piece` - moves a piece on the board.
/// * `undo` - undoes the last move.
/// * `redo` - redoes the last move.
/// * `pgn` - returns the PGN string of the game.
/// * `fen` - returns the FEN string of the game.
/// * `save` - saves the game to a file.
/// * `load` - loads the game from a file.
/// * `resign` - resigns the game for a player.
/// * `draw` - offers a draw to the opponent.
/// * `set_lost_in_time` - sets a player as lost in time.
///
pub trait Variant {
    /// Moves a piece on the board.
    ///
    /// # Arguments
    /// * `move_str` - A move string in algebraic notation.
    ///
    /// # Returns
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
    fn set_lost_in_time(&mut self, color: Color);

    /// Returns the board of the game.
    ///
    /// # Returns
    /// A copy of the board of the game.
    ///
    fn get_board(&self) -> Board;

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

    /// Returns the history of the game.
    ///
    /// # Returns
    /// A cloned PGN tree object that stores the moves of the game.
    ///
    fn get_history(&self) -> PgnTree<Move>;

    /// Returns the previous positions of the game.
    ///
    /// # Returns
    /// A hashmap that stores the number of times a position has occurred.
    ///
    fn get_prev_positions(&self) -> HashMap<String, u32>;

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
    /// * `Ok(Self)` - A new instance of the variant.
    /// * `Err(PgnError)` - An error occurred while parsing the PGN string.
    ///
    fn from_pgn(pgn: &str) -> Result<Self, PgnError>;

    /// Loads the game from a file.
    ///
    /// # Arguments
    /// * `path` - The path to the file.
    ///
    /// # Returns
    /// * `Ok(Self)` - The game was loaded successfully.
    /// * `Err(PgnError)` - An error occurred while loading the game.
    ///
    fn load(path: &str) -> Result<Self, PgnError>;

    /// Loads multiple games from a file.
    ///
    /// # Arguments
    /// * `path` - The path to the file.
    ///
    /// # Returns
    /// * `Ok(Vec<Self>)` - The games were loaded successfully.
    /// * `Err(PgnError)` - An error occurred while loading the games.
    ///
    fn load_all(path: &str) -> Result<Vec<Self>, PgnError>;
}
