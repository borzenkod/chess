use crate::{Side, error::ChessError, piece};

/// The enum representing the chess pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
    pub const ALL: [Self; Self::LEN] = [
        Self::Pawn,
        Self::Rook,
        Self::Knight,
        Self::Bishop,
        Self::Queen,
        Self::King,
    ];
    pub const LEN: usize = 6;

    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        unsafe { std::mem::transmute(value & 7) }
    }

    #[must_use]
    pub const fn to_char(&self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Rook => 'r',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        }
    }
}

/// Piece type
///
/// Internally represented as unsigned byte
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Piece(u8);

impl Piece {
    pub const WHITE_PAWN: Self = Self::from_side_and_type(Side::White, PieceType::Pawn);
    pub const WHITE_ROOK: Self = Self::from_side_and_type(Side::White, PieceType::Rook);
    pub const WHITE_KNIGHT: Self = Self::from_side_and_type(Side::White, PieceType::Knight);
    pub const WHITE_BISHOP: Self = Self::from_side_and_type(Side::White, PieceType::Bishop);
    pub const WHITE_QUEEN: Self = Self::from_side_and_type(Side::White, PieceType::Queen);
    pub const WHITE_KING: Self = Self::from_side_and_type(Side::White, PieceType::King);

    pub const BLACK_PAWN: Self = Self::from_side_and_type(Side::Black, PieceType::Pawn);
    pub const BLACK_ROOK: Self = Self::from_side_and_type(Side::Black, PieceType::Rook);
    pub const BLACK_KNIGHT: Self = Self::from_side_and_type(Side::Black, PieceType::Knight);
    pub const BLACK_BISHOP: Self = Self::from_side_and_type(Side::Black, PieceType::Bishop);
    pub const BLACK_QUEEN: Self = Self::from_side_and_type(Side::Black, PieceType::Queen);
    pub const BLACK_KING: Self = Self::from_side_and_type(Side::Black, PieceType::King);

    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        self.0
    }

    #[must_use]
    /// Convert from unsigned byte to [`Piece`]
    ///
    /// # Panics
    /// This function may panic. (see [`safe_variant`])
    ///
    /// Panics if piece has invalid range
    ///
    /// [`safe_variant`]: [`Piece::from_u8_safe`]
    pub const fn from_u8(piece: u8) -> Self {
        assert!(piece <= 10, "Invalid piece");
        unsafe { std::mem::transmute(piece) }
    }

    /// Safely convert
    pub fn from_u8_safe(piece: u8) -> Result<Self, ChessError> {
        if piece > 10 {
            Err(ChessError::InvalidPiece)
        } else {
            Ok(unsafe { std::mem::transmute::<u8, piece::Piece>(piece) })
        }
    }

    pub const fn from_char(c: char) -> Option<Self> {
        let v = match c {
            'p' => (Side::Black, PieceType::Pawn),
            'P' => (Side::White, PieceType::Pawn),
            'r' => (Side::Black, PieceType::Rook),
            'R' => (Side::White, PieceType::Rook),
            'n' => (Side::Black, PieceType::Knight),
            'N' => (Side::White, PieceType::Knight),
            'b' => (Side::Black, PieceType::Bishop),
            'B' => (Side::White, PieceType::Bishop),
            'q' => (Side::Black, PieceType::Queen),
            'Q' => (Side::White, PieceType::Queen),
            'k' => (Side::Black, PieceType::King),
            'K' => (Side::White, PieceType::King),
            _ => return None,
        };
        Some(Self::from_side_and_type(v.0, v.1))
    }

    pub const fn to_char(&self) -> char {
        let mut piece = self.piece_type().to_char();

        if let Side::White = self.side() {
            piece.make_ascii_uppercase()
        }

        piece
    }

    pub const fn from_side_and_type(side: Side, piece_type: PieceType) -> Self {
        let side = side.as_u8() << 3;
        let piece_type = piece_type as u8 & 7;
        Self(side | piece_type)
    }

    pub const fn piece_type(&self) -> PieceType {
        unsafe { std::mem::transmute(self.0 & 7) }
    }

    pub const fn side(&self) -> Side {
        unsafe { std::mem::transmute((self.0 >> 3) & 1) }
    }
}

impl TryFrom<u8> for Piece {
    type Error = ChessError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8_safe(value)
    }
}

impl From<(Side, PieceType)> for Piece {
    fn from(value: (Side, PieceType)) -> Self {
        Self::from_side_and_type(value.0, value.1)
    }
}

impl From<(PieceType, Side)> for Piece {
    fn from(value: (PieceType, Side)) -> Self {
        (value.1, value.0).into()
    }
}
