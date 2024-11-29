use std::collections::HashMap;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{errors::FenError, logic::Game};

#[derive(Parser)]
#[grammar = "./src/common/utils/pest/pgn.pest"]
struct PGNParser;

pub fn parse_standard_pgn(input: &str) -> Result<Game, FenError> {
    let pair = PGNParser::parse(Rule::pgn, input)
        .expect("Failed to parse PGN")
        .next()
        .unwrap();

    let mut game = Game::default();

    let mut metadata = HashMap::new();

    for record in pair.into_inner() {
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
                    println!("{}", key);
                    game.history.add_metadata(key, value);
                });
                parse_sequence(&mut game, record);
            }
            Rule::result => (),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(game)
}

fn parse_sequence(game: &mut Game, sequence: Pair<Rule>) {
    let mut num_moves = 0;
    for subsequence in sequence.into_inner() {
        match subsequence.as_rule() {
            Rule::line => {
                parse_line(game, subsequence, &mut num_moves);
            }
            Rule::white_sequence => {
                parse_white_sequence(game, subsequence, &mut num_moves);
            }
            Rule::black_sequence => {
                parse_black_sequence(game, subsequence, &mut num_moves);
            }
            Rule::COMMENT => {}
            _ => unreachable!(),
        }
    }
}

fn parse_line(game: &mut Game, line: Pair<Rule>, num_moves: &mut i32) {
    for mov_type in line.into_inner() {
        match mov_type.as_rule() {
            Rule::partial_move => {
                *num_moves += 1;

                let mut pairs = mov_type.into_inner();

                pairs.next().unwrap();
                let mov = pairs.next().unwrap().get_input();

                game.move_piece(mov).unwrap();
            }
            Rule::full_move => {
                *num_moves += 2;

                let mut pairs = mov_type.into_inner();

                pairs.next().unwrap();
                let mov1 = pairs.next().unwrap().get_input();
                let mov2 = pairs.next().unwrap().get_input();

                game.move_piece(mov1).unwrap();
                game.move_piece(mov2).unwrap();
            }
            _ => unreachable!(),
        }
    }
}

fn parse_white_sequence(game: &mut Game, white_sequence: Pair<Rule>, num_moves: &mut i32) {}

fn parse_black_sequence(game: &mut Game, black_sequence: Pair<Rule>, num_moves: &mut i32) {}

#[cfg(test)]
mod tests {
    use super::parse_standard_pgn;

