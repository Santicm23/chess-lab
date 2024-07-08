#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "pgn.pest"]
struct PGNParser;

fn parse_pgn(input: &str) {
    let pairs = PGNParser::parse(Rule::pgn, input).expect("Failed to parse PGN");
    for pair in pairs {
        println!("{:?}", pair);
    }
}
