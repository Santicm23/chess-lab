use crate::{
    core::{Color, GameStatus, Move, Piece, Square, Variant, VariantBuilder, WinReason},
    errors::{FenError, MoveError, PGNError},
    logic::Game,
    parsing::{
        fen::get_minified_fen,
        pgn::{parse_multiple_pgn, parse_pgn},
    },
    utils::os::{read_file, write_file},
};

/// Three Check Chess [Variant]
/// In Three Check Chess, the objective is to check the opponent's king three times.
/// The first player to deliver three checks wins the game, regardless of the Square on the board.
/// This variant adds an extra layer of strategy, as players must balance between attacking and defending against checks.
///
/// # Attributes
/// * `game` - The game struct that holds the state of the game
///
pub struct ThreeCheckChess {
    pub game: Game,
    pub checks: (u8, u8),
}

impl Default for ThreeCheckChess {
    /// Creates a new instance of the [ThreeCheckChess] [Variant] with default values
    ///
    /// # Returns
    /// A new instance of the [ThreeCheckChess] [Variant]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let game = ThreeCheckChess::default();
    /// ```
    ///
    fn default() -> Self {
        Self {
            game: Game::default(),
            checks: (0, 0),
        }
    }
}

impl VariantBuilder for ThreeCheckChess {
    /// Returns the name of the variant
    ///
    /// # Returns
    /// The name of the variant as a string slice
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let name = ThreeCheckChess::name();
    /// assert_eq!(name, "Three-check");
    /// ```
    ///
    fn name() -> &'static str {
        "Three-check"
    }

    /// Creates a new instance of the [ThreeCheckChess] [Variant] with the given game state
    ///
    /// # Arguments
    /// * `game` - The game struct that holds the state of the game
    ///
    /// # Returns
    /// A new instance of the [ThreeCheckChess] [Variant]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::logic::Game;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let game = Game::default();
    /// let three_check_game = ThreeCheckChess::new(game);
    /// ```
    ///
    fn new(game: Game) -> Self {
        Self {
            game,
            checks: (0, 0),
        }
    }

    /// Creates a new instance of the [ThreeCheckChess] [Variant] from a FEN string
    ///
    /// # Arguments
    /// * `fen` - A string slice that holds the FEN representation of the game
    ///
    /// # Returns
    /// A new instance of the [ThreeCheckChess] [Variant] if the FEN string is valid, otherwise returns a [FenError]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let game = ThreeCheckChess::from_fen(fen).unwrap();
    /// ```
    ///
    fn from_fen(fen: &str) -> Result<Self, FenError> {
        Ok(Self::new(Game::from_fen(fen)?))
    }

    /// Creates a new instance of the [ThreeCheckChess] [Variant] from a PGN string
    ///
    /// # Arguments
    /// * `pgn` - A string slice that holds the PGN representation of the game
    ///
    /// # Returns
    /// A new instance of the [ThreeCheckChess] [Variant] if the PGN string is valid, otherwise returns a [PGNError]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let pgn = "[Event \"Three-check\"]\n[Variant \"Three-check\"]\n\n1. e4 e5 2. Nf3 Nc6 3. Bb5 a6";
    ///
    /// let game = ThreeCheckChess::from_pgn(pgn).unwrap();
    /// ```
    ///
    fn from_pgn(pgn: &str) -> Result<Self, PGNError> {
        parse_pgn(pgn)
    }

    /// Loads a game from a PGN file
    ///
    /// # Arguments
    /// * `path` - A string slice that holds the path to the PGN file
    ///
    /// # Returns
    /// A new instance of the [ThreeCheckChess] [Variant] if the PGN
    /// file is valid, otherwise returns a [PGNError]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let path = "data/three_check/ex1.pgn";
    /// let game = ThreeCheckChess::load(path).unwrap();
    /// ```
    ///
    fn load(path: &str) -> Result<Self, PGNError> {
        let pgn = read_file(path)?;
        Self::from_pgn(&pgn)
    }

    /// Loads multiple games from a PGN file
    ///
    /// # Arguments
    /// * `path` - A string slice that holds the path to the PGN file
    ///
    /// # Returns
    /// A vector of [ThreeCheckChess] games if the PGN file is valid,
    /// otherwise returns a [PGNError]
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::VariantBuilder;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let path = "data/three_check/ex2.pgn";
    /// let games = ThreeCheckChess::load_all(path).unwrap();
    /// ```
    ///
    fn load_all(path: &str) -> Result<Vec<Self>, PGNError> {
        let pgn = read_file(path)?;
        parse_multiple_pgn(&pgn)
    }
}

