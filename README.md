# Chess Lab
[![Crates.io](https://img.shields.io/crates/v/chess-lab.svg?logo=rust&style=flat-square)](https://crates.io/crates/chess-lab)
[![Docs.rs](https://docs.rs/chess-lab/badge.svg)](https://docs.rs/chess-lab)
[![License](https://img.shields.io/crates/l/chess-lab.svg)](https://creativecommons.org/legal-code-defined/)
[![Actions Status](https://github.com/Santicm23/chess-lab/actions/workflows/rust.yml/badge.svg)](https://github.com/Santicm23/chess-lab/actions)

## Description
Chess Lab is a comprehensive chess library written in rust that supports multiple game variants along with FEN and PGN notations. It offers a straightforward API for seamless interaction with the game, enabling developers to integrate chess functionalities into their applications effortlessly. Additionally, it provides the capability to save and retrieve multiple game lines, facilitating the management of complex game scenarios.

## Examples
### Standard Chess
```rust
use chess_lab::variants::StandardChess;

let mut game = StandardChess::default();
game.move_piece("e4").unwrap();
game.move_piece("e5").unwrap();

assert!(game.pgn().contains("1. e4 e5"));
```
### From FEN (Position-Based)
```rust
use chess_lab::core::VariantBuilder;
use chess_lab::variants::StandardChess;

let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
let game = StandardChess::from_fen(fen).unwrap();

assert_eq!(game.fen(), fen);
```
### Chess960
```rust
use chess_lab::variants::Chess960;

let game = Chess960::default();
let fen = game.fen();

assert!(fen.contains(" w KQkq - 0 1"));
```
### Read PGN File
```rust
use chess_lab::core::VariantBuilder;
use chess_lab::variants::StandardChess;

let game = StandardChess::load("data/standard/ex1.pgn").unwrap();
assert!(game.pgn().contains("1. e4"));
```

## Installation
Add the following dependency to your `Cargo.toml` file:
```toml
[dependencies]
chess-lab = "0.2.1"
```

## Project documentation
https://deepwiki.com/Santicm23/chess-lab

## Features
- [x] Playable Chess game
- [x] FEN/PGN support
- [x] Multiple game lines
- [x] Multiple variants
- [ ] Chess engine

## Variants
- [x] Standard Chess
- [x] Position-Based Chess
- [x] Chess960
- [ ] Three-Check
- [ ] Antichess
- [ ] Horde
- [ ] Duck Chess
- [ ] Racing Kings
- [ ] King of the Hill
- [ ] Crazyhouse
- [ ] Atomic

## Created by
- [Santiago Castro Muñoz](https://santicm.com)
