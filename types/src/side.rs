use crate::error::ChessError;

/// The enum defining the player side
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Side {
    White = 0,
    Black = 1,
}

impl Side {
    pub const LEN: usize = 2;
    pub const ALL: [Self; Self::LEN] = [Self::White, Self::Black];

    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub const fn neg(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub const fn from_u8(side: u8) -> Self {
        unsafe { std::mem::transmute(side & 1) }
    }

    pub const fn from_char(c: char) -> Result<Self, ChessError> {
        match c {
            'w' | 'W' => Ok(Self::White),
            'b' | 'B' => Ok(Self::Black),
            _ => Err(ChessError::InvalidSide),
        }
    }

    pub const fn to_char(&self) -> char {
        match self {
            Self::White => 'w',
            Self::Black => 'b',
        }
    }
}

impl core::ops::Not for Side {
    type Output = Self;
    fn not(self) -> Self::Output {
        self.neg()
    }
}

impl TryFrom<String> for Side {
    type Error = ChessError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let first = value.chars().nth(0).unwrap();
        Self::from_char(first)
    }
}

impl std::str::FromStr for Side {
    type Err = ChessError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let first = value.chars().nth(0).unwrap();
        Self::from_char(first)
    }
}

impl TryFrom<char> for Side {
    type Error = ChessError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::from_char(value)
    }
}
