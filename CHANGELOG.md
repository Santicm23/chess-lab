All notable changes to this project will be documented in this file based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
We follow the [Semantic Versioning 2.0.0](http://semver.org/) format.


## [v0.1.1](https://github.com/Santicm23/chess-lab/releases/tag/v0.1.1) - 2024-06-27

### Added
- Core chess library, supporting all the basic rules of chess
  - Constants: including Colors, basic Piece types, Move types, Move data type, Game status, Position data type, Pgn tree and piece movement validation
  - Errors: Board error and Movement error
  - Logic: which groups all the basic functionality into three main structures:
    - Piece: representing a piece with its color and type
    - Board: containing the pieces in their corresponding position, supporting FEN (without considering additional game information in it) and giving some useful methods to check wether a position is attacked by a given color
    - Game: this is the main structure that contains and uses all the other structures, with FEN support, PGN support (only can read move by move for the moment), undo and redo methods, different lines and game over validations
- All public methods include doc strings within examples to make it simplier to use

### Deprecated
- Nothing.

### Removed
- Nothing.

### Fixed
- Nothing.
