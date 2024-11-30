use crate::{
    constants::{Color, GameStatus, Variant},
    errors::{FenError, MoveError, PgnError},
    logic::Game,
    utils::{
        os::{read_file, write_file},
        pest::pgn_parser::parse_standard_pgn,
    },
};

pub struct Chess960 {
    game: Game,
}

impl Variant for Chess960 {
    fn new() -> Self {
        Chess960 {
            game: Game::default(),
        }
    }

    fn from_fen(fen: &str) -> Result<Chess960, FenError> {
        Ok(Chess960 {
            game: Game::from_fen(fen)?,
        })
    }

    fn from_pgn(pgn: &str) -> Result<Chess960, PgnError> {
        Ok(Chess960 {
            game: parse_standard_pgn(pgn)?,
        })
    }

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
        write_file(path, self.pgn().as_str())?;
        Ok(())
    }

    fn load(&mut self, path: &str) -> Result<(), PgnError> {
        let pgn = read_file(path)?;
        self.game = Self::from_pgn(pgn.as_str())?.game;
        Ok(())
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
