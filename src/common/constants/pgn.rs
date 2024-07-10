use std::{cell::RefCell, fmt::Display, rc::Rc};

use super::{GameStatus, Position};

#[derive(Debug, Clone)]
pub enum OptionPgnMetadata {
    // Game metadata
    Variant(String),
    TimeControl(String),
    Termination(String),

    // Player metadata
    WhiteElo(u32),
    BlackElo(u32),
    WhiteTitle(String),
    BlackTitle(String),
    WhiteUSCF(String),
    BlackUSCF(String),
    WhiteNA(String),
    BlackNA(String),
    WhiteType(String),
    BlackType(String),

    // Event metadata
    EventDate(String),
    EventSponsor(String),
    Section(String),
    Stage(String),
    Board(String),

    // Opening metadata
    Opening(String),
    Variation(String),
    SubVariation(String),
    ECO(String),
    NIC(String),

    // Time and date metadata
    Time(String),
    UTCDate(String),
    UTCTime(String),

    // Starting position metadata
    SetUp(String),
    FEN(String),

    // Other metadata
    Annotator(String),
    Mode(String),
    PlyCount(u32),
}
impl OptionPgnMetadata {
    pub fn from_string(key: &str, value: &str) -> Option<OptionPgnMetadata> {
        match key {
            "Variant" => Some(OptionPgnMetadata::Variant(value.to_string())),
            "TimeControl" => Some(OptionPgnMetadata::TimeControl(value.to_string())),
            "Termination" => Some(OptionPgnMetadata::Termination(value.to_string())),
            "WhiteElo" => Some(OptionPgnMetadata::WhiteElo(value.parse().unwrap())),
            "BlackElo" => Some(OptionPgnMetadata::BlackElo(value.parse().unwrap())),
            "WhiteTitle" => Some(OptionPgnMetadata::WhiteTitle(value.to_string())),
            "BlackTitle" => Some(OptionPgnMetadata::BlackTitle(value.to_string())),
            "WhiteUSCF" => Some(OptionPgnMetadata::WhiteUSCF(value.to_string())),
            "BlackUSCF" => Some(OptionPgnMetadata::BlackUSCF(value.to_string())),
            "WhiteNA" => Some(OptionPgnMetadata::WhiteNA(value.to_string())),
            "BlackNA" => Some(OptionPgnMetadata::BlackNA(value.to_string())),
            "WhiteType" => Some(OptionPgnMetadata::WhiteType(value.to_string())),
            "BlackType" => Some(OptionPgnMetadata::BlackType(value.to_string())),
            "EventDate" => Some(OptionPgnMetadata::EventDate(value.to_string())),
            "EventSponsor" => Some(OptionPgnMetadata::EventSponsor(value.to_string())),
            "Section" => Some(OptionPgnMetadata::Section(value.to_string())),
            "Stage" => Some(OptionPgnMetadata::Stage(value.to_string())),
            "Board" => Some(OptionPgnMetadata::Board(value.to_string())),
            "Opening" => Some(OptionPgnMetadata::Opening(value.to_string())),
            "Variation" => Some(OptionPgnMetadata::Variation(value.to_string())),
            "SubVariation" => Some(OptionPgnMetadata::SubVariation(value.to_string())),
            "ECO" => Some(OptionPgnMetadata::ECO(value.to_string())),
            "NIC" => Some(OptionPgnMetadata::NIC(value.to_string())),
            "Time" => Some(OptionPgnMetadata::Time(value.to_string())),
            "UTCDate" => Some(OptionPgnMetadata::UTCDate(value.to_string())),
            "UTCTime" => Some(OptionPgnMetadata::UTCTime(value.to_string())),
            "SetUp" => Some(OptionPgnMetadata::SetUp(value.to_string())),
            "FEN" => Some(OptionPgnMetadata::FEN(value.to_string())),
            "Annotator" => Some(OptionPgnMetadata::Annotator(value.to_string())),
            "Mode" => Some(OptionPgnMetadata::Mode(value.to_string())),
            "PlyCount" => Some(OptionPgnMetadata::PlyCount(value.parse().unwrap())),
            _ => None,
        }
    }
}

