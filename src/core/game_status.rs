use std::fmt;

/// Represents the status of a chess game
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameStatus {
    /// The game is still ongoing
    InProgress,
    /// The game has ended in a draw
    Draw(DrawReason),
    /// White has won the game
    WhiteWins(WinReason),
    /// Black has won the game
    BlackWins(WinReason),
}

/// Represents the reason for a draw
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DrawReason {
    /// The game is a stalemate
    Stalemate,
    /// The game is a draw due to insufficient material
    InsufficientMaterial,
    /// The game is a draw due to threefold repetition
    ThreefoldRepetition,
    /// The game is a draw due to the fifty move rule
    FiftyMoveRule,
    /// The game is a draw due to agreement
    Agreement,
}

impl fmt::Display for DrawReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DrawReason::Stalemate => write!(f, "Stalemate"),
            DrawReason::InsufficientMaterial => write!(f, "Insufficient material"),
            DrawReason::ThreefoldRepetition => write!(f, "Threefold repetition"),
            DrawReason::FiftyMoveRule => write!(f, "Fifty move rule"),
            DrawReason::Agreement => write!(f, "Agreement"),
        }
    }
}

/// Represents the reason for a win
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WinReason {
    /// The game is a win due to checkmate
    Checkmate,
    /// The game is a win due to resignation
    Resignation,
    /// The game is a win due to time
    Time,
}

impl fmt::Display for WinReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WinReason::Checkmate => write!(f, "Checkmate"),
            WinReason::Resignation => write!(f, "Resignation"),
            WinReason::Time => write!(f, "Time"),
        }
    }
}
