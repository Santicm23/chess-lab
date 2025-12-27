use std::{collections::HashMap, fmt};

use crate::errors::MoveInfoError;

use super::{GameStatus, Piece, PieceType, Position};

/// Represents the type of a [Move]
///
#[derive(Debug, Clone, PartialEq)]
pub enum MoveType {
    /// A normal move
    Normal {
        /// Whether the move is a capture
        capture: bool,
        /// The [PieceType] to promote to
        promotion: Option<PieceType>,
    },
    /// A castle move
    Castle {
        /// The side of the board to castle on
        side: CastleType,
    },
    /// An en passant move
    EnPassant,
}

/// Represents the side of the board to castle on
///
#[derive(Debug, Clone, PartialEq)]
pub enum CastleType {
    /// The king side
    KingSide,
    /// The queen side
    QueenSide,
}

/// Represents a move in a chess game
///
#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    /// The [Piece] that is moving
    pub piece: Piece,
    /// The [Position] the piece is moving from
    pub from: Position,
    /// The [Position] the piece is moving to
    pub to: Position,
    /// The [type](Move) of the move
    pub move_type: MoveType,
    /// The type of the piece that is captured, if any
    pub captured_piece: Option<PieceType>,
    /// The position of the rook, if the move is a castle
    pub rook_from: Option<Position>,
    /// A tuple of booleans representing the ambiguity of the move
    pub ambiguity: (bool, bool),
    /// Whether the move puts the opponent in check
    pub check: bool,
    /// Whether the move puts the opponent in checkmate
    pub checkmate: bool,
}

impl Move {
    /// Creates a new [Move]
    ///
    /// # Arguments
    /// * `piece`: The [Piece] that is moving
    /// * `from`: The [Position] the piece is moving from
    /// * `to`: The [Position] the piece is moving to
    /// * `move_type`: The [type](MoveType) of the move
    /// * `captured_piece`: The [Piece] that is captured, if any
    /// * `rook_from`: The [Position] of the rook, if the move is a castle
    /// * `ambiguity`: A tuple of booleans representing the ambiguity of the move
    /// * `check`: Whether the move puts the opponent in check
    /// * `checkmate`: Whether the move puts the opponent in checkmate
    ///
    /// # Returns
    /// A `Result<Move, MoveInfoError>`
    /// * `Ok(Move)`: The move if it is valid
    /// * `Err(MoveInfoError)`: The error if the move is invalid
    ///
    /// # Example
    /// ```
    /// use chess_lab::core::{Color, PieceType, Piece, Position, Move, MoveType};
    ///
    /// let piece = Piece::new(Color::White, PieceType::Pawn);
    /// let from = Position::new(4, 1).unwrap();
    /// let to = Position::new(4, 3).unwrap();
    /// let move_type = MoveType::Normal {
    ///     capture: false,
    ///     promotion: None,
    /// };
    /// let captured_piece = None;
    /// let rook_from = None;
    /// let ambiguity = (false, false);
    /// let mv = Move::new(
    ///     piece,
    ///     from,
    ///     to,
    ///     move_type,
    ///     captured_piece,
    ///     rook_from,
    ///     ambiguity,
    ///     false,
    ///     false
    /// ).unwrap();
    ///
    /// assert_eq!(mv.to_string(), "e4");
    /// ```
    ///
    pub fn new(
        piece: Piece,
        from: Position,
        to: Position,
        move_type: MoveType,
        captured_piece: Option<PieceType>,
        rook_from: Option<Position>,
        ambiguity: (bool, bool),
        check: bool,
        checkmate: bool,
    ) -> Result<Move, MoveInfoError> {
        let mov = Move {
            piece,
            from,
            to,
            move_type: move_type.clone(),
            captured_piece,
            rook_from,
            ambiguity,
            check,
            checkmate,
        };
        match &move_type {
            MoveType::Normal { capture, promotion } => {
                if *capture {
                    if captured_piece.is_none() {
                        return Err(MoveInfoError::new(
                            String::from(
                                "The move is a capture, but no captured piece is provided",
                            ),
                            mov,
                        ));
                    }
                } else {
                    if captured_piece.is_some() {
                        return Err(MoveInfoError::new(
                            String::from(
                                "The move is not a capture, but a captured piece is provided",
                            ),
                            mov,
                        ));
                    }
                }
                if promotion.is_some() {
                    if piece.piece_type != PieceType::Pawn {
                        return Err(MoveInfoError::new(
                            String::from("The move is a promotion, but the piece is not a pawn"),
                            mov,
                        ));
                    }
                }
            }
            MoveType::Castle { side: _ } => {
                if piece.piece_type != PieceType::King {
                    return Err(MoveInfoError::new(
                        String::from("The move is a castle, but the piece is not a king"),
                        mov,
                    ));
                }
                if rook_from.is_none() {
                    return Err(MoveInfoError::new(
                        String::from("The move is a castle, but no rook position is provided"),
                        mov,
                    ));
                }
            }
            MoveType::EnPassant => {
                if piece.piece_type != PieceType::Pawn {
                    return Err(MoveInfoError::new(
                        String::from("The move is an en passant, but the piece is not a pawn"),
                        mov,
                    ));
                }
            }
        }
        Ok(mov)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut result = String::new();
        if self.piece.piece_type != PieceType::Pawn {
            result.push(self.piece.piece_type.to_char());
        }
        match &self.move_type {
            MoveType::Castle { side } => {
                result = match side {
                    CastleType::KingSide => "O-O".to_string(),
                    CastleType::QueenSide => "O-O-O".to_string(),
                };
            }
            MoveType::Normal { capture, promotion } => {
                let from_string = self.from.to_string();
                if self.ambiguity.0 || (PieceType::Pawn == self.piece.piece_type && *capture) {
                    result.push(from_string.chars().nth(0).unwrap());
                }
                if self.ambiguity.1 {
                    result.push(from_string.chars().nth(1).unwrap());
                }
                if *capture {
                    result.push('x');
                }
                result.push_str(&self.to.to_string());
                if let Some(promotion) = promotion {
                    result.push('=');
                    result.push(promotion.to_char());
                }
            }
            MoveType::EnPassant => {
                result.push_str(&self.from.to_string());
                result.push('x');
                result.push_str(&self.to.to_string());
            }
        }
        if self.checkmate {
            result.push('#');
        } else if self.check {
            result.push('+');
        }

        write!(f, "{}", result)
    }
}

