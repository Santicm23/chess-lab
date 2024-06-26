use crate::errors::MoveError;

use super::GameStatus;

pub trait Variant {
    fn move_piece(move_str: &str) -> Result<GameStatus, MoveError>;
    fn undo();
    fn redo();
    fn pgn();
    fn fen();
}
