use rand::Rng;

use crate::{
    core::{Color, GameStatus, Move, Piece, Position, Variant, VariantBuilder},
    errors::{FenError, MoveError, PGNError},
    logic::Game,
    parsing::{
        fen::get_minified_fen,
        pgn::{parse_multiple_pgn, parse_pgn},
    },
    utils::os::{read_file, write_file},
};

/// Chess960 is a variant of chess that uses the same rules as [standard chess](crate::variants::StandardChess), but the starting position of the pieces is randomized.
///
/// # Attributes
/// * `game` - The [Game] struct that contains the current state of the game
///
#[derive(Debug, Clone)]
pub struct Chess960 {
    /// The [Game] struct that contains the current state of the game
    game: Game,
}

impl Default for Chess960 {
    /// Generates a random starting position for the pieces
    ///
    /// # Returns
    /// A [Chess960] struct with a random starting position
    ///
    /// # Example
    /// ```
    /// # use chess_lab::variants::Chess960;
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
            "{}/pppppppp/8/8/8/8/PPPPPPPP/{} w KQkq - 0 1",
            first_row,
            first_row.to_uppercase()
        ))
        .unwrap();

        game.history.variant = Some("Chess960".to_string());

        Chess960 { game }
    }
}

impl VariantBuilder for Chess960 {
    /// Returns the name of the [Variant]
    ///
    /// # Returns
    /// A string with the name of the [Variant]
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::Chess960;
    /// let name = Chess960::name();
    /// assert_eq!(name, "Chess960");
    /// ```
    ///
    fn name() -> &'static str {
        "Chess960"
    }

    /// Returns a new instance of the variant from a [Game] struct
    ///
    /// # Arguments
    /// * `game` - The [Game] struct that contains the current state of the game
    ///
    /// # Returns
    /// A Chess960 struct with the game state
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::Chess960;
    /// use chess_lab::logic::Game;
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
    /// A `Result<Chess960, FenError>` object
    /// * `Ok(Chess960)` - A [Chess960] struct with the game state
    /// * `Err(FenError)` - An error that indicates that the FEN string is invalid
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::Chess960;
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
    /// A `Result<Chess960, PgnError>` object
    /// * `Ok(Chess960)` - A [Chess960] struct with the game state
    /// * `Err(PgnError)` - An error that indicates that the PGN string is invalid
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::Chess960;
    /// let pgn = "[Variant \"Chess960\"]\n1. e4 e5 2. Nf3 Nc6";
    /// let variant = Chess960::from_pgn(pgn).unwrap();
    /// ```
    ///
    fn from_pgn(pgn: &str) -> Result<Chess960, PGNError> {
        parse_pgn(pgn)
    }

    /// Loads a new instance of the variant from a PGN file
    ///
    /// # Arguments
    /// * `path` - The path to the PGN file
    ///
    /// # Returns
    /// A `Result<Chess960, PgnError>` object
    /// * `Ok(Chess960)` - A Chess960 struct with the game state
    /// * `Err(PgnError)` - An error that indicates that the PGN file is invalid
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::Chess960;
    /// let path = "data/chess960/ex1.pgn"; // TODO: Change to the chess960 file
    /// let variant = Chess960::load(path).unwrap();
    /// ```
    ///
    fn load(path: &str) -> Result<Chess960, PGNError> {
        let pgn = read_file(path)?;
        Chess960::from_pgn(&pgn)
    }

    /// Loads all the instances of the variant from a PGN file
    ///
    /// # Arguments
    /// * `path` - The path to the PGN file
    ///
    /// # Returns
    /// A `Result<Vec<Chess960>, PgnError>` object
    /// * `Ok(Vec<Chess960>)` - A vector with all the Chess960 structs with the game state
    /// * `Err(PgnError)` - An error that indicates that the PGN file is invalid
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::Chess960;
    /// let path = "data/chess960/ex2.pgn";
    /// let variants = Chess960::load_all(path).unwrap();
    /// ```
    ///
    fn load_all(path: &str) -> Result<Vec<Chess960>, PGNError> {
        let pgn = read_file(path)?;
        parse_multiple_pgn(&pgn)
    }
}

