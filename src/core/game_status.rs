use std::fmt;

/// Represents the status of a chess game
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameStatus {
    /// The [Game](crate::logic::Game) is still ongoing
    InProgress,
    /// The [Game](crate::logic::Game) has ended in a draw
    Draw(DrawReason),
    /// White has won the [Game](crate::logic::Game)
    WhiteWins(WinReason),
    /// Black has won the [Game](crate::logic::Game)
    BlackWins(WinReason),
}

/// Represents the reason for a draw
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DrawReason {
    /// The [Game](crate::logic::Game) is a stalemate
    Stalemate,
    /// The [Game](crate::logic::Game) is a draw due to insufficient material
    InsufficientMaterial,
    /// The [Game](crate::logic::Game) is a draw due to threefold repetition
    ThreefoldRepetition,
    /// The [Game](crate::logic::Game) is a draw due to the fifty move rule
    FiftyMoveRule,
    /// The [Game](crate::logic::Game) is a draw due to agreement
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

/// Represents winning reasons
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WinReason {
    /// The [Game](crate::logic::Game) was won due to checkmate
    Checkmate,
    /// The [Game](crate::logic::Game) was won due to resignation
    Resignation,
    /// The [Game](crate::logic::Game) was won due to time
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_game_status_display() {
        let mut draw_reason = DrawReason::Stalemate;
        assert_eq!(draw_reason.to_string(), "Stalemate");
        draw_reason = DrawReason::InsufficientMaterial;
        assert_eq!(draw_reason.to_string(), "Insufficient material");
        draw_reason = DrawReason::ThreefoldRepetition;
        assert_eq!(draw_reason.to_string(), "Threefold repetition");
        draw_reason = DrawReason::FiftyMoveRule;
        assert_eq!(draw_reason.to_string(), "Fifty move rule");
        draw_reason = DrawReason::Agreement;
        assert_eq!(draw_reason.to_string(), "Agreement");

        let mut win_reason = WinReason::Checkmate;
        assert_eq!(win_reason.to_string(), "Checkmate");
        win_reason = WinReason::Resignation;
        assert_eq!(win_reason.to_string(), "Resignation");
        win_reason = WinReason::Time;
        assert_eq!(win_reason.to_string(), "Time");
    }
}
