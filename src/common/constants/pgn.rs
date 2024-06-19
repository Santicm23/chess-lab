use std::{cell::RefCell, rc::Rc};

use super::Move;

/// A struct representing a PGN line or variation
/// Its also a tree node that contains a list of child nodes, the parent node,
/// the move number and the move itself
///
#[derive(Debug, Clone)]
pub struct PgnLine {
    pub lines: Vec<Rc<RefCell<PgnLine>>>,
    pub parent: Option<Rc<RefCell<PgnLine>>>,
    pub move_number: u32,
    pub mov: Move,
}

/// A struct representing a PGN tree
/// It contains the game metadata and a list of lines
/// The current line is the move node that is currently being checked
///
#[derive(Debug, Clone)]
pub struct PgnTree {
    pub event: Option<String>,
    pub site: Option<String>,
    pub date: Option<String>,
    pub round: Option<String>,
    pub white: Option<String>,
    pub black: Option<String>,
    pub result: Option<String>,
    pub variant: Option<String>,
    pub white_elo: Option<u32>,
    pub black_elo: Option<u32>,
    pub time_control: Option<String>,
    pub lines: Vec<Rc<RefCell<PgnLine>>>,
    pub current_line: Option<Rc<RefCell<PgnLine>>>,
}

impl Default for PgnTree {
    /// Creates a new PgnTree with no metadata and an empty list of lines
    ///
    /// # Returns
    /// A new PgnTree
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::pgn::PgnTree;
    ///
    /// let tree = PgnTree::default();
    /// ```
    ///
    fn default() -> PgnTree {
        PgnTree {
            event: None,
            site: None,
            date: None,
            round: None,
            white: None,
            black: None,
            result: None,
            variant: None,
            white_elo: None,
            black_elo: None,
            time_control: None,
            lines: Vec::new(),
            current_line: None,
        }
    }
}

impl PgnTree {
    /// Creates a new PgnTree with the provided metadata and an empty list of lines
    ///
    /// # Arguments
    /// * `event`: The event name
    /// * `site`: The site name
    /// * `date`: The date of the game
    /// * `round`: The round number
    /// * `white`: The white player name
    /// * `black`: The black player name
    /// * `result`: The result of the game
    /// * `variant`: The variant of the game
    /// * `white_elo`: The white player ELO
    /// * `black_elo`: The black player ELO
    /// * `time_control`: The time control of the game
    ///
    /// # Returns
    /// A new PgnTree
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::pgn::PgnTree;
    ///
    /// let tree = PgnTree::new(
    ///    Some("Event".to_string()),
    ///    Some("Site".to_string()),
    ///    Some("Date".to_string()),
    ///    Some("Round".to_string()),
    ///    Some("White".to_string()),
    ///    Some("Black".to_string()),
    ///    Some("Result".to_string()),
    ///    Some("Variant".to_string()),
    ///    Some(1000),
    ///    Some(1000),
    ///    Some("Time Control".to_string()),
    /// );
    /// ```
    ///
    pub fn new(
        event: Option<String>,
        site: Option<String>,
        date: Option<String>,
        round: Option<String>,
        white: Option<String>,
        black: Option<String>,
        result: Option<String>,
        variant: Option<String>,
        white_elo: Option<u32>,
        black_elo: Option<u32>,
        time_control: Option<String>,
    ) -> PgnTree {
        PgnTree {
            event,
            site,
            date,
            round,
            white,
            black,
            result,
            variant,
            white_elo,
            black_elo,
            time_control,
            lines: Vec::new(),
            current_line: None,
        }
    }

