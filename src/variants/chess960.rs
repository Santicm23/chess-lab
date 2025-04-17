use std::collections::HashMap;

use rand::Rng;

use crate::{
    core::{Color, GameStatus, Move, PgnTree, Position, Variant, VariantBuilder},
    errors::{FenError, MoveError, PgnError},
    logic::{Board, Game},
    parsing::pgn::{parse_pgn, parse_pgn_file},
    utils::os::{read_file, write_file},
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
    /// use chess_lab::core::VariantBuilder;
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
    /// use chess_lab::core::VariantBuilder;
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
    /// use chess_lab::core::VariantBuilder;
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
    /// ```
    /// use chess_lab::core::VariantBuilder;
    /// use chess_lab::variants::Chess960;
    ///
    /// let pgn = "1. e4 e5 2. Nf3 Nc6";
    /// let variant = Chess960::from_pgn(pgn).unwrap();
    /// ```
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
    /// ```
    /// use chess_lab::core::VariantBuilder;
    /// use chess_lab::variants::Chess960;
    ///
    /// let path = "data/standard/ex.pgn"; // TODO: Change to the chess960 file
    /// let variant = Chess960::load(path).unwrap();
    /// ```
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
    /// ```
    /// use chess_lab::core::VariantBuilder;
    /// use chess_lab::variants::Chess960;
    ///
    /// let path = "data/standard/ex3.pgn"; // TODO: Change to the chess960 file
    /// let variants = Chess960::load_all(path).unwrap();
    /// ```
    ///
    fn load_all(path: &str) -> Result<Vec<Chess960>, PgnError> {
        let pgn = read_file(path)?;
        parse_pgn_file(&pgn)
    }
}

impl Variant for Chess960 {
    /// Moves a piece on the board
    ///
    /// # Arguments
    /// * `move_str` - The move string that represents the move to be made
    ///
    /// # Returns
    /// * `Ok(GameStatus)` - The status of the game after the move
    /// * `Err(MoveError)` - An error that indicates that the move is invalid
    ///
    /// # Example
    /// ```
    /// use chess_lab::core::{Variant, VariantBuilder};
    /// use chess_lab::variants::Chess960;
    ///
    /// let mut variant = Chess960::default();
    /// variant.move_piece("e4").unwrap();
    /// ```
    ///
    fn move_piece(&mut self, move_str: &str) -> Result<GameStatus, MoveError> {
        self.game.move_piece(move_str)
    }

    /// Undoes the last move made
    ///
    /// # Example
    /// ```
    /// use chess_lab::core::{Variant, VariantBuilder};
    /// use chess_lab::variants::Chess960;
    ///
    /// let mut variant = Chess960::default();
    /// variant.move_piece("e4").unwrap();
    /// variant.undo();
    /// ```
    ///
    fn undo(&mut self) {
        self.game.undo()
    }

    /// Redoes the last move that was undone
    ///
    /// # Example
    /// ```
    /// use chess_lab::core::{Variant, VariantBuilder};
    /// use chess_lab::variants::Chess960;
    ///
    /// let mut variant = Chess960::default();
    /// variant.move_piece("e4").unwrap();
    /// variant.undo();
    /// variant.redo();
    /// ```
    ///
    fn redo(&mut self) {
        self.game.redo()
    }

    /// Returns the PGN string of the game
    ///
    /// # Returns
    /// A string with the PGN of the game
    ///
    /// # Example
    /// ```
    /// use chess_lab::core::{Variant, VariantBuilder};
    /// use chess_lab::variants::Chess960;
    ///
    /// let variant = Chess960::default();
    /// let pgn = variant.pgn();
    /// ```
    ///
    fn pgn(&self) -> String {
        self.game.pgn()
    }

    /// Returns the FEN string of the game
    ///
    /// # Returns
    /// A string with the FEN of the game
    ///
    /// # Example
    /// ```
    /// use chess_lab::core::{Variant, VariantBuilder};
    /// use chess_lab::variants::Chess960;
    ///
    /// let variant = Chess960::default();
    /// let fen = variant.fen();
    /// ```
    ///
    fn fen(&self) -> String {
        self.game.fen()
    }

    /// Saves the PGN string of the game to a file
    ///
    /// # Arguments
    /// * `path` - The path to the file
    /// * `overwrite` - A boolean that indicates if the file should be overwritten
    ///
    /// # Returns
    /// * `Ok(())` - The PGN was saved successfully
    /// * `Err(std::io::Error)` - An error that indicates that the PGN could not be saved
    ///
    /// # Example
    /// ```
    /// use chess_lab::core::{Variant, VariantBuilder};
    /// use chess_lab::variants::Chess960;
    ///
    /// let variant = Chess960::default();
    /// variant.save("data/chess960/ex.pgn", true).unwrap();
    /// ```
    ///
    fn save(&self, path: &str, overwrite: bool) -> Result<(), std::io::Error> {
        write_file(path, self.pgn().as_str(), !overwrite)?;
        Ok(())
    }

