use rand::Rng;

use crate::{
    constants::{Color, GameStatus, Variant, VariantBuilder},
    errors::{FenError, MoveError, PgnError},
    logic::{Board, Game},
    utils::{
        os::{read_file, write_file},
        pest::pgn_parser::{parse_pgn, parse_pgn_file},
    },
};

/// Chess960 variant
///
/// Chess960 is a variant of chess that uses the same rules as standard chess, but the starting position of the pieces is randomized.
///
/// # Attributes
/// * `game` - The game struct that contains the current state of the game
///
#[derive(Debug, Clone)]
pub struct Chess960 {
    game: Game,
}

impl Default for Chess960 {
    /// Generates a random starting position for the pieces
    ///
    /// # Returns
    /// A Chess960 struct with a random starting position
    ///
    /// # Example
    /// ```
    /// use chess_lab::variants::Chess960;
    ///
    /// let variant = Chess960::default();
    /// ```
    ///
    fn default() -> Chess960 {
        let mut first_row = String::new();
        let mut remaining_pieces = vec!['r', 'n', 'b', 'q', 'b', 'n', 'r'];
        let mut last_piece = ' ';

        let mut rng = rand::thread_rng();
        while last_piece != 'r' {
            let index = rng.gen_range(0..remaining_pieces.len());
            let piece = remaining_pieces.remove(index);
            first_row.push(piece);
            last_piece = piece;
        }

        remaining_pieces = remaining_pieces.into_iter().filter(|c| *c != 'r').collect();

        remaining_pieces.push('k');

        while last_piece != 'k' {
            let index = rng.gen_range(0..remaining_pieces.len());
            let piece = remaining_pieces.remove(index);
            first_row.push(piece);
            last_piece = piece;
        }

        remaining_pieces.push('r');

        while !remaining_pieces.is_empty() {
            let index = rng.gen_range(0..remaining_pieces.len());
            let piece = remaining_pieces.remove(index);
            first_row.push(piece);
        }

        let mut game = Game::from_fen(&format!(
            "{}/pppppppp/8/8/8/8/PPPPPPPP/{} w - - 0 1",
            first_row,
            first_row.to_uppercase()
        ))
        .unwrap();

        game.history.variant = Some("Chess960".to_string());

        Chess960 { game }
    }
}

impl VariantBuilder for Chess960 {
    /// Returns the name of the variant
    ///
    /// # Returns
    /// A string with the name of the variant
    ///
    /// # Example
    /// ```
    /// use chess_lab::constants::VariantBuilder;
    /// use chess_lab::variants::Chess960;
    ///
    /// let name = Chess960::name();
    /// ```
    ///
    fn name() -> &'static str {
        "Chess960"
    }

    /// Returns a new instance of the variant from a game struct
    ///
    /// # Arguments
    /// * `game` - The game struct that contains the current state of the game
    ///
    /// # Returns
    /// A Chess960 struct with the game state
    ///
    /// # Example
    /// ```
    /// use chess_lab::constants::VariantBuilder;
    /// use chess_lab::logic::Game;
    /// use chess_lab::variants::Chess960;
    ///
    /// let game = Game::default();
    /// let variant = Chess960::new(game);
    /// ```
    ///
    fn new(game: Game) -> Chess960 {
        Chess960 { game }
    }

    /// Returns a new instance of the variant from a FEN string
    ///
    /// # Arguments
    /// * `fen` - The FEN string that represents the game state
    ///
    /// # Returns
    /// * `Ok(Chess960)` - A Chess960 struct with the game state
    /// * `Err(FenError)` - An error that indicates that the FEN string is invalid
    ///
    /// # Example
    /// ```
    /// use chess_lab::constants::VariantBuilder;
    /// use chess_lab::variants::Chess960;
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let variant = Chess960::from_fen(fen).unwrap();
    /// ```
    ///
    fn from_fen(fen: &str) -> Result<Chess960, FenError> {
        Ok(Chess960 {
            game: Game::from_fen(fen)?,
        })
    }

    /// Returns a new instance of the variant from a PGN string
    ///
    /// # Arguments
    /// * `pgn` - The PGN string that represents the game state
    ///
    /// # Returns
    /// * `Ok(Chess960)` - A Chess960 struct with the game state
    /// * `Err(PgnError)` - An error that indicates that the PGN string is invalid
    ///
    /// # Example
    /// TODO
    ///
    fn from_pgn(pgn: &str) -> Result<Chess960, PgnError> {
        parse_pgn(pgn)
    }

    /// Loads a new instance of the variant from a PGN file
    ///
    /// # Arguments
    /// * `path` - The path to the PGN file
    ///
    /// # Returns
    /// * `Ok(Chess960)` - A Chess960 struct with the game state
    /// * `Err(PgnError)` - An error that indicates that the PGN file is invalid
    ///
    /// # Example
    /// TODO
    ///
    fn load(path: &str) -> Result<Chess960, PgnError> {
        let pgn = read_file(path)?;
        Chess960::from_pgn(&pgn)
    }

    /// Loads all the instances of the variant from a PGN file
    ///
    /// # Arguments
    /// * `path` - The path to the PGN file
    ///
    /// # Returns
    /// * `Ok(Vec<Chess960>)` - A vector with all the Chess960 structs with the game state
    /// * `Err(PgnError)` - An error that indicates that the PGN file is invalid
    ///
    /// # Example
    /// TODO
    ///
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
