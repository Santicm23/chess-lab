use crate::{
    constants::{Color, GameStatus, Variant, VariantBuilder},
    errors::{FenError, MoveError, PgnError},
    logic::{Board, Game},
    utils::{
        os::{read_file, write_file},
        pest::pgn_parser::{parse_pgn, parse_pgn_file},
    },
};

#[derive(Debug, Clone)]
pub struct Chess960 {
    game: Game,
}

impl Default for Chess960 {
    fn default() -> Chess960 {
        Chess960 {
            game: Game::default(),
        }
    }
}

impl VariantBuilder for Chess960 {
    fn name() -> &'static str {
        "Chess960"
    }

    fn new(game: Game) -> Chess960 {
        Chess960 { game }
    }

    fn from_fen(fen: &str) -> Result<Chess960, FenError> {
        Ok(Chess960 {
            game: Game::from_fen(fen)?,
        })
    }

    fn from_pgn(pgn: &str) -> Result<Chess960, PgnError> {
        parse_pgn(pgn)
    }

    fn load(path: &str) -> Result<Chess960, PgnError> {
        let pgn = read_file(path)?;
        Chess960::from_pgn(&pgn)
    }

    fn load_all(path: &str) -> Result<Vec<Chess960>, PgnError> {
        let pgn = read_file(path)?;
        parse_pgn_file(&pgn)
    }
}

impl Variant for Chess960 {
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

    fn save(&self, path: &str, overwrite: bool) -> Result<(), std::io::Error> {
        write_file(path, self.pgn().as_str(), !overwrite)?;
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

    fn get_board(&self) -> Board {
        todo!()
    }

    fn is_white_turn(&self) -> bool {
        todo!()
    }

    fn get_halfmove_clock(&self) -> u32 {
        todo!()
    }

    fn get_fullmove_number(&self) -> u32 {
        todo!()
    }

    fn get_castling_rights(&self) -> String {
        todo!()
    }

    fn get_en_passant(&self) -> Option<crate::constants::Position> {
        todo!()
    }

    fn get_starting_fen(&self) -> String {
        todo!()
    }

    fn get_history(&self) -> crate::constants::pgn::PgnTree<crate::constants::Move> {
        todo!()
    }

    fn get_prev_positions(&self) -> std::collections::HashMap<String, u32> {
        todo!()
    }

    fn get_status(&self) -> GameStatus {
        todo!()
    }
}
