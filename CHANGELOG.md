All notable changes to this project will be documented in this file based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
We follow the [Semantic Versioning 2.0.0](http://semver.org/) format.


## [v0.2.1](https://github.com/Santicm23/chess-lab/releases/tag/v0.2.1) - 2026-05-02

### Added
- Added `Chess960` variant with SPID-based initialization and FEN generation.
- Added `Chess960SPIDError` for SPID parsing errors and updated documentation.

## [v0.2.0](https://github.com/Santicm23/chess-lab/releases/tag/v0.2.0) - 2026-05-01

### Added
- New `variants` module with `StandardChess` and `Chess960` wrappers, PGN/FEN helpers, and variant-specific tests.
- New `parsing` module that organizes PGN and FEN parsing (including minified FEN helpers).
- Game APIs for move history and legality (`get_last_move`, `get_legal_moves`, `get_castle_rook_pos`) plus expanded tests/doc tests.
- Board movement helpers (`can_move`, `is_attacking`) and stricter error types for board operations.

### Changed
- Game end-state handling now includes insufficient material, threefold repetition, and fifty-move rule.
- Board FEN parsing now returns `Result` with explicit `FenError` instead of panicking.
- Improved documentation and examples across core, variants, and parsing.

### Fixed
- Legal move generation and castling legality edge cases.
- PGN parsing edge cases (multi-line games, consecutive subsequences) and PGN tree removal handling.
- Pawn movement and redo logic bugs.
- Memory leak (Rc to Weak) and safer error handling by removing unwraps.

## [v0.1.1](https://github.com/Santicm23/chess-lab/releases/tag/v0.1.1) - 2024-06-27

### Added
- Core chess library, supporting all the basic rules of chess
  - Constants: including Colors, basic Piece types, Move types, Move data type, Game status, Position data type, Pgn tree and piece movement validation
  - Errors: Board error and Movement error
  - Logic: which groups all the basic functionality into three main structures:
    - Piece: representing a piece with its color and type
    - Board: containing the pieces in their corresponding position, supporting FEN (without considering additional game information in it) and giving some useful methods to check wether a position is attacked by a given color
    - Game: this is the main structure that contains and uses all the other structures, with FEN support, PGN support (only can read move by move for the moment), undo and redo methods, different lines and game over validations
- All public methods include doc strings within examples to make it simpler to use