impl Variant for ThreeCheckChess {
    /// Moves a [Piece] on the board and tracks checks
    ///
    /// # Arguments
    /// * `move_str` - A move string in algebraic notation
    ///
    /// # Returns
    /// A `Result<GameStatus, MoveError>` object
    /// * `Ok(GameStatus)` - The status of the game after the move
    /// * `Err(MoveError)` - An error occurred while moving the piece
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::{GameStatus, Variant};
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let mut game = ThreeCheckChess::default();
    /// let status = game.move_piece("e4");
    ///
    /// assert_eq!(status, Ok(GameStatus::InProgress));
    /// ```
    ///
    fn move_piece(&mut self, move_str: &str) -> Result<GameStatus, MoveError> {
        let status = self.game.move_piece(move_str)?;
        if let GameStatus::InProgress = status {
            if self.game.check() {
                if self.game.is_white_turn {
                    self.checks.1 += 1;
                    if self.checks.1 >= 3 {
                        return Ok(GameStatus::BlackWins(WinReason::Checkmate));
                    }
                } else {
                    self.checks.0 += 1;
                    if self.checks.0 >= 3 {
                        return Ok(GameStatus::WhiteWins(WinReason::Checkmate));
                    }
                }
            }
        }
        Ok(status)
    }

    /// Undoes the last [Move] and rolls back check counters
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let mut game = ThreeCheckChess::default();
    /// game.move_piece("e4").unwrap();
    /// game.undo();
    ///
    /// assert_eq!(game.fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    fn undo(&mut self) {
        if self.game.check() {
            if self.game.is_white_turn {
                self.checks.1 -= 1;
            } else {
                self.checks.0 -= 1;
            }
        }
        self.game.undo();
    }

    /// Redoes the last undone [Move] and reapplies check counters
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let mut game = ThreeCheckChess::default();
    /// game.move_piece("e4").unwrap();
    /// game.undo();
    /// game.redo();
    ///
    /// assert_eq!(game.fen(), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
    /// ```
    ///
    fn redo(&mut self) {
        self.game.redo();
        if self.game.check() {
            if self.game.is_white_turn {
                self.checks.1 += 1;
            } else {
                self.checks.0 += 1;
            }
        }
    }

    /// Returns the PGN string of the game
    ///
    /// # Returns
    /// The PGN string of the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let mut game = ThreeCheckChess::default();
    /// game.move_piece("e4").unwrap();
    /// game.move_piece("e5").unwrap();
    /// let pgn = game.pgn();
    ///
    /// assert!(pgn.contains("1. e4 e5"));
    /// ```
    ///
    fn pgn(&self) -> String {
        self.game.pgn()
    }

    /// Returns the FEN string of the game
    ///
    /// # Returns
    /// The FEN string of the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let game = ThreeCheckChess::default();
    /// let fen = game.fen();
    ///
    /// assert_eq!(fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    fn fen(&self) -> String {
        self.game.fen()
    }

    /// Returns the piece at a given Square
    ///
    /// # Arguments
    /// * `sqr` - The Square to get the piece from
    ///
    /// # Returns
    /// The piece at the given Square, if there is one
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// use chess_lab::core::Square;
    ///
    /// let game = ThreeCheckChess::default();
    /// let piece = game.get_piece_at(Square::from_string("e2").unwrap());
    /// assert!(piece.is_some());
    /// assert_eq!(piece.unwrap().to_string(), "P");
    /// ```
    ///
    fn get_piece_at(&self, sqr: Square) -> Option<Piece> {
        self.game.get_piece_at(sqr)
    }

    /// Returns the legal moves of a piece at a given Square
    ///
    /// # Arguments
    /// * `sqr` - The Square to get the legal moves for
    ///
    /// # Returns
    /// A vector of legal moves for the piece at the given Square
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// use chess_lab::core::Square;
    ///
    /// let game = ThreeCheckChess::default();
    /// let legal_moves = game.get_legal_moves(Square::from_string("e2").unwrap());
    /// assert!(legal_moves.iter().any(|m| m.to_string() == "e4"));
    /// ```
    ///
    fn get_legal_moves(&self, sqr: Square) -> Vec<Move> {
        self.game.get_legal_moves(sqr)
    }

    /// Saves the game to a file
    ///
    /// # Arguments
    /// * `path` - The path to the file
    /// * `overwrite` - Whether to overwrite the file if it already exists
    ///
    /// # Returns
    /// A `Result<(), std::io::Error>` object
    /// * `Ok(())` - The game was saved successfully
    /// * `Err(std::io::Error)` - An error occurred while saving the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let game = ThreeCheckChess::default();
    /// let path = "data/standard/ex.pgn";
    ///
    /// game.save(path, true).unwrap();
    /// ```
    ///
    fn save(&self, path: &str, overwrite: bool) -> Result<(), std::io::Error> {
        write_file(path, self.pgn().as_str(), !overwrite)?;
        Ok(())
    }

    /// Resigns the game for a player
    ///
    /// # Arguments
    /// * `color` - The [Color] of the player who resigns
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// use chess_lab::core::Color;
    /// let mut game = ThreeCheckChess::default();
    /// game.resign(Color::White);
    /// ```
    ///
    fn resign(&mut self, color: Color) {
        self.game.resign(color)
    }

    /// Sets the game as a draw by agreement
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let mut game = ThreeCheckChess::default();
    /// game.draw();
    /// ```
    ///
    fn draw(&mut self) {
        self.game.draw_by_agreement()
    }

    /// Sets the game as lost on time for a player
    ///
    /// # Arguments
    /// * `color` - The [Color] of the player who lost on time
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// use chess_lab::core::Color;
    /// let mut game = ThreeCheckChess::default();
    /// game.lost_on_time(Color::Black);
    /// ```
    ///
    fn lost_on_time(&mut self, color: Color) {
        self.game.lost_on_time(color)
    }

    /// Returns the minified FEN string of the game
    ///
    /// # Returns
    /// The minified FEN string of the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let game = ThreeCheckChess::default();
    /// let minified_fen = game.get_minified_fen();
    /// assert_eq!(minified_fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    /// ```
    ///
    fn get_minified_fen(&self) -> String {
        get_minified_fen(&self.fen())
    }

    /// Returns the last [Move] of the game
    ///
    /// # Returns
    /// The last [Move] of the game, if there is one
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let mut game = ThreeCheckChess::default();
    /// game.move_piece("e4").unwrap();
    /// let last_move = game.get_last_move();
    /// assert_eq!(last_move.unwrap().to_string(), "e4");
    /// ```
    ///
    fn get_last_move(&self) -> Option<Move> {
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
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let mut game = ThreeCheckChess::default();
    /// assert!(game.is_white_turn());
    /// game.move_piece("e4").unwrap();
    /// assert!(!game.is_white_turn());
    /// ```
    ///
    fn is_white_turn(&self) -> bool {
        self.game.is_white_turn
    }

    /// Returns the halfmove clock of the game
    ///
    /// # Returns
    /// The halfmove clock of the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let game = ThreeCheckChess::default();
    /// let halfmove_clock = game.get_halfmove_clock();
    /// assert_eq!(halfmove_clock, 0);
    /// ```
    ///
    fn get_halfmove_clock(&self) -> u32 {
        self.game.halfmove_clock
    }

    /// Returns the fullmove number of the game
    ///
    /// # Returns
    /// The fullmove number of the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let game = ThreeCheckChess::default();
    /// let fullmove_number = game.get_fullmove_number();
    /// assert_eq!(fullmove_number, 1);
    /// ```
    ///
    fn get_fullmove_number(&self) -> u32 {
        self.game.fullmove_number
    }

    /// Returns the castling rights of the game
    ///
    /// # Returns
    /// The castling rights of the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let game = ThreeCheckChess::default();
    /// let castling_rights = game.get_castling_rights();
    /// assert_eq!(castling_rights, "KQkq");
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

    /// Returns the en passant square of the game
    ///
    /// # Returns
    /// The en passant square of the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let game = ThreeCheckChess::default();
    /// let en_passant = game.get_en_passant();
    /// assert_eq!(en_passant, None);
    /// ```
    ///
    fn get_en_passant(&self) -> Option<Square> {
        self.game.en_passant
    }

    /// Returns the starting FEN of the game
    ///
    /// # Returns
    /// A copy of the starting FEN of the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::Variant;
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let game = ThreeCheckChess::default();
    /// let starting_fen = game.get_starting_fen();
    /// assert_eq!(
    ///     starting_fen,
    ///     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    /// );
    /// ```
    ///
    fn get_starting_fen(&self) -> String {
        self.game.starting_fen.clone()
    }

    /// Returns the status of the game
    ///
    /// # Returns
    /// The status of the game
    ///
    /// # Examples
    /// ```
    /// # use chess_lab::core::{GameStatus, Variant};
    /// # use chess_lab::variants::ThreeCheckChess;
    /// let game = ThreeCheckChess::default();
    /// let status = game.get_status();
    /// assert_eq!(status, GameStatus::InProgress);
    /// ```
    ///
    fn get_status(&self) -> GameStatus {
        self.game.status
    }
}

