use std::collections::HashMap;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{
    core::{Variant, VariantBuilder},
    errors::PGNError,
    logic::Game,
    variants::StandardChess,
};

/// PGN parser struct that uses the pest library to parse PGN strings
#[derive(Parser)]
#[grammar = "./src/parsing/pgn/pgn.pest"]
struct PGNParser;

/// Parse a multiple games PGN string into a vector of [Game] structs
///
/// # Arguments
/// * `input` - A string slice that holds the PGN string to be parsed
///
/// # Returns
/// A `Result<Vec<T>, PGNError>` object
/// * `Ok(Vec<Game>)` - A vector of [Game] structs with the parsed PGNs
/// * `Err(PGNError)` - A [PGNError] with the reason why the PGN string could not be parsed
///
pub fn parse_multiple_pgn<T: Variant + VariantBuilder>(input: &str) -> Result<Vec<T>, PGNError> {
    let pair = PGNParser::parse(Rule::pgn_file, input)
        .map_err(|e| PGNError::InvalidPgn(e.to_string()))?
        .next()
        .ok_or(PGNError::InvalidPgn(input.to_string()))?;

    let mut games = Vec::new();

    for pgn in pair.into_inner() {
        if !matches!(pgn.as_rule(), Rule::pgn) {
            continue;
        }
        games.push(parse_single_pgn(pgn)?);
    }

    let mut variants: Vec<T> = Vec::new();

    for game in games.iter() {
        if game.get_variant() != T::name()
            && !(T::name() == StandardChess::name() && game.get_variant() == "From Position")
        {
            return Err(PGNError::InvalidVariant(game.get_variant()));
        }
        let variant = T::new(game.clone());
        variants.push(variant);
    }

    Ok(variants)
}

/// Parse a PGN string into a Game struct
///
/// # Arguments
/// * `input` - A string slice that holds the PGN to be parsed
///
/// # Returns
/// A `Result<T, PGNError>` object
/// * `Ok(Game)` - A Game struct with the parsed PGN
/// * `Err(PgnError)` - An error with the reason why the PGN could not be parsed
///
/// pub fn parse_pgn<T: Variant + VariantBuilder>(input: &str) -> Result<T, PGNError> {
pub fn parse_pgn<T: Variant + VariantBuilder>(input: &str) -> Result<T, PGNError> {
    let pair = PGNParser::parse(Rule::pgn, input)
        .map_err(|_| PGNError::InvalidPgn(input.to_string()))?
        .next()
        .ok_or(PGNError::InvalidPgn(input.to_string()))?;

    let game = parse_single_pgn(pair)?;
    if game.get_variant() != T::name()
        && !(T::name() == StandardChess::name() && game.get_variant() == "From Position")
    {
        return Err(PGNError::InvalidVariant(game.get_variant()));
    }

    Ok(T::new(game))
}

/// Plays a sequence of moves in a game
///
/// # Arguments
/// * `game` - A mutable reference to a Game struct
/// * `sequence` - A Pair<Rule> that holds the sequence of moves to be played
///
fn parse_sequence(game: &mut Game, sequence: Pair<Rule>) {
    for subsequence in sequence.into_inner() {
        match subsequence.as_rule() {
            Rule::line => {
                parse_line(game, subsequence);
            }
            Rule::white_sequence => {
                parse_white_sequence(game, subsequence);
            }
            Rule::black_sequence => {
                parse_black_sequence(game, subsequence);
            }
            _ => unreachable!(),
        }
    }
}

