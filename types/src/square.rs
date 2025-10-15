use crate::{Bitboard, Directions, error::ChessError, square};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rank {
    First = 0,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl Rank {
    pub const LEN: usize = 8;
    pub const ALL: [Rank; Self::LEN] = [
        Self::First,
        Self::Second,
        Self::Third,
        Self::Fourth,
        Self::Fifth,
        Self::Sixth,
        Self::Seventh,
        Self::Eighth,
    ];

    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub const fn bitboard(&self) -> Bitboard {
        const MASK: u64 = {
            let mut board = Bitboard::EMPTY;
            board = board.or(Square::A1.bitboard());
            board = board.or(Square::B1.bitboard());
            board = board.or(Square::C1.bitboard());
            board = board.or(Square::D1.bitboard());
            board = board.or(Square::E1.bitboard());
            board = board.or(Square::F1.bitboard());
            board = board.or(Square::G1.bitboard());
            board = board.or(Square::H1.bitboard());
            board.as_u64()
        };
        Bitboard::from_u64(MASK << (self.as_u8() * 8))
    }

    pub const fn to_char(&self) -> char {
        match self {
            Self::First => '1',
            Self::Second => '2',
            Self::Third => '3',
            Self::Fourth => '4',
            Self::Fifth => '5',
            Self::Sixth => '6',
            Self::Seventh => '7',
            Self::Eighth => '8',
        }
    }

    pub const fn from_u8(rank: u8) -> Self {
        if rank > 7 {
            panic!("rank must be in range 0..8");
        }
        unsafe { std::mem::transmute(rank) }
    }

    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Self::First),
            '2' => Some(Self::Second),
            '3' => Some(Self::Third),
            '4' => Some(Self::Fourth),
            '5' => Some(Self::Fifth),
            '6' => Some(Self::Sixth),
            '7' => Some(Self::Seventh),
            '8' => Some(Self::Eighth),
            _ => None,
        }
    }

    pub const fn from_u8_safe(rank: u8) -> Option<Self> {
        if rank > 7 {
            None
        } else {
            Some(unsafe { std::mem::transmute::<u8, square::Rank>(rank) })
        }
    }

    const fn shift(self, dir: Directions) -> Option<Rank> {
        match dir {
            Directions::Up => Self::from_u8_safe(self.as_u8() + 1),
            Directions::Down => Self::from_u8_safe(self.as_u8().overflowing_sub(1).0),
            Directions::Left => Some(self),
            Directions::Right => Some(self),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum File {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub const LEN: usize = 8;
    pub const ALL: [Self; Self::LEN] = [
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
        Self::H,
    ];

    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub const fn bitboard(&self) -> Bitboard {
        const MASK: u64 = {
            let mut board = Bitboard::EMPTY;
            board = board.or(Square::A1.bitboard());
            board = board.or(Square::A2.bitboard());
            board = board.or(Square::A3.bitboard());
            board = board.or(Square::A4.bitboard());
            board = board.or(Square::A5.bitboard());
            board = board.or(Square::A6.bitboard());
            board = board.or(Square::A7.bitboard());
            board = board.or(Square::A8.bitboard());
            board.as_u64()
        };

        Bitboard::from_u64(MASK << self.as_u8())
    }

    pub const fn to_char(&self) -> char {
        match self {
            Self::A => 'a',
            Self::B => 'b',
            Self::C => 'c',
            Self::D => 'd',
            Self::E => 'e',
            Self::F => 'f',
            Self::G => 'g',
            Self::H => 'h',
        }
    }

    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            'a' => Some(Self::A),
            'b' => Some(Self::B),
            'c' => Some(Self::C),
            'd' => Some(Self::D),
            'e' => Some(Self::E),
            'f' => Some(Self::F),
            'g' => Some(Self::G),
            'h' => Some(Self::H),
            _ => None,
        }
    }

    pub const fn from_u8(file: u8) -> Self {
        if file > 7 {
            panic!("file must be in range 0..8");
        }
        unsafe { std::mem::transmute(file) }
    }

    pub const fn from_u8_safe(file: u8) -> Option<Self> {
        if file > 7 {
            None
        } else {
            Some(unsafe { std::mem::transmute::<u8, square::File>(file) })
        }
    }

    const fn shift(self, dir: Directions) -> Option<Self> {
        match dir {
            Directions::Up => Some(self),
            Directions::Down => Some(self),
            Directions::Left => Self::from_u8_safe(self.as_u8().overflowing_sub(1).0),
            Directions::Right => Self::from_u8_safe(self.as_u8() + 1),
            _ => unreachable!(),
        }
    }
}

