use crate::errors::{FenError, MoveError, PgnError};

use super::{Color, GameStatus};

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
pub trait Variant: Sized {
    fn new() -> Self;
    fn from_fen(fen: &str) -> Result<Self, FenError>;
    fn from_pgn(pgn: &str) -> Result<Self, PgnError>;
    fn move_piece(&mut self, move_str: &str) -> Result<GameStatus, MoveError>;
    fn undo(&mut self);
    fn redo(&mut self);
    fn pgn(&self) -> String;
    fn fen(&self) -> String;
    fn save(&self, path: &str) -> Result<(), std::io::Error>;
    fn load(&mut self, path: &str) -> Result<(), PgnError>;
    fn resign(&mut self, color: Color);
    fn draw(&mut self);
    fn set_lost_in_time(&mut self, color: Color);
}