/// Parse a PGN rule into a Game struct
///
/// # Arguments
/// * `pgn` - A Pair<Rule> that holds the PGN to be parsed
///
/// # Returns
/// A `Result<Game, PGNError>` object
/// * `Ok(Game)` - A Game struct with the parsed PGN
/// * `Err(PgnError)` - An error with the reason why the PGN could not be parsed
///
fn parse_single_pgn(pgn: Pair<Rule>) -> Result<Game, PGNError> {
    let mut game = Game::default();

    let mut metadata = HashMap::new();

    for record in pgn.into_inner() {
        match record.as_rule() {
            Rule::metadata => {
                let mut pairs = record.into_inner();
                let key = pairs.next().unwrap().as_span().as_str();
                let op_value = pairs.next();
                let value = if op_value.is_some() {
                    op_value.unwrap().as_span().as_str()
                } else {
                    ""
                };

                if !metadata.contains_key(key) {
                    metadata.insert(key, value);
                }
                if key == "FEN" {
                    game = Game::from_fen(value)?;
                }
            }
            Rule::sequence => {
                metadata.iter().for_each(|(key, value)| {
                    if let Err(e) = game.history.add_metadata(key, value) {
                        eprintln!("Warning: {}", e);
                    }
                });
                parse_sequence(&mut game, record);
            }
            Rule::result => (),
            Rule::COMMENT => (),
            _ => unreachable!(),
        }
    }

    Ok(game)
}

/// Plays a white variation of moves in a game
///
/// # Arguments
/// * `game` - A mutable reference to a Game struct
/// * `white_sequence` - A Pair<Rule> that holds the white sequence of moves to be played
///
fn parse_white_sequence(game: &mut Game, white_sequence: Pair<Rule>) {
    for mov_type in white_sequence.into_inner() {
        match mov_type.as_rule() {
            Rule::partial_move => {
                parse_partial_move(game, mov_type);
            }
            Rule::full_move => {
                parse_full_move(game, mov_type);
            }
            Rule::half_sequence => {
                parse_half_sequence(game, mov_type);
            }
            Rule::multi_subsequence => {
                parse_multi_subsequence(game, mov_type);
            }
            Rule::COMMENT => (),
            _ => unreachable!(),
        }
    }
}

/// Plays a black variation of moves in a game
///
/// # Arguments
/// * `game` - A mutable reference to a Game struct
/// * `black_sequence` - A Pair<Rule> that holds the black sequence of moves to be played
///
fn parse_black_sequence(game: &mut Game, black_sequence: Pair<Rule>) {
    for mov_type in black_sequence.into_inner() {
        match mov_type.as_rule() {
            Rule::full_move => {
                parse_full_move(game, mov_type);
            }
            Rule::sequence => {
                parse_sequence(game, mov_type);
            }
            Rule::multi_half_subsequence => {
                parse_multi_half_subsequence(game, mov_type);
            }
            Rule::COMMENT => (),
            _ => unreachable!(),
        }
    }
}

/// Plays a multi subsequence of moves in a game
///
/// # Arguments
/// * `game` - A mutable reference to a Game struct
/// * `multi_subsequence` - A Pair<Rule> that holds the multi subsequence of moves to be played
///
fn parse_multi_subsequence(game: &mut Game, multi_subsequence: Pair<Rule>) {
    for subsequence in multi_subsequence.into_inner() {
        match subsequence.as_rule() {
            Rule::subsequence => {
                parse_subsequence(game, subsequence.clone());
            }
            Rule::COMMENT => (),
            _ => unreachable!(),
        }
    }
}

/// Plays a sub variation of moves in a game
///
/// # Arguments
/// * `game` - A mutable reference to a Game struct
/// * `subsequence` - A Pair<Rule> that holds the subsequence of moves to be played
///
fn parse_subsequence(game: &mut Game, subsequence: Pair<Rule>) {
    let sequence = subsequence.into_inner().next().unwrap();

    game.undo();

    let root_fullmove_number = game.fullmove_number;

    parse_sequence(game, sequence);

    let mut fullmove_number = game.fullmove_number;

    while root_fullmove_number != fullmove_number && game.fen() != game.starting_fen {
        game.undo();
        fullmove_number = game.fullmove_number;
    }

    game.undo();
    game.redo();
}

/// Plays a multi half subsequence of moves in a game
///
/// # Arguments
/// * `game` - A mutable reference to a Game struct
/// * `multi_half_subsequence` - A Pair<Rule> that holds the multi half subsequence of moves to be played
///
fn parse_multi_half_subsequence(game: &mut Game, multi_half_subsequence: Pair<Rule>) {
    for half_subsequence in multi_half_subsequence.into_inner() {
        match half_subsequence.as_rule() {
            Rule::half_subsequence => {
                parse_half_subsequence(game, half_subsequence);
            }
            Rule::COMMENT => (),
            _ => unreachable!(),
        }
    }
}