/// Square of the chessboard
///
/// Represented as a number from 0 to 63
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(type="number"))]
#[rustfmt::skip]
#[repr(u8)]
pub enum Square {
    A1 = 0, B1, C1, D1, E1, F1, G1, H1,
    A2,     B2, C2, D2, E2, F2, G2, H2,
    A3,     B3, C3, D3, E3, F3, G3, H3,
    A4,     B4, C4, D4, E4, F4, G4, H4,
    A5,     B5, C5, D5, E5, F5, G5, H5,
    A6,     B6, C6, D6, E6, F6, G6, H6,
    A7,     B7, C7, D7, E7, F7, G7, H7,
    A8,     B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub const LEN: usize = 64;
    #[rustfmt::skip]
    pub const ALL: [Square; Self::LEN] = [
        Self::A1, Self::B1, Self::C1, Self::D1, Self::E1, Self::F1, Self::G1, Self::H1,
        Self::A2, Self::B2, Self::C2, Self::D2, Self::E2, Self::F2, Self::G2, Self::H2,
        Self::A3, Self::B3, Self::C3, Self::D3, Self::E3, Self::F3, Self::G3, Self::H3,
        Self::A4, Self::B4, Self::C4, Self::D4, Self::E4, Self::F4, Self::G4, Self::H4,
        Self::A5, Self::B5, Self::C5, Self::D5, Self::E5, Self::F5, Self::G5, Self::H5,
        Self::A6, Self::B6, Self::C6, Self::D6, Self::E6, Self::F6, Self::G6, Self::H6,
        Self::A7, Self::B7, Self::C7, Self::D7, Self::E7, Self::F7, Self::G7, Self::H7,
        Self::A8, Self::B8, Self::C8, Self::D8, Self::E8, Self::F8, Self::G8, Self::H8,
    ];

    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub const fn from_u8(sq: u8) -> Self {
        unsafe { std::mem::transmute(sq & 63) }
    }

    pub const fn rank(&self) -> Rank {
        Rank::from_u8(self.as_u8() / 8)
    }

    pub const fn file(&self) -> File {
        File::from_u8(self.as_u8() % 8)
    }

    pub const fn at(file: File, rank: Rank) -> Self {
        Self::from_u8(file.as_u8() + rank.as_u8() * 8)
    }

    pub const fn bitboard(&self) -> Bitboard {
        Bitboard::from_u64(1 << self.as_u8())
    }

    const fn double_shift(&self, dir: Directions, dir2: Directions) -> Option<Self> {
        let file = match self.file().shift(dir) {
            Some(f) => f,
            None => return None,
        };
        let file = match file.shift(dir2) {
            Some(f) => f,
            None => return None,
        };
        let rank = match self.rank().shift(dir) {
            Some(r) => r,
            None => return None,
        };
        let rank = match rank.shift(dir2) {
            Some(r) => r,
            None => return None,
        };
        Some(Self::at(file, rank))
    }

    pub const fn shift(&self, dir: Directions) -> Option<Self> {
        match dir {
            Directions::UpLeft => self.double_shift(Directions::Up, Directions::Left),
            Directions::UpRight => self.double_shift(Directions::Right, Directions::Up),
            Directions::DownLeft => self.double_shift(Directions::Left, Directions::Down),
            Directions::DownRight => self.double_shift(Directions::Down, Directions::Right),

            _ => {
                let file = match self.file().shift(dir) {
                    Some(f) => f,
                    None => return None,
                };
                let rank = match self.rank().shift(dir) {
                    Some(r) => r,
                    None => return None,
                };
                Some(Self::at(file, rank))
            }
        }
    }

    pub const fn to_chars(&self) -> [char; 2] {
        [self.file().to_char(), self.rank().to_char()]
    }

    pub const fn from_chars(file: char, rank: char) -> Option<Self> {
        let file = match File::from_char(file) {
            Some(f) => f,
            None => return None,
        };
        let rank = match Rank::from_char(rank) {
            Some(r) => r,
            None => return None,
        };
        Some(Self::at(file, rank))
    }
}

impl From<u8> for Square {
    fn from(sq: u8) -> Self {
        Self::from_u8(sq)
    }
}

impl TryFrom<&str> for Square {
    type Error = ChessError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();

        let Some(file) = value.chars().nth(0) else {
            return Err(ChessError::InvalidSquare);
        };
        let file = match File::from_char(file) {
            Some(f) => f,
            None => {
                return Err(ChessError::InvalidFile);
            }
        };

        let Some(rank) = value.chars().nth(1) else {
            return Err(ChessError::InvalidSquare);
        };
        let rank = match Rank::from_char(rank) {
            Some(r) => r,
            None => {
                return Err(ChessError::InvalidRank);
            }
        };

        Ok(Self::at(file, rank))
    }
}