    /// Adds a move to the current line
    ///
    /// # Arguments
    /// * `mov`: The move to add
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::{pgn::PgnTree, Move, MoveType, PieceType, Color, Position};
    /// use chess_lib::logic::Piece;
    ///
    /// let mut pgn_tree = PgnTree::default();
    /// let mov = Move::new(
    ///     Piece::new(Color::Black, PieceType::Pawn),
    ///     Position::from_string("e2"),
    ///     Position::from_string("e4"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// );
    /// pgn_tree.add_move(mov.clone());
    /// assert_eq!(mov, pgn_tree.get_move().unwrap());
    /// ```
    ///
    pub fn add_move(&mut self, mov: Move) {
        if let Some(current_line) = &self.current_line {
            let new_line = Rc::new(RefCell::new(PgnLine {
                lines: Vec::new(),
                parent: Some(Rc::clone(&current_line)),
                move_number: current_line.as_ref().borrow().lines.len() as u32 + 1,
                mov,
            }));
            current_line
                .as_ref()
                .borrow_mut()
                .lines
                .push(Rc::clone(&new_line));
            self.current_line = Some(new_line);
        } else {
            let new_line = Rc::new(RefCell::new(PgnLine {
                lines: Vec::new(),
                parent: None,
                move_number: 1,
                mov,
            }));
            self.lines.push(Rc::clone(&new_line));
            self.current_line = Some(new_line);
        }
    }