/// Represents the information of a [Move]
///
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MoveInfo {
    /// The number of halfmoves since the last capture or pawn move
    pub halfmove_clock: u32,
    /// The number of fullmoves
    pub fullmove_number: u32,
    /// The en passant target square
    pub en_passant: Option<Position>,
    /// The castling rights
    pub castling_rights: u8,
    /// The status of the [Game](crate::logic::Game)
    pub game_status: GameStatus,
    /// A map of previous board positions and their occurrence counts
    pub prev_positions: HashMap<String, u32>,
}

impl MoveInfo {
    /// Creates a new [MoveInfo]
    ///
    /// # Arguments
    /// * `halfmove_clock`: The number of halfmoves since the last capture or pawn move
    /// * `fullmove_number`: The number of fullmoves
    /// * `en_passant`: The en passant target square
    /// * `castling_rights`: The castling rights
    /// * `game_status`: The current [GameStatus]
    ///
    /// # Example
    /// ```
    /// use chess_lab::core::{GameStatus, MoveInfo};
    /// use std::collections::HashMap;
    ///
    /// let move_info = MoveInfo::new(0, 1, None, 0, GameStatus::InProgress, HashMap::new());
    ///
    /// assert_eq!(move_info.halfmove_clock, 0);
    /// assert_eq!(move_info.fullmove_number, 1);
    /// assert_eq!(move_info.en_passant, None);
    /// assert_eq!(move_info.castling_rights, 0);
    /// assert_eq!(move_info.game_status, GameStatus::InProgress);
    /// assert_eq!(move_info.prev_positions.len(), 0);
    /// ```
    ///
    pub fn new(
        halfmove_clock: u32,
        fullmove_number: u32,
        en_passant: Option<Position>,
        castling_rights: u8,
        game_status: GameStatus,
        prev_positions: HashMap<String, u32>,
    ) -> MoveInfo {
        MoveInfo {
            halfmove_clock,
            fullmove_number,
            en_passant,
            castling_rights,
            game_status,
            prev_positions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Color, PieceType};

    #[test]
    fn test_move_display_normal() {
        let piece = Piece::new(Color::White, PieceType::Knight);
        let from = Position::new(1, 0).unwrap(); // b1
        let to = Position::new(2, 2).unwrap(); // c3
        let move_type = MoveType::Normal {
            capture: false,
            promotion: None,
        };
        let mv = Move::new(
            piece,
            from,
            to,
            move_type,
            None,
            None,
            (false, false),
            false,
            false,
        )
        .unwrap();
        assert_eq!(mv.to_string(), "Nc3");
    }

    #[test]
    fn test_move_display_capture() {
        let piece = Piece::new(Color::Black, PieceType::Bishop);
        let from = Position::new(2, 7).unwrap(); // c8
        let to = Position::new(5, 4).unwrap(); // f5
        let move_type = MoveType::Normal {
            capture: true,
            promotion: None,
        };
        let mv = Move::new(
            piece,
            from,
            to,
            move_type,
            Some(PieceType::Pawn),
            None,
            (false, false),
            true,
            false,
        )
        .unwrap();
        assert_eq!(mv.to_string(), "Bxf5+");
    }
}
