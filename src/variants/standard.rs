use std::{fs::File, io::Write};

use crate::{
    constants::{Color, GameStatus, Variant},
    errors::MoveError,
    logic::Game,
};

pub struct ChessGame {
    game: Game,
}

impl ChessGame {
    pub fn new() -> ChessGame {
        ChessGame {
            game: Game::default(),
        }
    }
}

impl Variant for ChessGame {
    fn move_piece(&mut self, move_str: &str) -> Result<GameStatus, MoveError> {
        self.game.move_piece(move_str)
    }

    fn undo(&mut self) {
        self.game.undo()
    }

    fn redo(&mut self) {
        self.game.redo()
    }

    fn pgn(&self) -> String {
        self.game.pgn()
    }

    fn fen(&self) -> String {
        self.game.fen()
    }

    fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write_all(self.game.pgn().as_bytes())?;
        Ok(())
    }

    fn load(&mut self, path: &str) -> Result<(), std::io::Error> {
        todo!()
    }

    fn resign(&mut self, color: Color) {
        self.game.resign(color)
    }

    fn draw(&mut self) {
        self.game.set_draw_by_agreement()
    }

    fn set_lost_in_time(&mut self, color: Color) {
        self.game.set_lost_in_time(color)
    }
}