impl Variant for Chess960 {
    /// Moves a piece on the board
    ///
    /// # Arguments
    /// * `move_str` - The move string that represents the move to be made
    ///
    /// # Returns
    /// A `Result<GameStatus, MoveError>` object
    /// * `Ok(GameStatus)` - The status of the game after the move
    /// * `Err(MoveError)` - An error that indicates that the move is invalid
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    /// let mut variant = Chess960::default();
    /// variant.move_piece("e4").unwrap();
    /// ```
    ///
    fn move_piece(&mut self, move_str: &str) -> Result<GameStatus, MoveError> {
        self.game.move_piece(move_str)
    }

    /// Undoes the last [Move] made
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    /// let mut variant = Chess960::default();
    /// variant.move_piece("e4").unwrap();
    /// variant.undo();
    /// ```
    ///
    fn undo(&mut self) {
        self.game.undo()
    }

    /// Redoes the last [Move] that was undone
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
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
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    /// let variant = Chess960::default();
    /// let pgn = variant.pgn();
    /// ```
    ///
    fn pgn(&self) -> String {
        self.game.pgn()
    }

    /// Returns the FEN string of the [Game]
    ///
    /// # Returns
    /// A string with the FEN of the [Game]
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    ///
    /// let variant = Chess960::default();
    /// let fen = variant.fen();
    /// ```
    ///
    fn fen(&self) -> String {
        self.game.fen()
    }

    fn get_piece_at(&self, pos: Position) -> Option<Piece> {
        self.game.get_piece_at(pos)
    }

    fn get_legal_moves(&self, pos: Position) -> Vec<Move> {
        self.game.get_legal_moves(pos)
    }

    /// Saves the PGN string of the [Game] to a file
    ///
    /// # Arguments
    /// * `path` - The path to the file
    /// * `overwrite` - A boolean that indicates if the file should be overwritten
    ///
    /// # Returns
    /// A `Result<(), std::io::Error>` object
    /// * `Ok(())` - The PGN was saved successfully
    /// * `Err(std::io::Error)` - An error that indicates that the PGN could not be saved
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    ///
    /// let variant = Chess960::default();
    /// variant.save("data/chess960/ex.pgn", true).unwrap();
    /// # std::fs::remove_file("data/chess960/ex.pgn").unwrap();
    /// ```
    ///
    fn save(&self, path: &str, overwrite: bool) -> Result<(), std::io::Error> {
        write_file(path, self.pgn().as_str(), !overwrite)?;
        Ok(())
    }

    /// Resigns the [Game] for a [Color]
    ///
    /// # Arguments
    /// * `color` - The color that resigns
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    /// use chess_lab::core::Color;
    ///
    /// let mut variant = Chess960::default();
    /// variant.resign(Color::White);
    /// ```
    ///
    fn resign(&mut self, color: Color) {
        self.game.resign(color)
    }

    /// Sets the [Game] as a draw
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    /// let mut variant = Chess960::default();
    /// variant.draw();
    /// ```
    ///
    fn draw(&mut self) {
        self.game.draw_by_agreement()
    }

    /// Sets the game lost in time for a [Color]
    ///
    /// # Arguments
    /// * `color` - The [Color] that lost in time
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    /// use chess_lab::core::Color;
    ///
    /// let mut variant = Chess960::default();
    /// variant.lost_on_time(Color::White);
    /// ```
    ///
    fn lost_on_time(&mut self, color: Color) {
        self.game.lost_on_time(color)
    }

    /// Returns the minified FEN string of the [Game]
    ///
    /// # Returns
    /// A string with the minified FEN of the [Game]
    ///
    /// # Example
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    /// let variant = Chess960::default();
    /// let minified_fen = variant.get_minified_fen();
    /// ```
    ///
    fn get_minified_fen(&self) -> String {
        get_minified_fen(&self.fen())
    }

    /// Returns the last [Move] of the [Game]
    ///
    /// # Returns
    /// The last [Move] of the [Game], if there is one
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::StandardChess;
    /// let mut game = StandardChess::default();
    /// game.move_piece("e4").unwrap();
    /// let last_move = game.get_last_move();
    /// assert_eq!(last_move.unwrap().to_string(), "e4");
    /// ```
    ///
    fn get_last_move(&self) -> Option<crate::core::Move> {
        self.game.get_last_move()
    }