impl Display for OptionPgnMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionPgnMetadata::Variant(variant) => write!(f, "[Variant \"{}\"]", variant),
            OptionPgnMetadata::TimeControl(time_control) => {
                write!(f, "[TimeControl \"{}\"]", time_control)
            }
            OptionPgnMetadata::Termination(termination) => {
                write!(f, "[Termination \"{}\"]", termination)
            }
            OptionPgnMetadata::WhiteElo(white_elo) => write!(f, "[WhiteElo \"{}\"]", white_elo),
            OptionPgnMetadata::BlackElo(black_elo) => write!(f, "[BlackElo \"{}\"]", black_elo),
            OptionPgnMetadata::WhiteTitle(white_title) => {
                write!(f, "[WhiteTitle \"{}\"]", white_title)
            }
            OptionPgnMetadata::BlackTitle(black_title) => {
                write!(f, "[BlackTitle \"{}\"]", black_title)
            }
            OptionPgnMetadata::WhiteUSCF(white_uscf) => write!(f, "[WhiteUSCF \"{}\"]", white_uscf),
            OptionPgnMetadata::BlackUSCF(black_uscf) => write!(f, "[BlackUSCF \"{}\"]", black_uscf),
            OptionPgnMetadata::WhiteNA(white_na) => write!(f, "[WhiteNA \"{}\"]", white_na),
            OptionPgnMetadata::BlackNA(black_na) => write!(f, "[BlackNA \"{}\"]", black_na),
            OptionPgnMetadata::WhiteType(white_type) => write!(f, "[WhiteType \"{}\"]", white_type),
            OptionPgnMetadata::BlackType(black_type) => write!(f, "[BlackType \"{}\"]", black_type),
            OptionPgnMetadata::EventDate(event_date) => write!(f, "[EventDate \"{}\"]", event_date),
            OptionPgnMetadata::EventSponsor(event_sponsor) => {
                write!(f, "[EventSponsor \"{}\"]", event_sponsor)
            }
            OptionPgnMetadata::Section(section) => write!(f, "[Section \"{}\"]", section),
            OptionPgnMetadata::Stage(stage) => write!(f, "[Stage \"{}\"]", stage),
            OptionPgnMetadata::Board(board) => write!(f, "[Board \"{}\"]", board),
            OptionPgnMetadata::Opening(opening) => write!(f, "[Opening \"{}\"]", opening),
            OptionPgnMetadata::Variation(variation) => write!(f, "[Variation \"{}\"]", variation),
            OptionPgnMetadata::SubVariation(sub_variation) => {
                write!(f, "[SubVariation \"{}\"]", sub_variation)
            }
            OptionPgnMetadata::ECO(eco) => write!(f, "[ECO \"{}\"]", eco),
            OptionPgnMetadata::NIC(nic) => write!(f, "[NIC \"{}\"]", nic),
            OptionPgnMetadata::Time(time) => write!(f, "[Time \"{}\"]", time),
            OptionPgnMetadata::UTCDate(utc_date) => write!(f, "[UTCDate \"{}\"]", utc_date),
            OptionPgnMetadata::UTCTime(utc_time) => write!(f, "[UTCTime \"{}\"]", utc_time),
            OptionPgnMetadata::SetUp(set_up) => write!(f, "[SetUp \"{}\"]", set_up),
            OptionPgnMetadata::FEN(fen) => write!(f, "[FEN \"{}\"]", fen),
            OptionPgnMetadata::Annotator(annotator) => write!(f, "[Annotator \"{}\"]", annotator),
            OptionPgnMetadata::Mode(mode) => write!(f, "[Mode \"{}\"]", mode),
            OptionPgnMetadata::PlyCount(ply_count) => write!(f, "[PlyCount \"{}\"]", ply_count),
        }
    }
}

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
    pub game_status: GameStatus,
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
    pub event: String,
    pub site: String,
    pub date: String,
    pub round: String,
    pub white: String,
    pub black: String,
    pub result: String,
    pub option_metadata: Vec<OptionPgnMetadata>,
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
    /// use chess_lab::constants::pgn::PgnTree;
    /// use chess_lab::constants::Move;
    ///
    /// let tree: PgnTree<Move> = PgnTree::default();
    /// ```
    ///
    fn default() -> PgnTree<T> {
        PgnTree {
            event: "".to_string(),
            site: "".to_string(),
            date: "".to_string(),
            round: "".to_string(),
            white: "".to_string(),
            black: "".to_string(),
            result: "".to_string(),
            option_metadata: Vec::new(),
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
    /// use chess_lab::constants::pgn::PgnTree;
    /// use chess_lab::constants::Move;
    ///
    /// let tree: PgnTree<Move> = PgnTree::new(
    ///    "Event".to_string(),
    ///    "Site".to_string(),
    ///    "Date".to_string(),
    ///    "Round".to_string(),
    ///    "White".to_string(),
    ///    "Black".to_string(),
    ///    "Result".to_string(),
    ///    Vec::new(),
    /// );
    /// ```
    ///
    pub fn new(
        event: String,
        site: String,
        date: String,
        round: String,
        white: String,
        black: String,
        result: String,
        other_metadata: Vec<OptionPgnMetadata>,
    ) -> PgnTree<T> {
        PgnTree {
            event,
            site,
            date,
            round,
            white,
            black,
            result,
            option_metadata: other_metadata,
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
    /// use chess_lab::constants::{pgn::PgnTree, Move, MoveType, PieceType, Color, Position, GameStatus};
    /// use chess_lab::logic::Piece;
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
    /// pgn_tree.add_move(mov.clone(), 0, 0, None, 0, GameStatus::InProgress);
    ///
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
        game_status: GameStatus,
    ) {
        if let Some(current_line) = &self.current_line {
            let new_line = Rc::new(RefCell::new(PgnLine {
                lines: Vec::new(),
                parent: Some(Rc::clone(&current_line)),
                halfmove_clock,
                fullmove_number,
                en_passant,
                castling_rights,
                game_status,
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
                game_status,
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
    /// use chess_lab::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position, GameStatus};
    /// use chess_lab::logic::Piece;
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
    /// ), 0, 0, None, 0, GameStatus::InProgress);
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
    /// use chess_lab::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position, GameStatus};
    /// use chess_lab::logic::Piece;
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
    /// tree.add_move(mov.clone(), 0, 0, None, 0, GameStatus::InProgress);
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
    /// use chess_lab::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position, GameStatus};
    /// use chess_lab::logic::Piece;
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
    /// tree.add_move(mov.clone(), 0, 0, None, 0, GameStatus::InProgress);
    ///
    /// assert_eq!(tree.get_prev_move_info(), (0, 0, None, 0, GameStatus::InProgress));
    /// ```
    ///
    pub fn get_prev_move_info(&self) -> (u32, u32, Option<Position>, u8, GameStatus) {
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
            current_line.game_status,
        )
    }

    /// Returns the next move
    ///
    /// # Returns
    /// The next move
    ///
    /// # Examples
    /// ```
    /// use chess_lab::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position, GameStatus};
    /// use chess_lab::logic::Piece;
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
    /// pgn_tree.add_move(mov1.clone(), 0, 0, None, 0, GameStatus::InProgress);
    /// pgn_tree.add_move(mov2.clone(), 0, 0, None, 0, GameStatus::InProgress);
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
    /// use chess_lab::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position, GameStatus};
    /// use chess_lab::logic::Piece;
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
    /// pgn_tree.add_move(mov1.clone(), 0, 0, None, 0, GameStatus::InProgress);
    /// pgn_tree.prev_move();
    /// pgn_tree.add_move(mov2.clone(), 0, 0, None, 0, GameStatus::InProgress);
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
    /// use chess_lab::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position, GameStatus};
    /// use chess_lab::logic::Piece;
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
    /// pgn_tree.add_move(mov1.clone(), 0, 0, None, 0, GameStatus::InProgress);
    /// pgn_tree.prev_move();
    /// pgn_tree.add_move(mov2.clone(), 0, 0, None, 0, GameStatus::InProgress);
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
    /// use chess_lab::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position, GameStatus};
    /// use chess_lab::logic::Piece;
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
    /// pgn_tree.add_move(mov1.clone(), 0, 0, None, 0, GameStatus::InProgress);
    /// pgn_tree.add_move(mov2.clone(), 0, 0, None, 0, GameStatus::InProgress);
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
        if !self.result.is_empty() {
            pgn.push_str(&format!(" {}\n", self.result));
        }
        pgn
    }

    pub fn game_over(&mut self, game_status: GameStatus) {
        if game_status != GameStatus::InProgress {
            match game_status {
                GameStatus::WhiteWins(_) => {
                    self.result = "1-0".to_string();
                }
                GameStatus::BlackWins(_) => {
                    self.result = "0-1".to_string();
                }
                GameStatus::Draw(_) => {
                    self.result = "1/2-1/2".to_string();
                }
                _ => (),
            }
        }
    }

    /// Returns the PGN header
    ///
    /// # Returns
    /// The PGN header
    ///
    fn pgn_header(&self) -> String {
        let mut header = String::new();
        header.push_str(&format!("[Event \"{}\"]\n", self.event));
        header.push_str(&format!("[Site \"{}\"]\n", self.site));
        header.push_str(&format!("[Date \"{}\"]\n", self.date));
        header.push_str(&format!("[Round \"{}\"]\n", self.round));
        header.push_str(&format!("[White \"{}\"]\n", self.white));
        header.push_str(&format!("[Black \"{}\"]\n", self.black));
        header.push_str(&format!("[Result \"{}\"]\n", self.result));
        for metadata in self.option_metadata.iter() {
            header.push_str(&format!("{}\n", metadata));
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
    /// use chess_lab::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position, GameStatus};
    /// use chess_lab::logic::Piece;
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
    /// pgn_tree.add_move(mov1.clone(), 0, 0, None, 0, GameStatus::InProgress);
    /// pgn_tree.add_move(mov2.clone(), 0, 0, None, 0, GameStatus::InProgress);
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
    /// use chess_lab::constants::{pgn::PgnTree, Move, PieceType, MoveType, Color, Position, GameStatus};
    /// use chess_lab::logic::Piece;
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
    /// pgn_tree.add_move(mov1.clone(), 0, 0, None, 0, GameStatus::InProgress);
    /// pgn_tree.add_move(mov2.clone(), 0, 0, None, 0, GameStatus::InProgress);
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
    use crate::constants::{Color, GameStatus, Move, MoveType, PieceType, Position};
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
        pgn_tree.add_move(mov.clone(), 0, 0, None, 0, GameStatus::InProgress);

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
        pgn_tree.add_move(mov.clone(), 0, 0, None, 0, GameStatus::InProgress);
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
        pgn_tree.add_move(mov1.clone(), 0, 0, None, 0, GameStatus::InProgress);
        pgn_tree.add_move(mov2.clone(), 0, 0, None, 0, GameStatus::InProgress);

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
        pgn_tree.add_move(mov1.clone(), 0, 0, None, 0, GameStatus::InProgress);
        pgn_tree.add_move(mov2.clone(), 0, 0, None, 0, GameStatus::InProgress);

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

        pgn_tree.add_move(mov1.clone(), 0, 0, None, 0, GameStatus::InProgress);
        pgn_tree.prev_move();
        pgn_tree.add_move(mov2.clone(), 0, 0, None, 0, GameStatus::InProgress);
        pgn_tree.prev_move();

        assert_eq!(vec![mov1.clone(), mov2.clone()], pgn_tree.all_next_moves());
    }

    #[test]
    fn test_pgn_header() {
        let mut tree: PgnTree<Move> = PgnTree::default();
        tree.event = "Event".to_string();
        tree.site = "Site".to_string();
        tree.date = "Date".to_string();
        tree.round = "Round".to_string();
        tree.white = "White".to_string();
        tree.black = "Black".to_string();
        tree.result = "Result".to_string();

        assert_eq!(tree.pgn_header(), "[Event \"Event\"]\n[Site \"Site\"]\n[Date \"Date\"]\n[Round \"Round\"]\n[White \"White\"]\n[Black \"Black\"]\n[Result \"Result\"]\n");
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
        pgn_tree.add_move(mov1.clone(), 0, 0, None, 0, GameStatus::InProgress);
        pgn_tree.prev_move();
        pgn_tree.add_move(mov2.clone(), 0, 0, None, 0, GameStatus::InProgress);

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
        pgn_tree.add_move(mov1.clone(), 0, 0, None, 0, GameStatus::InProgress);
        pgn_tree.add_move(mov2.clone(), 0, 0, None, 0, GameStatus::InProgress);

        assert_eq!(pgn_tree.pgn(), "[Event \"\"]\n[Site \"\"]\n[Date \"\"]\n[Round \"\"]\n[White \"\"]\n[Black \"\"]\n[Result \"\"]\n1. e4 e5");
    }
}