/// Plays a sub variation of moves in a game that starts with a half move
///
/// # Arguments
/// * `game` - A mutable reference to a Game struct
/// * `half_subsequence` - A Pair<Rule> that holds the half subsequence of moves to be played
///
fn parse_half_sequence(game: &mut Game, half_sequence: Pair<Rule>) {
    for pair in half_sequence.into_inner() {
        match pair.as_rule() {
            Rule::r#move => {
                let mov = pair.as_span().as_str();
                game.move_piece(mov).unwrap();
            }
            Rule::half_move => {
                parse_half_move(game, pair);
            }
            Rule::sequence => {
                parse_sequence(game, pair);
            }
            Rule::COMMENT => (),
            _ => unreachable!(),
        }
    }
}

/// Plays a sub variation of moves in a game that starts with a half move
///
/// # Arguments
/// * `game` - A mutable reference to a Game struct
/// * `half_subsequence` - A Pair<Rule> that holds the half subsequence of moves to be played
///
fn parse_half_subsequence(game: &mut Game, half_subsequence: Pair<Rule>) {
    let half_sequence = half_subsequence.into_inner().next().unwrap();

    game.undo();
    let first_fen = game.fen();

    parse_half_sequence(game, half_sequence);

    game.undo();

    while game.fen() != first_fen {
        game.undo();
    }

    game.redo();
}

/// Plays a line of moves in a game
///
/// # Arguments
/// * `game` - A mutable reference to a Game struct
/// * `line` - A Pair<Rule> that holds the line of moves to be played
///
fn parse_line(game: &mut Game, line: Pair<Rule>) {
    for mov_type in line.into_inner() {
        match mov_type.as_rule() {
            Rule::partial_move => {
                parse_partial_move(game, mov_type);
            }
            Rule::full_move => {
                parse_full_move(game, mov_type);
            }
            Rule::COMMENT => (),
            _ => unreachable!(),
        }
    }
}

/// Plays a partial move in a game
///
/// # Arguments
/// * `game` - A mutable reference to a Game struct
/// * `partial_move` - A Pair<Rule> that holds the partial move to be played
///
fn parse_partial_move(game: &mut Game, partial_move: Pair<Rule>) {
    for part in partial_move.into_inner() {
        match part.as_rule() {
            Rule::r#move => {
                let mov = part.as_span().as_str();
                game.move_piece(mov).unwrap();
            }
            Rule::move_number | Rule::COMMENT => (),
            _ => {
                println!("{:?}", part);
                unreachable!()
            }
        }
    }
}

/// Plays a half move in a game
///
/// # Arguments
/// * `game` - A mutable reference to a Game struct
/// * `half_move` - A Pair<Rule> that holds the half move to be played
///
fn parse_half_move(game: &mut Game, half_move: Pair<Rule>) {
    for part in half_move.into_inner() {
        match part.as_rule() {
            Rule::r#move => {
                let mov = part.as_span().as_str();
                game.move_piece(mov).unwrap();
            }
            Rule::second_move_number | Rule::COMMENT => (),
            _ => unreachable!(),
        }
    }
}