    /// Returns whether it is white's turn to move
    ///
    /// # Returns
    /// Whether it is white's turn to move
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    /// let game = Chess960::default();
    /// let color = game.is_white_turn();
    /// ```
    ///
    fn is_white_turn(&self) -> bool {
        self.game.is_white_turn
    }

    /// Returns the halfmove clock of the [Game]
    ///
    /// # Returns
    /// The halfmove clock of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    /// let game = Chess960::default();
    /// let halfmove_clock = game.get_halfmove_clock();
    /// ```
    ///
    fn get_halfmove_clock(&self) -> u32 {
        self.game.halfmove_clock
    }

    /// Returns the fullmove number of the [Game]
    ///
    /// # Returns
    /// The fullmove number of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    /// let game = Chess960::default();
    /// let fullmove_number = game.get_fullmove_number();
    /// ```
    ///
    fn get_fullmove_number(&self) -> u32 {
        self.game.fullmove_number
    }

    /// Returns the current castling rights of the [Game]
    ///
    /// # Returns
    /// The current castling rights of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
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

    /// Returns the en passant square of the [Game]
    ///
    /// # Returns
    /// The en passant square of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    /// let game = Chess960::default();
    /// let en_passant = game.get_en_passant();
    /// ```
    ///
    fn get_en_passant(&self) -> Option<Position> {
        self.game.en_passant
    }

    /// Returns the starting FEN of the [Game]
    ///
    /// # Returns
    /// A copy of the starting FEN of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    /// let game = Chess960::default();
    /// let starting_fen = game.get_starting_fen();
    /// ```
    ///
    fn get_starting_fen(&self) -> String {
        self.game.starting_fen.clone()
    }

    /// Returns the status of the [Game]
    ///
    /// # Returns
    /// The status of the [Game]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::Chess960;
    ///
    /// let game = Chess960::default();
    /// let status = game.get_status();
    /// ```
    ///
    fn get_status(&self) -> GameStatus {
        self.game.status
    }
}

#[cfg(test)]
mod tests {
    use crate::core::WinReason;

    use super::*;

    #[test]
    fn test_standard_chess_name() {
        assert_eq!(Chess960::name(), "Chess960");
    }

    #[test]
    fn test_default() {
        let variant = Chess960::default();

        let minified_fen = variant.get_minified_fen();

        let parts: Vec<&str> = minified_fen.split('/').collect();

        assert!(parts[0].chars().all(|c| c.is_lowercase()));
        assert!(parts[7].chars().all(|c| c.is_uppercase()));
        assert_eq!(parts[0], parts[7].to_lowercase());

        for piece in ['r', 'n', 'b', 'q', 'k'] {
            let count = parts[0].chars().filter(|&c| c == piece).count();
            match piece {
                'r' => assert_eq!(count, 2),
                'n' => assert_eq!(count, 2),
                'b' => assert_eq!(count, 2),
                'q' => assert_eq!(count, 1),
                'k' => assert_eq!(count, 1),
                _ => (),
            }
        }

        let mut castle_chars = vec!['r', 'k', 'r'];
        for char in parts[0].chars() {
            if !castle_chars.is_empty() && char == castle_chars[0] {
                castle_chars.remove(0);
            }
        }
        assert!(castle_chars.is_empty());
    }

    #[test]
    fn test_new() {
        let game = Game::default();
        let variant = Chess960::new(game.clone());
        assert_eq!(variant.fen(), game.fen());
    }

