use std::{cell::RefCell, fmt::Display, rc::Rc};

use super::Position;

/// A struct representing a PGN line or variation
/// Its also a tree node that contains a list of child nodes, the parent node,
/// the move number and the move itself
///
#[derive(Debug, Clone)]
pub struct PgnLine<T: PartialEq + Clone + Display> {
    pub lines: Vec<Rc<RefCell<PgnLine<T>>>>,
    pub parent: Option<Rc<RefCell<PgnLine<T>>>>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub en_passant: Option<Position>,
    pub castling_rights: u8,
    pub mov: T,
}

impl<T: PartialEq + Clone + Display> PartialEq for PgnLine<T> {
    /// Compares two PgnLine structs
    /// Two PgnLine structs are equal if their moves are equal
    ///
    /// # Arguments
    /// * `other`: The other PgnLine struct
    ///
    /// # Returns
    /// A boolean indicating if the two PgnLine structs are equal
    ///
    fn eq(&self, other: &Self) -> bool {
        self.mov == other.mov
    }
}

/// A struct representing a PGN tree
/// It contains the game metadata and a list of lines
/// The current line is the move node that is currently being checked
///
#[derive(Debug, Clone)]
pub struct PgnTree<T: PartialEq + Clone + Display> {
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
    pub termination: Option<String>,
    lines: Vec<Rc<RefCell<PgnLine<T>>>>,
    current_line: Option<Rc<RefCell<PgnLine<T>>>>,
}

impl<T: PartialEq + Clone + Display> Default for PgnTree<T> {
    /// Creates a new PgnTree with no metadata and an empty list of lines
    ///
    /// # Returns
    /// A new PgnTree
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::pgn::PgnTree;
    /// use chess_lib::constants::Move;
    ///
    /// let tree: PgnTree<Move> = PgnTree::default();
    /// ```
    ///
    fn default() -> PgnTree<T> {
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
            termination: None,
            lines: Vec::new(),
            current_line: None,
        }
    }
}