/// Plays a full move in a game
///
/// # Arguments
/// * `game` - A mutable reference to a Game struct
/// * `full_move` - A Pair<Rule> that holds the full move to be played
///
fn parse_full_move(game: &mut Game, full_move: Pair<Rule>) {
    for part in full_move.into_inner() {
        match part.as_rule() {
            Rule::r#move => {
                let mov = part.as_span().as_str();
                game.move_piece(mov).unwrap();
            }
            Rule::move_number | Rule::COMMENT => (),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::Variant, utils::os::read_file, variants::StandardChess};

    use super::*;

    #[test]
    fn test_parse_pgn() {
        let input = "[Event \"caro kann: exchange\"]
        [Site \"https://lichess.org/study/H6cwzXnM/pE8AwLer\"]
        [Result \"1-0\"]
        [Variant \"Standard\"]
        [ECO \"B13\"]
        [Opening \"Caro-Kann Defense: Exchange Variation\"]
        [Annotator \"https://lichess.org/@/Santicastrom\"]
        [UTCDate \"2021.07.20\"]
        [UTCTime \"16:35:48\"]
        1. e4 c6 2. d4 d5 3. exd5 cxd5 { [%cal Gc2c4,Gg1f3,Gc2c3,Gc1f4,Yf1d3] } 4. Bd3 (4. c4 Nf6 { [%cal Gg1f3,Gc4d5,Gc4c5,Yb1c3] } 5. Nc3 Nc6 { [%cal Gc4d5,Gc1f4,Gc1e3,Rg1f3,Bc1g5] } 6. Nf3 (6. Bg5 a6 7. Bxf6 (7. Nf3 Be6 { [%cal Yf1e2,Gc4c5] } 8. Be2 (8. c5 g6 9. Bd3 Bg7) 8... g6 { [%cal Gb2b3,Yg5f6,Ge1g1] } 9. Bxf6 exf6 { [%cal Gc4c5,Ye1g1] } 10. O-O Bg7 { [%cal Yd1d2,Gc4c5] } 11. Qd2 h5 { [%cal Rb2b3] }) 7... exf6 8. cxd5 Ne7 { [%cal Rd1a4] } 9. Qa4+ Bd7) 6... g6 { [%cal Gc1g5,Gf1e2,Gc4c5,Yc4d5,Gh2h3]} 7. cxd5 Nxd5 { [%csl Gd1,Gb3][%cal Gd1b3,Gf1c4,Gf1b5,Gf1e2,Gd1d3] } 8. Qb3 e6 { [%csl Gf1,Gb5][%cal Gf1b5,Gc1g5,Gf1c4,Gc3d5,Gf1e2] } 9. Bb5 Bg7 { [%csl Ge1,Gg1][%cal Ge1g1,Gc1g5,Gb5c6,Gc3d5,Gf3e5,Gb3a3] } 10. O-O O-O { [%csl Gb5,Gc6][%cal Gb5c6,Gf1d1,Gc3d5,Gc1g5] } 11. Bxc6 bxc6 { [%csl Gc3,Ga4][%cal Gc3a4,Gf1e1,Gc3e4,Gf1d1,Gc1d2,Gc1g5] } 12. Na4 Qd6 { [%csl Gf1,Ge1][%cal Gf1e1,Gc1d2,Gb3d1,Ga2a3,Ga4c5] } 13. Re1 Rb8 { [%csl Gb3,Gd1][%cal Gb3d1,Gb3c2,Gb3d3] } 14. Qd1 c5 15. Nxc5 Bb7 { [%csl Gc5,Gb7][%cal Gc5b7,Gc5e4] } 16. Nxb7 Rxb7 { [%csl Gb2,Gb3][%cal Gb2b3,Gh2h4,Ge1e2] } 17. b3 Rc8 { [%cal Gf3e5] }) (4. Nf3 Nc6 { [%cal Bc2c3,Rc2c4,Gf3e5,Gf1b5,Gf1e2,Gc1f4,Gh2h3,Gb1d2,Gb1c3,Gb2b3] } 5. c3 (5. c4) (5. Bb5 Qa5+ 6. Nc3 Bg4 { [%cal Rh2h3,Bc1d2] })) 4... Nc6 { [%cal Gg1f3,Yc2c3,Gg1e2,Ga2a3,Gc1f4] } 5. c3 (5. Nf3 Bg4 { [%cal Yc2c3,Gc1e3,Ge1g1,Gb1d2] } 6. c3 Qc7 { [%cal Rb1d2,Be1g1,Gh2h3,Gc1e3,Gc1g5,Gd1b3,Gb1a3] } 7. O-O e6 { [%cal Rb1d2,Bh2h3,Gf1e1,Gc1e3,Gc1g5,Gb1a3] } 8. h3 Bh5 { [%cal Yf1e1,Gc1e3,Gb2b4,Gb1d2,Ga2a4] } 9. Re1 Bd6 { [%cal Yb1d2,Gc1g5,Gb2b4,Gc1e3,Gb1a3] } 10. Nbd2 Nge7 { [%cal Ra2a4,Bd2f1,Gd2b3,Gb2b3] } 11. Nf1 h6 { [%cal Yd3e2] } 12. Be2 Bg6 { [%cal Re2d3,Bf3h4] }) 5... Nf6 { [%cal Gc1g5,Gg1e2,Gh2h3,Rc1f4,Bg1f3] } 6. Nf3 (6. Bf4 Bg4 { [%cal Rd1c2,Bd1b3,Gg1f3,Gg1e2,Gf2f3,Gd1a4,Gd3e2] } 7. Qb3 (7. f3 Bh5 8. g4 Bg6 9. Ne2) 7... Qd7 { [%cal Yb1d2,Gh2h3] } 8. Nd2 e6 { [%cal Yg1f3,Gh2h3] } 9. Ngf3 Bd6 { [%cal Yf4d6,Gf3e5,Gf4e5,Gf4g3,Ge1g1,Gf4g5] } 10. Bxd6 Qxd6 { [%cal Ye1g1,Gb3b7,Gh2h3] } 11. O-O O-O { [%cal Yf1e1,Ga1e1] } 12. Rfe1 Bh5 { [%cal Bf3e5,Rh2h3] } 13. Ne5 Qc7 { [%cal Bf2f4,Rb3c2,Gh2h3] } 14. f4 Ne7 { [%cal Gb3c2,Ra2a3,Gg2g3] }) (6. Bg5 Bg4 { [%cal Bd1b3,Rg1e2,Gg1f3] } 7. Qb3 (7. Ne2 e6 { [%cal Yd1c2,Gd1b3] } 8. Qc2 Qc7 { [%cal Gg5f6,Gg7f6,Gd3h7,Ye2g3] } 9. Ng3 Nh5 { [%cal Yb1d2] }) 7... e5 { [%cal Rb3b7] } 8. Qxb7 Bd7 9. Bxf6 gxf6 10. Bf5 Rb8 { [%cal Rf5d7] }) 6... Bg4 { [%cal Gb1d2,Be1g1,Rh2h3] } 7. O-O Qb8 { [%cal Rh2h3] } 8. h3 Bh5 { [%cal Rg2g3,Gc1g5] } 1-0";

        let variant: StandardChess = parse_pgn(input).unwrap();

        let pgn = variant.pgn();

        assert!(pgn.contains("[Event \"caro kann: exchange\"]"));
        assert!(pgn.contains("[Site \"https://lichess.org/study/H6cwzXnM/pE8AwLer\"]"));
        assert!(pgn.contains("[Result \"1-0\"]"));
        assert!(!pgn.contains("[Variant \"Standard\"]"));
        assert!(pgn.contains("[ECO \"B13\"]"));
        assert!(pgn.contains("[Opening \"Caro-Kann Defense: Exchange Variation\"]"));
        assert!(pgn.contains("[Annotator \"https://lichess.org/@/Santicastrom\"]"));
        assert!(pgn.contains("[UTCDate \"2021.07.20\"]"));
        assert!(pgn.contains("[UTCTime \"16:35:48\"]"));
        assert!(pgn.contains("1. e4 c6 2. d4 d5 3. exd5 cxd5 4. Bd3 (4. c4 Nf6 5. Nc3 Nc6 6. Nf3 (6. Bg5 a6 7. Bxf6 (7. Nf3 Be6 8. Be2 (8. c5 g6 9. Bd3 Bg7) 8... g6 9. Bxf6 exf6 10. O-O Bg7 11. Qd2 h5) 7... exf6 8. cxd5 Ne7 9. Qa4+ Bd7) 6... g6 7. cxd5 Nxd5 8. Qb3 e6 9. Bb5 Bg7 10. O-O O-O 11. Bxc6 bxc6 12. Na4 Qd6 13. Re1 Rb8 14. Qd1 c5 15. Nxc5 Bb7 16. Nxb7 Rxb7 17. b3 Rc8) (4. Nf3 Nc6 5. c3 (5. c4) (5. Bb5 Qa5+ 6. Nc3 Bg4)) 4... Nc6 5. c3 (5. Nf3 Bg4 6. c3 Qc7 7. O-O e6 8. h3 Bh5 9. Re1 Bd6 10. Nbd2 Nge7 11. Nf1 h6 12. Be2 Bg6) 5... Nf6 6. Nf3 (6. Bf4 Bg4 7. Qb3 (7. f3 Bh5 8. g4 Bg6 9. Ne2) 7... Qd7 8. Nd2 e6 9. Ngf3 Bd6 10. Bxd6 Qxd6 11. O-O O-O 12. Rfe1 Bh5 13. Ne5 Qc7 14. f4 Ne7) (6. Bg5 Bg4 7. Qb3 (7. Ne2 e6 8. Qc2 Qc7 9. Ng3 Nh5) 7... e5 8. Qxb7 Bd7 9. Bxf6 gxf6 10. Bf5 Rb8) 6... Bg4 7. O-O Qb8 8. h3 Bh5 1-0"));
    }

    #[test]
    fn test_invalid_variant() {
        let input = "[Variant \"Chess960\"]
        1. e4 c6";
        let result: Result<StandardChess, PGNError> = parse_pgn(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_invalid_variants() {
        let input = "[Event \"caro kann: exchange\"]
        [Variant \"Chess960\"]
        [Site \"\"]
        1. e4 c6
        [Event \"another game\"]
        [Variant \"Standard\"]
        1. d4 d5";
        let result: Result<Vec<StandardChess>, PGNError> = parse_multiple_pgn(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_comments_in_pgn() {
        let input = "[Event \"game with comments\"]
            {First comment}
            1. e4 {A comment} (1. c4 {Another comment}) {Other one} (1. d4) e5 2. {Other} Nf3 Nc6 {And other} (2... {Another one} Nf6 3. Nxe5 {And other} d6) {And other} (2... g6) 3. Bb5 (3. Bc4) 3... {This one} a6
            {Last one}";

        let variant = parse_pgn::<StandardChess>(&input).unwrap();

        assert_eq!(variant.pgn(), "[Event \"game with comments\"]\n[Site \"\"]\n[Date \"\"]\n[Round \"\"]\n[White \"\"]\n[Black \"\"]\n[Result \"\"]\n1. e4 (1. c4) (1. d4) 1... e5 2. Nf3 Nc6 (2... Nf6 3. Nxe5 d6) (2... g6) 3. Bb5 (3. Bc4) 3... a6")
    }

    #[test]
    fn test_multiple_lines_pgn() {
        let input = "[Event \"game 1\"]
        1. e4 (1. c4) (1. d4) e5 2. Nf3 Nc6 (2... Nf6 3. Nxe5 d6) (2... g6) 3. Bb5 (3. Bc4) 3... a6";

        let variant = parse_pgn::<StandardChess>(&input).unwrap();

        assert_eq!(variant.pgn(), "[Event \"game 1\"]\n[Site \"\"]\n[Date \"\"]\n[Round \"\"]\n[White \"\"]\n[Black \"\"]\n[Result \"\"]\n1. e4 (1. c4) (1. d4) 1... e5 2. Nf3 Nc6 (2... Nf6 3. Nxe5 d6) (2... g6) 3. Bb5 (3. Bc4) 3... a6")
    }

    #[test]
    fn test_from_fen_in_pgn() {
        let input = "[Event \"game from FEN\"]
        [FEN \"r1bqkbnr/pppppppp/n7/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 1\"]
        1. e4 e5 2. Nc3 Ne7";

        let variant = parse_pgn::<StandardChess>(&input).unwrap();

        assert_eq!(
            variant.fen(),
            "r1bqkb1r/ppppnppp/n7/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 2 3"
        );
    }

    #[test]
    fn test_parse_pgn_file() {
        let input = read_file("data/standard/ex3.pgn").unwrap();

        let variants: Vec<StandardChess> = parse_multiple_pgn(&input).unwrap();

        assert_eq!(variants.len(), 20);

        for variant in variants {
            println!("{}\n", variant.pgn());
        }
    }
}