    /// Removes the current line
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position};
    /// use chess_lib::logic::Piece;
    ///
    /// let mut tree = PgnTree::default();
    ///
    /// tree.add_move(Move::new(
    ///     Piece::new(Color::Black, PieceType::Pawn),
    ///     Position::from_string("e2"),
    ///     Position::from_string("e4"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// ));
    /// tree.rm_move();
    ///
    /// assert!(tree.current_line.is_none());
    /// ```
    ///
    pub fn rm_move(&mut self) {
        if let None = &self.current_line {
            return;
        }

        let current_line = self.current_line.take().unwrap();
        let current_line_borrowed = current_line.borrow();

        if current_line_borrowed.parent.is_none() {
            return;
        }

        let parent = Rc::clone(&current_line_borrowed.parent.as_ref().unwrap());
        let index = parent
            .borrow()
            .lines
            .iter()
            .position(|x| Rc::ptr_eq(x, &self.current_line.as_ref().unwrap()))
            .unwrap();

        parent.borrow_mut().lines.remove(index);

        self.current_line = Some(parent);
    }

    /// Returns the current move
    ///
    /// # Returns
    /// The current move
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position};
    /// use chess_lib::logic::Piece;
    ///
    /// let mut tree = PgnTree::default();
    /// let mov = Move::new(
    ///     Piece::new(Color::Black, PieceType::Pawn),
    ///     Position::from_string("e2"),
    ///     Position::from_string("e4"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// );
    /// tree.add_move(mov.clone());
    /// assert_eq!(tree.get_move(), Some(mov));
    /// ```
    ///
    pub fn get_move(&self) -> Option<Move> {
        Some(self.current_line.as_ref()?.borrow().mov.clone())
    }

    /// Returns the current move number
    ///
    /// # Returns
    /// The current move number
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position};
    /// use chess_lib::logic::Piece;
    ///
    /// let mut tree = PgnTree::default();
    /// tree.add_move(Move::new(
    ///     Piece::new(Color::Black, PieceType::Pawn),
    ///     Position::from_string("e2"),
    ///     Position::from_string("e4"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// ));
    /// assert_eq!(tree.get_move_number(), 1);
    /// ```
    ///
    pub fn get_move_number(&self) -> u32 {
        if let Some(current_line) = &self.current_line {
            current_line.borrow().move_number
        } else {
            0
        }
    }

    /// Returns the next move
    ///
    /// # Returns
    /// The next move
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position};
    /// use chess_lib::logic::Piece;
    ///
    /// let mut pgn_tree = PgnTree::default();
    /// let mov1 = Move::new(
    ///     Piece::new(Color::Black, PieceType::Pawn),
    ///     Position::from_string("e2"),
    ///     Position::from_string("e4"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// );
    /// let mov2 = Move::new(
    ///     Piece::new(Color::White, PieceType::Pawn),
    ///     Position::from_string("e7"),
    ///     Position::from_string("e5"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// );
    /// pgn_tree.add_move(mov1.clone());
    /// pgn_tree.add_move(mov2.clone());
    ///
    /// assert_eq!(mov2, pgn_tree.get_move().unwrap());
    /// assert_eq!(mov1, pgn_tree.prev_move().unwrap());
    /// assert_eq!(mov2, pgn_tree.next_move().unwrap());
    /// ```
    ///
    pub fn next_move(&mut self) -> Option<Move> {
        self.next_move_variant(0)
    }

    /// Returns the next move variant
    ///
    /// # Arguments
    /// * `variant`: The variant to get
    ///
    /// # Returns
    /// The next move variant
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position};
    /// use chess_lib::logic::Piece;
    ///
    /// let mut pgn_tree = PgnTree::default();
    /// let mov1 = Move::new(
    ///     Piece::new(Color::White, PieceType::Pawn),
    ///     Position::from_string("e2"),
    ///     Position::from_string("e4"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// );
    /// let mov2 = Move::new(
    ///     Piece::new(Color::White, PieceType::Pawn),
    ///     Position::from_string("d2"),
    ///     Position::from_string("d4"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// );
    /// pgn_tree.add_move(mov1.clone());
    /// pgn_tree.prev_move();
    /// pgn_tree.add_move(mov2.clone());
    ///
    /// pgn_tree.prev_move();
    /// assert_eq!(mov1, pgn_tree.next_move().unwrap());
    ///
    /// pgn_tree.prev_move();
    /// assert_eq!(mov2, pgn_tree.next_move_variant(1).unwrap());
    /// ```
    ///
    pub fn next_move_variant(&mut self, variant: u32) -> Option<Move> {
        if let Some(current_line) = &self.current_line {
            if current_line.borrow().lines.len() > variant as usize {
                let next_line = Rc::clone(&current_line.borrow().lines[variant as usize]);
                self.current_line = Some(Rc::clone(&next_line));
                return Some(next_line.borrow().mov.clone());
            }
        } else {
            if self.lines.len() > variant as usize {
                let next_line = Rc::clone(&self.lines[variant as usize]);
                self.current_line = Some(Rc::clone(&next_line));
                return Some(next_line.borrow().mov.clone());
            }
        }
        None
    }

    /// Returns the previous move
    ///
    /// # Returns
    /// The previous move
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position};
    /// use chess_lib::logic::Piece;
    ///
    /// let mut pgn_tree = PgnTree::default();
    /// let mov1 = Move::new(
    ///     Piece::new(Color::Black, PieceType::Pawn),
    ///     Position::from_string("e2"),
    ///     Position::from_string("e4"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// );
    /// let mov2 = Move::new(
    ///     Piece::new(Color::White, PieceType::Pawn),
    ///     Position::from_string("e7"),
    ///     Position::from_string("e5"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// );
    /// pgn_tree.add_move(mov1.clone());
    /// pgn_tree.add_move(mov2.clone());
    ///
    /// assert_eq!(mov2, pgn_tree.get_move().unwrap());
    /// assert_eq!(mov1, pgn_tree.prev_move().unwrap());
    /// ```
    ///
    pub fn prev_move(&mut self) -> Option<Move> {
        if self.current_line.is_none() || self.current_line.as_ref()?.borrow().parent.is_none() {
            self.current_line = None;
            return None;
        }

        let parent = Rc::clone(
            &self
                .current_line
                .as_ref()?
                .borrow()
                .parent
                .as_ref()
                .unwrap(),
        );
        self.current_line = Some(Rc::clone(&parent));
        Some(self.current_line.as_ref()?.borrow().mov.clone())
    }

    /// Returns the PGN header
    ///
    /// # Returns
    /// The PGN header
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::pgn::PgnTree;
    ///
    /// let mut tree = PgnTree::default();
    /// tree.event = Some("Event".to_string());
    ///
    /// assert_eq!(tree.pgn_header(), "[Event Event]\n");
    /// ```
    ///
    pub fn pgn_header(&self) -> String {
        let mut header = String::new();
        if let Some(event) = &self.event {
            header.push_str(&format!("[Event {}]\n", event));
        }
        if let Some(site) = &self.site {
            header.push_str(&format!("[Site {}]\n", site));
        }
        if let Some(date) = &self.date {
            header.push_str(&format!("[Date {}]\n", date));
        }
        if let Some(round) = &self.round {
            header.push_str(&format!("[Round {}]\n", round));
        }
        if let Some(white) = &self.white {
            header.push_str(&format!("[White {}]\n", white));
        }
        if let Some(black) = &self.black {
            header.push_str(&format!("[Black {}]\n", black));
        }
        if let Some(result) = &self.result {
            header.push_str(&format!("[Result {}]\n", result));
        }
        if let Some(white_elo) = &self.white_elo {
            header.push_str(&format!("[WhiteElo {}]\n", white_elo));
        }
        if let Some(black_elo) = &self.black_elo {
            header.push_str(&format!("[BlackElo {}]\n", black_elo));
        }
        if let Some(time_control) = &self.time_control {
            header.push_str(&format!("[TimeControl {}]\n", time_control));
        }
        if let Some(variant) = &self.variant {
            header.push_str(&format!("[Variant {}]\n", variant));
        }
        header
    }
}

