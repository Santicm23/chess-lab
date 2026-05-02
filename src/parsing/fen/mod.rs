use regex::Regex;

use crate::{
    core::{Piece, Position},
    errors::{Chess960SPIDError, FenError},
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

/// Gets the FEN back rank pieces from a Chess960 Starting Position ID
///
/// # Arguments
/// * `spid`: A u16 representing the Chess960 Starting Position ID
///
/// # Returns
/// * `Ok(String)`: A string representing the back rank pieces in FEN format
/// * `Err(Chess960SPIDError)`: An error if the SPID is invalid
///
///
fn back_rank_pieces_from_chess960_spid(spid: u16) -> Result<String, Chess960SPIDError> {
    if spid >= 960 {
        return Err(Chess960SPIDError::new(spid));
    }

    let mut id = spid as usize;
    let mut pieces: [Option<char>; 8] = [None; 8];

    let light_squares = [1, 3, 5, 7];
    let dark_squares = [0, 2, 4, 6];

    let light_index = id % 4;
    id /= 4;
    let dark_index = id % 4;
    id /= 4;

    pieces[light_squares[light_index]] = Some('B');
    pieces[dark_squares[dark_index]] = Some('B');

    let queen_index = id % 6;
    id /= 6;

    let mut empty: Vec<usize> = pieces
        .iter()
        .enumerate()
        .filter_map(|(idx, piece)| if piece.is_none() { Some(idx) } else { None })
        .collect();
    let queen_pos = empty.remove(queen_index);
    pieces[queen_pos] = Some('Q');

    let knight_table = [
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (1, 2),
        (1, 3),
        (1, 4),
        (2, 3),
        (2, 4),
        (3, 4),
    ];
    let (n1, n2) = knight_table[id % 10];

    empty = pieces
        .iter()
        .enumerate()
        .filter_map(|(idx, piece)| if piece.is_none() { Some(idx) } else { None })
        .collect();
    let knight1_pos = empty[n1];
    let knight2_pos = empty[n2];
    pieces[knight1_pos] = Some('N');
    pieces[knight2_pos] = Some('N');

    let mut remaining: Vec<usize> = pieces
        .iter()
        .enumerate()
        .filter_map(|(idx, piece)| if piece.is_none() { Some(idx) } else { None })
        .collect();
    remaining.sort_unstable();
    pieces[remaining[0]] = Some('R');
    pieces[remaining[1]] = Some('K');
    pieces[remaining[2]] = Some('R');

    Ok(pieces.iter().map(|p| p.unwrap()).collect())
}

/// Gets the full FEN string from a Chess960 Starting Position ID
///
/// # Arguments
/// * `spid`: A u16 representing the Chess960 Starting Position ID
///
/// # Returns
/// * `Ok(String)`: A string representing the full FEN for the given SPID
/// * `Err(Chess960SPIDError)`: An error if the SPID is invalid
///
/// # Example
/// ```
/// # use chess_lab::parsing::fen::get_fen_from_chess960_spid;
/// let fen = get_fen_from_chess960_spid(518).unwrap();
/// assert_eq!(fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
/// ```
///
pub fn get_fen_from_chess960_spid(spid: u16) -> Result<String, Chess960SPIDError> {
    let back_rank = back_rank_pieces_from_chess960_spid(spid)?;
    Ok(format!(
        "{}/pppppppp/8/8/8/8/PPPPPPPP/{} w KQkq - 0 1",
        back_rank.to_lowercase(),
        back_rank
    ))
}

/// Gets the reduced FEN string from a full FEN string
///
/// # Arguments
/// * `fen`: A string slice that holds the FEN representation of the game
///
/// # Returns
/// A string that contains the board representation, active color, castling availability and en passant target
///
fn get_fen_reduced(fen: &str) -> String {
    let mut fen_parts: Vec<&str> = fen.split_whitespace().collect();
    fen_parts.pop();
    fen_parts.pop();
    fen_parts.join(" ")
}

/// Gets the minified FEN string from a full FEN string
///
/// # Arguments
/// * `fen`: A string slice that holds the FEN representation of the game
///
/// # Returns
/// A string that contains only the board representation of the FEN string
///
pub(crate) fn get_minified_fen(fen: &str) -> String {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    parts[0].to_string()
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
pub fn parse_minified_fen(fen: &str) -> Result<Board, FenError> {
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
    fn test_back_rank_pieces_from_chess960_spid() {
        let pieces = back_rank_pieces_from_chess960_spid(518).unwrap();
        assert_eq!(pieces, "RNBQKBNR");
        let pieces = back_rank_pieces_from_chess960_spid(0).unwrap();
        assert_eq!(pieces, "BBQNNRKR");
    }

    #[test]
    fn test_invalid_chess960_spid() {
        let result = back_rank_pieces_from_chess960_spid(960);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_fen_from_chess960_spid() {
        let fen = get_fen_from_chess960_spid(518).unwrap();
        assert_eq!(
            fen,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
        let fen = get_fen_from_chess960_spid(40).unwrap();
        assert_eq!(
            fen,
            "nbnqbrkr/pppppppp/8/8/8/8/PPPPPPPP/NBNQBRKR w KQkq - 0 1"
        );
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

    #[test]
    fn test_minified_fen() {
        let board = parse_minified_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
    }

    #[test]
    fn test_invalid_minified_fen() {
        let board = parse_minified_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/8");
        assert!(board.is_err());
    }
}