#[cfg(test)]
mod tests {
    use crate::core::DrawReason;

    use super::*;

    #[test]
    fn test_three_check_name() {
        assert_eq!(ThreeCheckChess::name(), "Three-check");
    }

    #[test]
    fn test_default() {
        let variant = ThreeCheckChess::default();
        assert_eq!(
            variant.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
        assert_eq!(variant.checks, (0, 0));
    }

    #[test]
    fn test_new() {
        let game = Game::default();
        let variant = ThreeCheckChess::new(game.clone());
        assert_eq!(variant.fen(), game.fen());
        assert_eq!(variant.checks, (0, 0));
    }

    #[test]
    fn test_from_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let variant = ThreeCheckChess::from_fen(fen).unwrap();
        assert_eq!(variant.fen(), fen);
    }

    #[test]
    fn test_from_pgn() {
        let pgn =
            "[Event \"Three-check\"]\n[Variant \"Three-check\"]\n\n1. e4 e5 2. Nf3 Nc6 3. Bb5 a6";
        let variant = ThreeCheckChess::from_pgn(pgn).unwrap();
        assert!(variant.pgn().contains("1. e4 e5 2. Nf3 Nc6 3. Bb5 a6"));
    }

    #[test]
    fn test_load() {
        let path = "data/three_check/ex1.pgn";
        let variant = ThreeCheckChess::load(path).unwrap();
        assert!(variant.pgn().contains("1. e4 e5 2. Nf3 Nc6 3. Bb5 a6"));
    }

