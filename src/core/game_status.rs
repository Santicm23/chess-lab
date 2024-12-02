use std::fmt;

/// Represents the status of a chess game
///
/// # Variants
/// * `InProgress`: The game is in progress
/// * `Draw`: The game is a draw
///     - `reason`: The reason for the draw
/// * `WhiteWins`: White wins the game
///     - `reason`: The reason for the win
/// * `BlackWins`: Black wins the game
///     - `reason`: The reason for the win
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameStatus {
    InProgress,
    Draw(DrawReason),
    WhiteWins(WinReason),
    BlackWins(WinReason),
}

/// Represents the reason for a draw
///
/// # Variants
/// * `Stalemate`: The game is a stalemate
/// * `InsufficientMaterial`: The game is a draw due to insufficient material
/// * `ThreefoldRepetition`: The game is a draw due to threefold repetition
/// * `FiftyMoveRule`: The game is a draw due to the fifty move rule
/// * `Agreement`: The game is a draw due to agreement
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DrawReason {
    Stalemate,
    InsufficientMaterial,
    ThreefoldRepetition,
    FiftyMoveRule,
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
/// # Variants
/// * `Checkmate`: The game is a win due to checkmate
/// * `Resignation`: The game is a win due to resignation
/// * `Time`: The game is a win due to time
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WinReason {
    Checkmate,
    Resignation,
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