    /// Resigns the game for a color
    ///
    /// # Arguments
    /// * `color` - The color that resigns
    ///
    /// # Example
    /// ```
    /// use chess_lab::core::{Variant, VariantBuilder, Color};
    /// use chess_lab::variants::Chess960;
    ///
    /// let mut variant = Chess960::default();
    /// variant.resign(Color::White);
    /// ```
    ///
    fn resign(&mut self, color: Color) {
        self.game.resign(color)
    }

    /// Sets the game as a draw
    ///
    /// # Example
    /// ```
    /// use chess_lab::core::{Variant, VariantBuilder};
    /// use chess_lab::variants::Chess960;
    ///
    /// let mut variant = Chess960::default();
    /// variant.draw();
    /// ```
    ///
    fn draw(&mut self) {
        self.game.set_draw_by_agreement()
    }

    /// Sets the game lost in time for a color
    ///
    /// # Arguments
    /// * `color` - The color that lost in time
    ///
    /// # Example
    /// ```
    /// use chess_lab::core::{Variant, VariantBuilder, Color};
    /// use chess_lab::variants::Chess960;
    ///
    /// let mut variant = Chess960::default();
    /// variant.set_lost_in_time(Color::White);
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
    /// # Examples
    /// ```
    /// use chess_lab::core::Variant;
    /// use chess_lab::variants::Chess960;
    ///
    /// let game = Chess960::default();
    /// let board = game.get_board();
    /// ```
    ///
    fn get_board(&self) -> Board {
        self.game.board.clone()
    }

    /// Returns whether it is white's turn to move.
    ///
    /// # Returns
    /// Whether it is white's turn to move.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Variant;
    /// use chess_lab::variants::Chess960;
    ///
    /// let game = Chess960::default();
    /// let color = game.is_white_turn();
    /// ```
    ///
    fn is_white_turn(&self) -> bool {
        self.game.is_white_turn
    }

    /// Returns the halfmove clock of the game.
    ///
    /// # Returns
    /// The halfmove clock of the game.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Variant;
    /// use chess_lab::variants::Chess960;
    ///
    /// let game = Chess960::default();
    /// let halfmove_clock = game.get_halfmove_clock();
    /// ```
    ///
    fn get_halfmove_clock(&self) -> u32 {
        self.game.halfmove_clock
    }

    /// Returns the fullmove number of the game.
    ///
    /// # Returns
    /// The fullmove number of the game.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Variant;
    /// use chess_lab::variants::Chess960;
    ///
    /// let game = Chess960::default();
    /// let fullmove_number = game.get_fullmove_number();
    /// ```
    ///
    fn get_fullmove_number(&self) -> u32 {
        self.game.fullmove_number
    }

    /// Returns the castling rights of the game.
    ///
    /// # Returns
    /// The castling rights of the game.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Variant;
    /// use chess_lab::variants::Chess960;
    ///
    /// let game = Chess960::default();
    /// let castling_rights = game.get_castling_rights();
    /// ```
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
    /// # Examples
    /// ```
    /// use chess_lab::core::Variant;
    /// use chess_lab::variants::Chess960;
    ///
    /// let game = Chess960::default();
    /// let en_passant = game.get_en_passant();
    /// ```
    ///
    fn get_en_passant(&self) -> Option<Position> {
        self.game.en_passant
    }

    /// Returns the starting FEN of the game.
    ///
    /// # Returns
    /// A copy of the starting FEN of the game.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Variant;
    /// use chess_lab::variants::Chess960;
    ///
    /// let game = Chess960::default();
    /// let starting_fen = game.get_starting_fen();
    /// ```
    ///
    fn get_starting_fen(&self) -> String {
        self.game.starting_fen.clone()
    }

    /// Returns the history of the game.
    ///
    /// # Returns
    /// A copy of the history of the game.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Variant;
    /// use chess_lab::variants::Chess960;
    ///
    /// let game = Chess960::default();
    /// let history = game.get_history();
    /// ```
    ///
    fn get_history(&self) -> PgnTree<Move> {
        self.game.history.clone()
    }

    /// Returns the previous positions of the game.
    ///
    /// # Returns
    /// A copy of the previous positions of the game.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::Variant;
    /// use chess_lab::variants::Chess960;
    ///
    /// let game = Chess960::default();
    /// let prev_positions = game.get_prev_positions();
    /// ```
    ///
    fn get_prev_positions(&self) -> HashMap<String, u32> {
        self.game.prev_positions.clone()
    }

    /// Returns the status of the game.
    ///
    /// # Returns
    /// The status of the game.
    ///
    /// # Examples
    /// ```
    /// use chess_lab::core::{Variant, GameStatus};
    /// use chess_lab::variants::Chess960;
    ///
    /// let game = Chess960::default();
    /// let status = game.get_status();
    /// ```
    ///
    fn get_status(&self) -> GameStatus {
        self.game.status
    }
}

// TODO: add unit tests