impl Iterator for PgnTree {
    type Item = Move;

    /// Returns the next move
    ///
    /// # Returns
    /// The next move
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position};
    /// use chess_lib::logic::Piece;
    ///
    /// let mut pgn_tree = PgnTree::default();
    /// let mov1 = Move::new(
    ///     Piece::new(Color::Black, PieceType::Pawn),
    ///     Position::from_string("e2"),
    ///     Position::from_string("e4"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// );
    /// let mov2 = Move::new(
    ///     Piece::new(Color::White, PieceType::Pawn),
    ///     Position::from_string("e7"),
    ///     Position::from_string("e5"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// );
    /// pgn_tree.add_move(mov1.clone());
    /// pgn_tree.add_move(mov2.clone());
    ///
    /// assert_eq!(mov2, pgn_tree.get_move().unwrap());
    /// assert_eq!(mov1, pgn_tree.next_back().unwrap());
    /// assert_eq!(mov2, pgn_tree.next().unwrap());
    /// ```
    ///
    fn next(&mut self) -> Option<Self::Item> {
        self.next_move()
    }
}

impl DoubleEndedIterator for PgnTree {
    /// Returns the previous move
    ///
    /// # Returns
    /// The previous move
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position};
    /// use chess_lib::logic::Piece;
    ///
    /// let mut pgn_tree = PgnTree::default();
    /// let mov1 = Move::new(
    ///     Piece::new(Color::Black, PieceType::Pawn),
    ///     Position::from_string("e2"),
    ///     Position::from_string("e4"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// );
    /// let mov2 = Move::new(
    ///     Piece::new(Color::White, PieceType::Pawn),
    ///     Position::from_string("e7"),
    ///     Position::from_string("e5"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    /// );
    /// pgn_tree.add_move(mov1.clone());
    /// pgn_tree.add_move(mov2.clone());
    ///
    /// assert_eq!(mov2, pgn_tree.get_move().unwrap());
    /// assert_eq!(mov1, pgn_tree.next_back().unwrap());
    /// ```
    ///
    fn next_back(&mut self) -> Option<Self::Item> {
        self.prev_move()
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::pgn::PgnTree;
    use crate::constants::{Color, Move, MoveType, PieceType, Position};
    use crate::logic::Piece;

    #[test]
    fn test_add_move() {
        let mut pgn_tree = PgnTree::default();
        let mov = Move::new(
            Piece::new(Color::Black, PieceType::Pawn),
            Position::from_string("e2"),
            Position::from_string("e4"),
            MoveType::Normal {
                capture: false,
                promotion: None,
            },
            None,
            None,
        );
        pgn_tree.add_move(mov.clone());

        assert_eq!(mov, pgn_tree.get_move().unwrap());
    }

    #[test]
    fn test_rm_move() {
        let mut pgn_tree = PgnTree::default();
        let mov = Move::new(
            Piece::new(Color::Black, PieceType::Pawn),
            Position::from_string("e2"),
            Position::from_string("e4"),
            MoveType::Normal {
                capture: false,
                promotion: None,
            },
            None,
            None,
        );
        pgn_tree.add_move(mov.clone());
        pgn_tree.rm_move();

        assert!(pgn_tree.get_move().is_none());
    }

    #[test]
    fn test_prev_move() {
        let mut pgn_tree = PgnTree::default();
        let mov1 = Move::new(
            Piece::new(Color::Black, PieceType::Pawn),
            Position::from_string("e2"),
            Position::from_string("e4"),
            MoveType::Normal {
                capture: false,
                promotion: None,
            },
            None,
            None,
        );
        let mov2 = Move::new(
            Piece::new(Color::White, PieceType::Pawn),
            Position::from_string("e7"),
            Position::from_string("e5"),
            MoveType::Normal {
                capture: false,
                promotion: None,
            },
            None,
            None,
        );
        pgn_tree.add_move(mov1.clone());
        pgn_tree.add_move(mov2.clone());

        assert_eq!(mov2, pgn_tree.get_move().unwrap());
        assert_eq!(mov1, pgn_tree.prev_move().unwrap());
        assert!(pgn_tree.prev_move().is_none());
    }

    #[test]
    fn test_next_move() {
        let mut pgn_tree = PgnTree::default();
        let mov1 = Move::new(
            Piece::new(Color::Black, PieceType::Pawn),
            Position::from_string("e2"),
            Position::from_string("e4"),
            MoveType::Normal {
                capture: false,
                promotion: None,
            },
            None,
            None,
        );
        let mov2 = Move::new(
            Piece::new(Color::White, PieceType::Pawn),
            Position::from_string("e7"),
            Position::from_string("e5"),
            MoveType::Normal {
                capture: false,
                promotion: None,
            },
            None,
            None,
        );
        pgn_tree.add_move(mov1.clone());
        pgn_tree.add_move(mov2.clone());

        assert_eq!(mov2, pgn_tree.get_move().unwrap());
        assert_eq!(mov1, pgn_tree.prev_move().unwrap());
        assert_eq!(mov2, pgn_tree.next_move().unwrap());
    }

    #[test]
    fn test_pgn_header() {
        let mut tree = PgnTree::default();
        tree.event = Some("Event".to_string());
        tree.site = Some("Site".to_string());
        tree.date = Some("Date".to_string());
        tree.round = Some("Round".to_string());
        tree.white = Some("White".to_string());
        tree.black = Some("Black".to_string());
        tree.result = Some("Result".to_string());
        tree.white_elo = Some(1000);
        tree.black_elo = Some(1000);
        tree.time_control = Some("TimeControl".to_string());
        tree.variant = Some("Variant".to_string());

        assert_eq!(tree.pgn_header(), "[Event Event]\n[Site Site]\n[Date Date]\n[Round Round]\n[White White]\n[Black Black]\n[Result Result]\n[WhiteElo 1000]\n[BlackElo 1000]\n[TimeControl TimeControl]\n[Variant Variant]\n");
    }

    #[test]
    fn test_next_variant() {
        let mut pgn_tree = PgnTree::default();
        let mov1 = Move::new(
            Piece::new(Color::White, PieceType::Pawn),
            Position::from_string("e2"),
            Position::from_string("e4"),
            MoveType::Normal {
                capture: false,
                promotion: None,
            },
            None,
            None,
        );
        let mov2 = Move::new(
            Piece::new(Color::White, PieceType::Pawn),
            Position::from_string("d2"),
            Position::from_string("d4"),
            MoveType::Normal {
                capture: false,
                promotion: None,
            },
            None,
            None,
        );
        pgn_tree.add_move(mov1.clone());
        pgn_tree.prev_move();
        pgn_tree.add_move(mov2.clone());

        pgn_tree.prev_move();
        assert_eq!(mov1, pgn_tree.next_move().unwrap());

        pgn_tree.prev_move();
        assert_eq!(mov2, pgn_tree.next_move_variant(1).unwrap());
    }
}
