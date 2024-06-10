/// Represents the color of a chess piece
///
/// # Variants
/// * `White`: The white color
/// * `Black`: The black color
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Gets the opposite color
    ///
    /// # Returns
    /// The color opposite to the current one
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::Color;
    ///
    /// assert_eq!(Color::White.opposite(), Color::Black);
    /// assert_eq!(Color::Black.opposite(), Color::White);
    /// ```
    ///
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// Represents the type of a chess piece
///
/// # Variants
/// * `Pawn`: A pawn
/// * `Knight`: A knight
/// * `Bishop`: A bishop
/// * `Rook`: A rook
/// * `Queen`: A queen
/// * `King`: A king
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    /// Gets the piece type from a character
    ///
    /// # Arguments
    /// * `c`: The character to convert (only valid uppercase characters)
    ///
    /// # Returns
    /// The piece type if the character is valid, otherwise `None`
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::PieceType;
    ///
    /// assert_eq!(PieceType::from_char('P'), Some(PieceType::Pawn));
    /// assert_eq!(PieceType::from_char('N'), Some(PieceType::Knight));
    /// assert_eq!(PieceType::from_char('B'), Some(PieceType::Bishop));
    /// assert_eq!(PieceType::from_char('R'), Some(PieceType::Rook));
    /// assert_eq!(PieceType::from_char('Q'), Some(PieceType::Queen));
    /// assert_eq!(PieceType::from_char('K'), Some(PieceType::King));
    ///
    /// assert_eq!(PieceType::from_char('x'), None);
    /// assert_eq!(PieceType::from_char('y'), None);
    /// assert_eq!(PieceType::from_char('p'), None);
    /// ```
    ///
    pub fn from_char(c: char) -> Option<PieceType> {
        match c {
            'P' => Some(PieceType::Pawn),
            'N' => Some(PieceType::Knight),
            'B' => Some(PieceType::Bishop),
            'R' => Some(PieceType::Rook),
            'Q' => Some(PieceType::Queen),
            'K' => Some(PieceType::King),
            _ => None,
        }
    }

    /// Gets the character representation of the piece type
    ///
    /// # Returns
    /// The character representation of the piece type
    ///
    /// # Examples
    /// ```
    /// use chess_lib::constants::PieceType;
    ///
    /// assert_eq!(PieceType::Pawn.to_char(), 'P');
    /// assert_eq!(PieceType::Knight.to_char(), 'N');
    /// assert_eq!(PieceType::Bishop.to_char(), 'B');
    /// assert_eq!(PieceType::Rook.to_char(), 'R');
    /// assert_eq!(PieceType::Queen.to_char(), 'Q');
    /// assert_eq!(PieceType::King.to_char(), 'K');
    /// ```
    ///
    pub fn to_char(&self) -> char {
        match self {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }
}

/// Represents the type of a move
///
/// # Variants
/// * `Normal`: A normal move
///     - `capture`: Whether the move is a capture
///     - `promotion`: The piece type to promote to
/// * `Castle`: A castle move
///     - `side`: The side of the board to castle on
/// * `EnPassant`: An en passant move
#[derive(Debug, Clone)]
pub enum MoveType {
    Normal {
        capture: bool,
        promotion: Option<PieceType>,
    },
    Castle {
        side: CastleType,
    },
    EnPassant,
}

/// Represents the side of the board to castle on
///
/// # Variants
/// * `KingSide`: The king side
/// * `QueenSide`: The queen side
///
#[derive(Debug, Clone)]
pub enum CastleType {
    KingSide,
    QueenSide,
}