impl<T: PartialEq + Clone + Display> PgnTree<T> {
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
    /// use chess_lib::constants::Move;
    ///
    /// let tree: PgnTree<Move> = PgnTree::new(
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
    ///    Some("Termination".to_string()),
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
        termination: Option<String>,
    ) -> PgnTree<T> {
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
            termination,
            lines: Vec::new(),
            current_line: None,
        }
    }

    /// Adds a move to the current line
    ///
    /// # Arguments
    /// * `mov`: The move to add
    /// * `halfmove_clock`: The halfmove clock
    /// * `fullmove_number`: The fullmove number
    /// * `en_passant`: The en passant position
    /// * `castling_rights`: The castling rights
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::{pgn::PgnTree, Move, MoveType, PieceType, Color, Position};
    /// use chess_lib::logic::Piece;
    ///
    /// let mut pgn_tree: PgnTree<Move> = PgnTree::default();
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
    ///     (false, false),
    ///     false,
    ///     false,
    /// );
    /// pgn_tree.add_move(mov.clone(), 0, 0, None, 0);
    /// assert_eq!(mov, pgn_tree.get_move().unwrap());
    /// ```
    ///
    pub fn add_move(
        &mut self,
        mov: T,
        halfmove_clock: u32,
        fullmove_number: u32,
        en_passant: Option<Position>,
        castling_rights: u8,
    ) {
        if let Some(current_line) = &self.current_line {
            let new_line = Rc::new(RefCell::new(PgnLine {
                lines: Vec::new(),
                parent: Some(Rc::clone(&current_line)),
                halfmove_clock,
                fullmove_number,
                en_passant,
                castling_rights,
                mov,
            }));
            if !current_line.as_ref().borrow_mut().lines.contains(&new_line) {
                current_line
                    .as_ref()
                    .borrow_mut()
                    .lines
                    .push(Rc::clone(&new_line));
            }
            self.current_line = Some(new_line);
        } else {
            let new_line = Rc::new(RefCell::new(PgnLine {
                lines: Vec::new(),
                parent: None,
                halfmove_clock,
                fullmove_number,
                en_passant,
                castling_rights,
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
    ///     (false, false),
    ///     false,
    ///     false,
    /// ), 0, 0, None, 0);
    /// tree.rm_move();
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
    ///     (false, false),
    ///     false,
    ///     false,
    /// );
    /// tree.add_move(mov.clone(), 0, 0, None, 0);
    /// assert_eq!(tree.get_move(), Some(mov));
    /// ```
    ///
    pub fn get_move(&self) -> Option<T> {
        Some(self.current_line.as_ref()?.borrow().mov.clone())
    }

    /// Returns the move info
    ///
    /// # Returns
    /// A tuple containing the halfmove clock, the fullmove number, the en passant position and the castling rights
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
    ///     (false, false),
    ///     false,
    ///     false,
    /// );
    /// tree.add_move(mov.clone(), 0, 0, None, 0);
    /// assert_eq!(tree.get_prev_move_info(), (0, 0, None, 0));
    /// ```
    ///
    pub fn get_prev_move_info(&self) -> (u32, u32, Option<Position>, u8) {
        let current_line = self
            .current_line
            .as_ref()
            .unwrap_or_else(|| {
                panic!("No current line found. Please add a move before calling this method")
            })
            .borrow();
        (
            current_line.halfmove_clock,
            current_line.fullmove_number,
            current_line.en_passant,
            current_line.castling_rights,
        )
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
    ///     (false, false),
    ///     false,
    ///     false,
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
    ///     (false, false),
    ///     false,
    ///     false,
    /// );
    /// pgn_tree.add_move(mov1.clone(), 0, 0, None, 0);
    /// pgn_tree.add_move(mov2.clone(), 0, 0, None, 0);
    ///
    /// assert_eq!(mov2, pgn_tree.get_move().unwrap());
    /// assert_eq!(mov1, pgn_tree.prev_move().unwrap());
    /// assert_eq!(mov2, pgn_tree.next_move().unwrap());
    /// ```
    ///
    pub fn next_move(&mut self) -> Option<T> {
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
    ///     (false, false),
    ///     false,
    ///     false,
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
    ///     (false, false),
    ///     false,
    ///     false,
    /// );
    /// pgn_tree.add_move(mov1.clone(), 0, 0, None, 0);
    /// pgn_tree.prev_move();
    /// pgn_tree.add_move(mov2.clone(), 0, 0, None, 0);
    ///
    /// pgn_tree.prev_move();
    /// assert_eq!(mov1, pgn_tree.next_move().unwrap());
    ///
    /// pgn_tree.prev_move();
    /// assert_eq!(mov2, pgn_tree.next_move_variant(1).unwrap());
    /// ```
    ///
    pub fn next_move_variant(&mut self, variant: u32) -> Option<T> {
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

    /// Returns all the next moves
    ///
    /// # Returns
    /// All the next moves
    ///
    /// # Example
    /// ```
    /// use chess_lib::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position};
    /// use chess_lib::logic::Piece;
    ///
    /// let mut pgn_tree = PgnTree::default();
    /// let mov1 = Move::new(
    ///     Piece::new(Color::White, PieceType::Pawn),
    ///     Position::from_string("e4"),
    ///     Position::from_string("e2"),
    ///     MoveType::Normal {
    ///         capture: false,
    ///         promotion: None,
    ///     },
    ///     None,
    ///     None,
    ///     (false, false),
    ///     false,
    ///     false,
    /// );
    ///
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
    ///     (false, false),
    ///     false,
    ///     false,
    /// );
    ///
    /// pgn_tree.add_move(mov1.clone(), 0, 0, None, 0);
    /// pgn_tree.prev_move();
    /// pgn_tree.add_move(mov2.clone(), 0, 0, None, 0);
    /// pgn_tree.prev_move();
    ///
    /// assert_eq!(vec![mov1.clone(), mov2.clone()], pgn_tree.all_next_moves());
    /// ```
    ///
    pub fn all_next_moves(&self) -> Vec<T> {
        let mut moves = Vec::new();
        if let Some(current_line) = &self.current_line {
            for line in current_line.borrow().lines.iter() {
                moves.push(line.borrow().mov.clone());
            }
        } else {
            for line in self.lines.iter() {
                moves.push(line.borrow().mov.clone());
            }
        }
        moves
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
    ///     (false, false),
    ///     false,
    ///     false,
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
    ///     (false, false),
    ///     false,
    ///     false,
    /// );
    /// pgn_tree.add_move(mov1.clone(), 0, 0, None, 0);
    /// pgn_tree.add_move(mov2.clone(), 0, 0, None, 0);
    ///
    /// assert_eq!(mov2, pgn_tree.get_move().unwrap());
    /// assert_eq!(mov1, pgn_tree.prev_move().unwrap());
    /// ```
    ///
    pub fn prev_move(&mut self) -> Option<T> {
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

    pub fn pgn(&self) -> String {
        let mut pgn = String::new();
        pgn.push_str(&self.pgn_header());
        pgn.push_str(&self.pgn_moves());
        pgn
    }

    /// Returns the PGN header
    ///
    /// # Returns
    /// The PGN header
    ///
    fn pgn_header(&self) -> String {
        let mut header = String::new();
        if let Some(event) = &self.event {
            header.push_str(&format!("[Event \"{}\"]\n", event));
        }
        if let Some(site) = &self.site {
            header.push_str(&format!("[Site \"{}\"]\n", site));
        }
        if let Some(date) = &self.date {
            header.push_str(&format!("[Date \"{}\"]\n", date));
        }
        if let Some(round) = &self.round {
            header.push_str(&format!("[Round \"{}\"]\n", round));
        }
        if let Some(white) = &self.white {
            header.push_str(&format!("[White \"{}\"]\n", white));
        }
        if let Some(black) = &self.black {
            header.push_str(&format!("[Black \"{}\"]\n", black));
        }
        if let Some(result) = &self.result {
            header.push_str(&format!("[Result \"{}\"]\n", result));
        }
        if let Some(white_elo) = &self.white_elo {
            header.push_str(&format!("[WhiteElo \"{}\"]\n", white_elo));
        }
        if let Some(black_elo) = &self.black_elo {
            header.push_str(&format!("[BlackElo \"{}\"]\n", black_elo));
        }
        if let Some(time_control) = &self.time_control {
            header.push_str(&format!("[TimeControl \"{}\"]\n", time_control));
        }
        if let Some(variant) = &self.variant {
            header.push_str(&format!("[Variant \"{}\"]\n", variant));
        }
        header
    }

    fn pgn_moves(&self) -> String {
        let mut pgn = String::new();

        if self.lines.is_empty() {
            return pgn;
        }

        let line = self.lines[0].as_ref().borrow();
        pgn.push_str(&format!("1. {}", line.mov));

        for next in self.lines.iter().skip(1) {
            pgn.push_str(&format!(
                " {}",
                self.pgn_line_moves(Rc::clone(next), 1, true)
            ));
        }

        pgn.push_str(&format!(
            " {}",
            self.pgn_line_moves(Rc::clone(&self.lines[0]), 2, false)
        ));

        pgn
    }

    fn pgn_line_moves(
        &self,
        line: Rc<RefCell<PgnLine<T>>>,
        move_number: u32,
        secondary: bool,
    ) -> String {
        let mut pgn = String::new();

        let mut tmp_move_number = move_number;

        if secondary {
            if tmp_move_number % 2 == 0 {
                pgn.push_str(&format!("{}... ", tmp_move_number / 2))
            } else {
                pgn.push_str(&format!("{}. ", tmp_move_number / 2 + 1));
            };
            pgn.push_str(&format!("{} ", line.as_ref().borrow().mov));

            tmp_move_number += 1;
        }

        let mut stack = vec![line];

        while let Some(current) = stack.pop() {
            let line = current.as_ref().borrow();

            if line.lines.is_empty() {
                pgn.pop();
                continue;
            } else {
                if tmp_move_number % 2 != 0 {
                    pgn.push_str(&format!("{}. ", tmp_move_number / 2 + 1));
                };
                tmp_move_number += 1;

                let next = Rc::clone(&line.lines[0]);
                pgn.push_str(&format!("{} ", next.as_ref().borrow().mov));
                stack.push(Rc::clone(&next));

                if line.lines.len() != 1 {
                    for next in line.lines.iter().skip(1) {
                        pgn.push_str(&format!(
                            "{} ",
                            self.pgn_line_moves(Rc::clone(next), tmp_move_number - 1, true)
                        ));
                    }
                }
            }
        }

        if secondary {
            format!("({})", pgn)
        } else {
            pgn
        }
    }
}

impl<T: PartialEq + Clone + Display> Iterator for PgnTree<T> {
    type Item = T;

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
    ///     (false, false),
    ///     false,
    ///     false,
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
    ///     (false, false),
    ///     false,
    ///     false,
    /// );
    /// pgn_tree.add_move(mov1.clone(), 0, 0, None, 0);
    /// pgn_tree.add_move(mov2.clone(), 0, 0, None, 0);
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

impl<T: PartialEq + Clone + Display> DoubleEndedIterator for PgnTree<T> {
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
    ///     (false, false),
    ///     false,
    ///     false,
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
    ///     (false, false),
    ///     false,
    ///     false,
    /// );
    /// pgn_tree.add_move(mov1.clone(), 0, 0, None, 0);
    /// pgn_tree.add_move(mov2.clone(), 0, 0, None, 0);
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
            (false, false),
            false,
            false,
        );
        pgn_tree.add_move(mov.clone(), 0, 0, None, 0);

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
            (false, false),
            false,
            false,
        );
        pgn_tree.add_move(mov.clone(), 0, 0, None, 0);
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
            (false, false),
            false,
            false,
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
            (false, false),
            false,
            false,
        );
        pgn_tree.add_move(mov1.clone(), 0, 0, None, 0);
        pgn_tree.add_move(mov2.clone(), 0, 0, None, 0);

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
            (false, false),
            false,
            false,
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
            (false, false),
            false,
            false,
        );
        pgn_tree.add_move(mov1.clone(), 0, 0, None, 0);
        pgn_tree.add_move(mov2.clone(), 0, 0, None, 0);

        assert_eq!(mov2, pgn_tree.get_move().unwrap());
        assert_eq!(mov1, pgn_tree.prev_move().unwrap());
        assert_eq!(mov2, pgn_tree.next_move().unwrap());
    }

    #[test]
    fn test_all_next_moves() {
        let mut pgn_tree = PgnTree::default();
        let mov1 = Move::new(
            Piece::new(Color::White, PieceType::Pawn),
            Position::from_string("e4"),
            Position::from_string("e2"),
            MoveType::Normal {
                capture: false,
                promotion: None,
            },
            None,
            None,
            (false, false),
            false,
            false,
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
            (false, false),
            false,
            false,
        );

        pgn_tree.add_move(mov1.clone(), 0, 0, None, 0);
        pgn_tree.prev_move();
        pgn_tree.add_move(mov2.clone(), 0, 0, None, 0);
        pgn_tree.prev_move();

        assert_eq!(vec![mov1.clone(), mov2.clone()], pgn_tree.all_next_moves());
    }

    #[test]
    fn test_pgn_header() {
        let mut tree: PgnTree<Move> = PgnTree::default();
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

        assert_eq!(tree.pgn_header(), "[Event \"Event\"]\n[Site \"Site\"]\n[Date \"Date\"]\n[Round \"Round\"]\n[White \"White\"]\n[Black \"Black\"]\n[Result \"Result\"]\n[WhiteElo \"1000\"]\n[BlackElo \"1000\"]\n[TimeControl \"TimeControl\"]\n[Variant \"Variant\"]\n");
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
            (false, false),
            false,
            false,
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
            (false, false),
            false,
            false,
        );
        pgn_tree.add_move(mov1.clone(), 0, 0, None, 0);
        pgn_tree.prev_move();
        pgn_tree.add_move(mov2.clone(), 0, 0, None, 0);

        pgn_tree.prev_move();
        assert_eq!(mov1, pgn_tree.next_move().unwrap());

        pgn_tree.prev_move();
        assert_eq!(mov2, pgn_tree.next_move_variant(1).unwrap());
    }

    #[test]
    fn test_pgn() {
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
            (false, false),
            false,
            false,
        );
        let mov2 = Move::new(
            Piece::new(Color::Black, PieceType::Pawn),
            Position::from_string("e7"),
            Position::from_string("e5"),
            MoveType::Normal {
                capture: false,
                promotion: None,
            },
            None,
            None,
            (false, false),
            false,
            false,
        );
        pgn_tree.add_move(mov1.clone(), 0, 0, None, 0);
        pgn_tree.add_move(mov2.clone(), 0, 0, None, 0);

        assert_eq!(pgn_tree.pgn(), "1. e4 e5");
    }
}
