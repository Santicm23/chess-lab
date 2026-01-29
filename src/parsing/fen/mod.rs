use regex::Regex;

use crate::{
    core::{Piece, Position},
    errors::FenError,
    logic::{Board, Game},
};

/// Creates a new game from a FEN string
///
/// # Arguments
/// * `fen`: A string slice that holds the FEN representation of the game
///
/// # Returns
/// * `Ok(Game)`: A new game
/// * `Err(FenError)`: An error if the FEN is invalid
///
/// # Example
/// ```
/// # use chess_lab::logic::Game;
/// let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
/// assert_eq!(game.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
/// ```
///
pub fn parse_fen(fen: &str) -> Result<Game, FenError> {
    let re = Regex::new(r"^([1-8PpNnBbRrQqKk]{1,8}/){7}[1-8PpNnBbRrQqKk]{1,8} [wb] (-|[KQkq]{1,4}) (-|[a-h][1-8]) \d+ ([1-9]\d*)$").unwrap();

    if !re.is_match(fen) {
        return Err(FenError::new(fen.to_string()));
    }

    let mut game = Game::default();
    game.starting_fen = fen.to_string();

    game.prev_positions.clear();
    game.prev_positions.insert(get_fen_reduced(&game.fen()), 1);

    let parts = fen.split(' ').collect::<Vec<&str>>();
    game.board = Board::new(parts[0])?;
    game.is_white_turn = parts[1] == "w";
    game.castling_rights = parts[2].chars().fold(0, |acc, c| match c {
        'K' => acc | 0b1000,
        'Q' => acc | 0b0100,
        'k' => acc | 0b0010,
        'q' => acc | 0b0001,
        _ => 0,
    });

    game.en_passant = if parts[3] == "-" {
        None
    } else {
        Some(Position::from_string(parts[3]).unwrap())
    };
    game.halfmove_clock = parts[4].parse::<u32>().unwrap();
    game.fullmove_number = parts[5].parse::<u32>().unwrap();

    Ok(game)
}

fn get_fen_reduced(fen: &str) -> String {
    let mut fen_parts: Vec<&str> = fen.split_whitespace().collect();
    fen_parts.pop();
    fen_parts.pop();
    fen_parts.join(" ")
}

/// Creates a new board from a FEN string
///
/// # Arguments
/// * `fen`: A FEN string representing the board
///
/// # Returns
/// A `Result<Board, FenError>` object
/// * `Ok(Board)`: A new board with the position represented by the FEN string
/// * `Err(FenError)`: If the FEN string is invalid
///
pub fn parse_simple_fen(fen: &str) -> Result<Board, FenError> {
    let re = Regex::new(r"^([1-8PpNnBbRrQqKk]{1,8}/){7}[1-8PpNnBbRrQqKk]{1,8}$").unwrap(); // safe unwrap

    if !re.is_match(fen) {
        return Err(FenError::new(fen.to_string()));
    }

    let mut board = Board::empty();
    let ranks = fen.split('/').collect::<Vec<&str>>();

    let mut row = 8;
    for rank in ranks {
        row -= 1;

        let mut col = 0;
        for c in rank.chars() {
            match c {
                '1'..='8' => {
                    col += c.to_digit(10).unwrap() as u8;
                }
                _ => {
                    let piece = Piece::from_fen(c).unwrap();

                    board
                        .set_piece(
                            piece,
                            &Position::new(col, row).map_err(|_| FenError::new(fen.to_string()))?,
                        )
                        .unwrap();

                    col += 1;
                }
            }
        }
    }
    Ok(board)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_fen() {
        let board = parse_simple_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
    }

    #[test]
    fn test_invalid_simple_fen() {
        let board = parse_simple_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/8");
        assert!(board.is_err());
    }

    #[test]
    fn test_fen() {
        let game =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_invalid_fen() {
        let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0");
        assert!(game.is_err());
    }

    #[test]
    fn test_fen_reduced() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let reduced = get_fen_reduced(fen);
        assert_eq!(
            reduced,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -"
        );
    }

    #[test]
    fn test_en_passant_fen() {
        let game =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e3 0 1").unwrap();
        assert_eq!(
            game.fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e3 0 1"
        );
        assert_eq!(game.en_passant, Some(Position::from_string("e3").unwrap()));
    }
}