    #[test]
    fn test_from_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let variant = Chess960::from_fen(fen).unwrap();
        assert_eq!(variant.fen(), fen);
    }

    #[test]
    fn test_from_pgn() {
        let pgn = "[Variant \"Chess960\"]\n
            [FEN \"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1\"]\n
            1. e4 e5 2. Nf3 Nc6 3. Bb5 a6";
        let variant = Chess960::from_pgn(pgn).unwrap();
        assert!(variant.pgn().contains("1. e4 e5 2. Nf3 Nc6 3. Bb5 a6"));
    }

    #[test]
    fn test_load() {
        let path = "data/chess960/ex1.pgn";
        let variant = Chess960::load(path).unwrap();
        assert!(variant.pgn().contains("1. e4 c6 2. d4 d5 3. exd5 cxd5"));
    }

    #[test]
    fn test_load_all() {
        let path = "data/chess960/ex2.pgn";
        let variants = Chess960::load_all(path).unwrap();
        assert_eq!(variants.len(), 3);
    }

    #[test]
    fn test_move_piece() {
        let mut variant =
            Chess960::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let status = variant.move_piece("e4").unwrap();
        assert_eq!(status, GameStatus::InProgress);
        assert_eq!(
            variant.fen(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn test_undo_redo() {
        let mut variant =
            Chess960::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        variant.move_piece("e4").unwrap();
        variant.undo();
        assert_eq!(
            variant.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
        variant.redo();
        assert_eq!(
            variant.fen(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn test_save() {
        let mut variant = Chess960::default();
        let path = "data/chess960/test_save.pgn";

        variant.move_piece("e4").unwrap();
        variant.save(path, true).unwrap();

        let loaded_variant = Chess960::load(path).unwrap();
        assert_eq!(variant.fen(), loaded_variant.fen());

        // Clean up
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_resign() {
        let mut variant = Chess960::default();

        variant.resign(Color::White);
        assert_eq!(
            variant.get_status(),
            GameStatus::BlackWins(WinReason::Resignation)
        );
    }

    #[test]
    fn test_draw() {
        let mut variant = Chess960::default();

        variant.draw();
        assert_eq!(
            variant.get_status(),
            GameStatus::Draw(crate::core::DrawReason::Agreement)
        );
    }

    #[test]
    fn test_lost_on_time() {
        let mut variant = Chess960::default();

        variant.lost_on_time(Color::Black);
        assert_eq!(variant.get_status(), GameStatus::WhiteWins(WinReason::Time));
    }

    #[test]
    fn test_minified_fen() {
        let variant =
            Chess960::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let minified_fen = variant.get_minified_fen();
        assert_eq!(minified_fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }

    #[test]
    fn test_get_last_move() {
        let mut variant = Chess960::default();
        assert_eq!(variant.get_last_move(), None);

        variant.move_piece("e4").unwrap();
        let last_move = variant.get_last_move();
        assert!(last_move.is_some());
        assert_eq!(last_move.unwrap().to_string(), "e4");
    }

    #[test]
    fn test_is_white_turn() {
        let mut variant = Chess960::default();
        assert!(variant.is_white_turn());

        variant.move_piece("e4").unwrap();
        assert!(!variant.is_white_turn());
    }

    #[test]
    fn test_get_castling_rights() {
        let mut variant = Chess960::default();
        let castling_rights = variant.get_castling_rights();
        assert_eq!(castling_rights, "KQkq");

        variant =
            Chess960::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1").unwrap();
        let castling_rights = variant.get_castling_rights();
        assert_eq!(castling_rights, "-");
    }

    #[test]
    fn test_get_en_passant() {
        let mut variant = Chess960::default();
        assert_eq!(variant.get_en_passant(), None);

        variant.move_piece("e4").unwrap();
        variant.move_piece("d5").unwrap();
        variant.move_piece("e5").unwrap();
        variant.move_piece("f5").unwrap();
        assert_eq!(
            variant.get_en_passant().unwrap(),
            Position::new(5, 5).unwrap()
        );
    }

    #[test]
    fn test_get_halfmove_clock() {
        let variant = Chess960::default();
        assert_eq!(variant.get_halfmove_clock(), 0);
    }

    #[test]
    fn test_get_fullmove_number() {
        let variant = Chess960::default();
        assert_eq!(variant.get_fullmove_number(), 1);
    }

    #[test]
    fn test_get_starting_fen() {
        let variant = Chess960::default();
        let starting_fen = variant.get_starting_fen();

        assert!(starting_fen.contains("w KQkq - 0 1"));
    }

    #[test]
    fn test_get_status() {
        let variant = Chess960::default();
        assert_eq!(variant.get_status(), GameStatus::InProgress);
    }
}
