use crate::errors::MoveError;

use super::{Color, GameStatus};

pub trait Variant {
    fn new() -> Self;
    fn from_fen(fen: &str) -> Self;
    fn move_piece(&mut self, move_str: &str) -> Result<GameStatus, MoveError>;
    fn undo(&mut self);
    fn redo(&mut self);
    fn pgn(&self) -> String;
    fn fen(&self) -> String;
    fn save(&self, path: &str) -> Result<(), std::io::Error>;
    fn load(&mut self, path: &str) -> Result<(), std::io::Error>;
    fn resign(&mut self, color: Color);
    fn draw(&mut self);
    fn set_lost_in_time(&mut self, color: Color);
}