    #[test]
    fn test_parse_pgn() {
        let input = "[Event \"caro kann: exchange\"]\
        [Site \"https://lichess.org/study/H6cwzXnM/pE8AwLer\"]\
        [Result \"\"]\
        [Variant \"Standard\"]\
        [ECO \"B13\"]\
        [Opening \"Caro-Kann Defense: Exchange Variation\"]\
        [Annotator \"https://lichess.org/@/Santicastrom\"]\
        [UTCDate \"2021.07.20\"]\
        [UTCTime \"16:35:48\"]\
        1. e4 c6 2. d4 d5 3. exd5 cxd5 { [%cal Gc2c4,Gg1f3,Gc2c3,Gc1f4,Yf1d3] } 4. Bd3 (4. c4 Nf6 { [%cal Gg1f3,Gc4d5,Gc4c5,Yb1c3] } 5. Nc3 Nc6 { [%cal Gc4d5,Gc1f4,Gc1e3,Rg1f3,Bc1g5] } 6. Nf3 (6. Bg5 a6 7. Bxf6 (7. Nf3 Be6 { [%cal Yf1e2,Gc4c5] } 8. Be2 (8. c5 g6 9. Bd3 Bg7) 8... g6 { [%cal Gb2b3,Yg5f6,Ge1g1] } 9. Bxf6 exf6 { [%cal Gc4c5,Ye1g1] } 10. O-O Bg7 { [%cal Yd1d2,Gc4c5] } 11. Qd2 h5 { [%cal Rb2b3] }) 7... exf6 8. cxd5 Ne7 { [%cal Rd1a4] } 9. Qa4+ Bd7) 6... g6 { [%cal Gc1g5,Gf1e2,Gc4c5,Yc4d5,Gh2h3]} 7. cxd5 Nxd5 { [%csl Gd1,Gb3][%cal Gd1b3,Gf1c4,Gf1b5,Gf1e2,Gd1d3] } 8. Qb3 e6 { [%csl Gf1,Gb5][%cal Gf1b5,Gc1g5,Gf1c4,Gc3d5,Gf1e2] } 9. Bb5 Bg7 { [%csl Ge1,Gg1][%cal Ge1g1,Gc1g5,Gb5c6,Gc3d5,Gf3e5,Gb3a3] } 10. O-O O-O { [%csl Gb5,Gc6][%cal Gb5c6,Gf1d1,Gc3d5,Gc1g5] } 11. Bxc6 bxc6 { [%csl Gc3,Ga4][%cal Gc3a4,Gf1e1,Gc3e4,Gf1d1,Gc1d2,Gc1g5] } 12. Na4 Qd6 { [%csl Gf1,Ge1][%cal Gf1e1,Gc1d2,Gb3d1,Ga2a3,Ga4c5] } 13. Re1 Rb8 { [%csl Gb3,Gd1][%cal Gb3d1,Gb3c2,Gb3d3] } 14. Qd1 c5 15. Nxc5 Bb7 { [%csl Gc5,Gb7][%cal Gc5b7,Gc5e4] } 16. Nxb7 Rxb7 { [%csl Gb2,Gb3][%cal Gb2b3,Gh2h4,Ge1e2] } 17. b3 Rc8 { [%cal Gf3e5] }) (4. Nf3 Nc6 { [%cal Bc2c3,Rc2c4,Gf3e5,Gf1b5,Gf1e2,Gc1f4,Gh2h3,Gb1d2,Gb1c3,Gb2b3] } 5. c3 (5. c4) (5. Bb5 Qa5+ 6. Nc3 Bg4 { [%cal Rh2h3,Bc1d2] })) 4... Nc6 { [%cal Gg1f3,Yc2c3,Gg1e2,Ga2a3,Gc1f4] } 5. c3 (5. Nf3 Bg4 { [%cal Yc2c3,Gc1e3,Ge1g1,Gb1d2] } 6. c3 Qc7 { [%cal Rb1d2,Be1g1,Gh2h3,Gc1e3,Gc1g5,Gd1b3,Gb1a3] } 7. O-O e6 { [%cal Rb1d2,Bh2h3,Gf1e1,Gc1e3,Gc1g5,Gb1a3] } 8. h3 Bh5 { [%cal Yf1e1,Gc1e3,Gb2b4,Gb1d2,Ga2a4] } 9. Re1 Bd6 { [%cal Yb1d2,Gc1g5,Gb2b4,Gc1e3,Gb1a3] } 10. Nbd2 Nge7 { [%cal Ra2a4,Bd2f1,Gd2b3,Gb2b3] } 11. Nf1 h6 { [%cal Yd3e2] } 12. Be2 Bg6 { [%cal Re2d3,Bf3h4] }) 5... Nf6 { [%cal Gc1g5,Gg1e2,Gh2h3,Rc1f4,Bg1f3] } 6. Nf3 (6. Bf4 Bg4 { [%cal Rd1c2,Bd1b3,Gg1f3,Gg1e2,Gf2f3,Gd1a4,Gd3e2] } 7. Qb3 (7. f3 Bh5 8. g4 Bg6 9. Ne2) 7... Qd7 { [%cal Yb1d2,Gh2h3] } 8. Nd2 e6 { [%cal Yg1f3,Gh2h3] } 9. Ngf3 Bd6 { [%cal Yf4d6,Gf3e5,Gf4e5,Gf4g3,Ge1g1,Gf4g5] } 10. Bxd6 Qxd6 { [%cal Ye1g1,Gb3b7,Gh2h3] } 11. O-O O-O { [%cal Yf1e1,Ga1e1] } 12. Rfe1 Bh5 { [%cal Bf3e5,Rh2h3] } 13. Ne5 Qc7 { [%cal Bf2f4,Rb3c2,Gh2h3] } 14. f4 Ne7 { [%cal Gb3c2,Ra2a3,Gg2g3] }) (6. Bg5 Bg4 { [%cal Bd1b3,Rg1e2,Gg1f3] } 7. Qb3 (7. Ne2 e6 { [%cal Yd1c2,Gd1b3] } 8. Qc2 Qc7 { [%cal Gg5f6,Gg7f6,Gd3h7,Ye2g3] } 9. Ng3 Nh5 { [%cal Yb1d2] }) 7... e5 { [%cal Rb3b7] } 8. Qxb7 Bd7 9. Bxf6 gxf6 10. Bf5 Rb8 { [%cal Rf5d7] }) 6... Bg4 { [%cal Gb1d2,Be1g1,Rh2h3] } 7. O-O Qb8 { [%cal Rh2h3] } 8. h3 Bh5 { [%cal Rg2g3,Gc1g5] } 1-0";

        let game = parse_standard_pgn(input).unwrap();
        println!("{}", game.pgn());
        assert!(false)
    }
}