    #[test]
    fn test_load_all() {
        let path = "data/three_check/ex2.pgn";
        let variants = ThreeCheckChess::load_all(path).unwrap();
        assert_eq!(variants.len(), 2);
    }

    #[test]
    fn test_move_piece() {
        let mut variant = ThreeCheckChess::default();
        let status = variant.move_piece("e4").unwrap();
        assert_eq!(status, GameStatus::InProgress);
        assert_eq!(
            variant.fen(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn test_get_piece_at() {
        let variant = ThreeCheckChess::default();
        let piece = variant.get_piece_at(Square::from_string("e2").unwrap());
        assert!(piece.is_some());
        assert_eq!(piece.unwrap().to_string(), "P");
    }

    #[test]
    fn test_get_legal_moves() {
        let variant = ThreeCheckChess::default();
        let legal_moves = variant.get_legal_moves(Square::from_string("e2").unwrap());
        assert!(legal_moves.iter().any(|m| m.to_string() == "e4"));
        assert!(legal_moves.iter().any(|m| m.to_string() == "e3"));
    }

    #[test]
    fn test_undo_redo() {
        let mut variant = ThreeCheckChess::default();
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
        let mut variant = ThreeCheckChess::default();
        let path = "data/three_check/test_save.pgn";

        variant.move_piece("e4").unwrap();
        variant.save(path, true).unwrap();

        let saved_pgn = std::fs::read_to_string(path).unwrap();
        let pgn = format!(
            "[Event \"Three-check\"]\n[Variant \"Three-check\"]\n\n{}",
            saved_pgn.trim()
        );
        let loaded_variant = ThreeCheckChess::from_pgn(&pgn).unwrap();
        assert_eq!(variant.fen(), loaded_variant.fen());

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_resign() {
        let mut variant = ThreeCheckChess::default();

        variant.resign(Color::White);
        assert_eq!(
            variant.get_status(),
            GameStatus::BlackWins(WinReason::Resignation)
        );
    }

    #[test]
    fn test_draw() {
        let mut variant = ThreeCheckChess::default();

        variant.draw();
        assert_eq!(
            variant.get_status(),
            GameStatus::Draw(DrawReason::Agreement)
        );
    }

    #[test]
    fn test_lost_on_time() {
        let mut variant = ThreeCheckChess::default();

        variant.lost_on_time(Color::Black);
        assert_eq!(variant.get_status(), GameStatus::WhiteWins(WinReason::Time));
    }

    #[test]
    fn test_minified_fen() {
        let variant = ThreeCheckChess::default();
        let minified_fen = variant.get_minified_fen();
        assert_eq!(minified_fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }

    #[test]
    fn test_get_last_move() {
        let mut variant = ThreeCheckChess::default();
        assert_eq!(variant.get_last_move(), None);

        variant.move_piece("e4").unwrap();
        let last_move = variant.get_last_move();
        assert!(last_move.is_some());
        assert_eq!(last_move.unwrap().to_string(), "e4");
    }

    #[test]
    fn test_is_white_turn() {
        let mut variant = ThreeCheckChess::default();
        assert!(variant.is_white_turn());

        variant.move_piece("e4").unwrap();
        assert!(!variant.is_white_turn());
    }

    #[test]
    fn test_get_castling_rights() {
        let mut variant = ThreeCheckChess::default();
        let castling_rights = variant.get_castling_rights();
        assert_eq!(castling_rights, "KQkq");

        variant =
            ThreeCheckChess::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1")
                .unwrap();
        let castling_rights = variant.get_castling_rights();
        assert_eq!(castling_rights, "-");
    }

    #[test]
    fn test_get_en_passant() {
        let mut variant = ThreeCheckChess::default();
        assert_eq!(variant.get_en_passant(), None);

        variant.move_piece("e4").unwrap();
        variant.move_piece("d5").unwrap();
        variant.move_piece("e5").unwrap();
        variant.move_piece("f5").unwrap();
        assert_eq!(
            variant.get_en_passant().unwrap(),
            Square::new(5, 5).unwrap()
        );
    }

    #[test]
    fn test_get_halfmove_clock() {
        let variant = ThreeCheckChess::default();
        assert_eq!(variant.get_halfmove_clock(), 0);
    }

    #[test]
    fn test_get_fullmove_number() {
        let variant = ThreeCheckChess::default();
        assert_eq!(variant.get_fullmove_number(), 1);
    }

    #[test]
    fn test_get_starting_fen() {
        let variant = ThreeCheckChess::default();
        let starting_fen = variant.get_starting_fen();

        assert_eq!(
            starting_fen,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_get_status() {
        let variant = ThreeCheckChess::default();
        assert_eq!(variant.get_status(), GameStatus::InProgress);
    }

    #[test]
    fn test_check_increments_counter() {
        let mut game = ThreeCheckChess::from_fen("4k3/8/8/8/8/8/4Q3/4K3 w - - 0 1").unwrap();

        let status = game.move_piece("Qe7+").unwrap();

        assert_eq!(status, GameStatus::InProgress);
        assert_eq!(game.checks, (1, 0));
    }

    #[test]
    fn test_third_check_wins() {
        let mut game = ThreeCheckChess::from_fen("4k3/8/8/8/8/8/4Q3/4K3 w - - 0 1").unwrap();
        game.checks = (2, 0);

        let status = game.move_piece("Qe7+").unwrap();

        assert_eq!(status, GameStatus::WhiteWins(WinReason::Checkmate));
        assert_eq!(game.checks, (3, 0));
    }

    #[test]
    fn test_undo_redo_adjusts_checks() {
        let mut game = ThreeCheckChess::from_fen("4k3/8/8/8/8/8/4Q3/4K3 w - - 0 1").unwrap();

        game.move_piece("Qe7+").unwrap();
        assert_eq!(game.checks, (1, 0));

        game.undo();
        assert_eq!(game.checks, (0, 0));

        game.redo();
        assert_eq!(game.checks, (1, 0));
    }
}
