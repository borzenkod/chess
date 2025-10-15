use crate::{File, Rank, Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Directions {
    pub const LEN: usize = 8;
    pub const ALL: [Directions; Self::LEN] = [
        Self::Up,
        Self::Down,
        Self::Left,
        Self::Right,
        Self::UpLeft,
        Self::UpRight,
        Self::DownLeft,
        Self::DownRight,
    ];

    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    #[must_use]
    pub const fn edge(&self) -> Bitboard {
        match self {
            Self::Up => Rank::Eighth.bitboard(),
            Self::Down => Rank::First.bitboard(),
            Self::Left => File::A.bitboard(),
            Self::Right => File::H.bitboard(),
            Self::UpLeft => {
                let up = Self::Up.edge();
                let left = Self::Left.edge();
                up.or(left)
            }
            Self::UpRight => {
                let up = Self::Up.edge();
                let right = Self::Right.edge();
                up.or(right)
            }
            Self::DownLeft => {
                let down = Self::Down.edge();
                let left = Self::Left.edge();
                down.or(left)
            }
            Self::DownRight => {
                let down = Self::Down.edge();
                let right = Self::Right.edge();
                down.or(right)
            }
        }
    }

    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        // Safety: & 7 ensures that the value is in range [0, 7]
        unsafe { core::mem::transmute(value & 7) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self::neg(&Self::EMPTY);
    pub const WHITE_SQUARES: Self = Self(0x55AA_55AA_55AA_55AA);
    pub const BLACK_SQUARES: Self = Self::neg(&Self::WHITE_SQUARES);

    #[must_use]
    pub const fn from_u64(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn neg(&self) -> Self {
        Self(!self.0)
    }

    #[must_use]
    pub const fn or(&self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    #[must_use]
    pub const fn and(&self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    #[must_use]
    pub const fn xor(&self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    #[must_use]
    pub const fn overlaps(&self, bitboard: Bitboard) -> bool {
        self.0 & bitboard.0 != 0
    }

    #[must_use]
    pub const fn first_square(&self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Some(Square::from_u8(self.0.trailing_zeros() as u8))
        }
    }

    #[must_use]
    pub const fn from_squares(sq: &[Square]) -> Self {
        let mut new = Self::EMPTY;
        let mut index = 0;
        loop {
            if index == sq.len() {
                break;
            }
            new = new.or(sq[index].bitboard());
            index += 1;
        }
        new
    }

    #[must_use]
    pub const fn popcnt(&self) -> u32 {
        self.0.count_ones()
    }

    #[must_use]
    pub const fn shift_up(&self) -> Bitboard {
        Self(self.0 << 8)
    }

    #[must_use]
    pub const fn shift_down(&self) -> Bitboard {
        Self(self.0 >> 8)
    }

    #[must_use]
    pub const fn shift_left(&self) -> Bitboard {
        Self(self.0 >> 1)
    }

    #[must_use]
    pub const fn shift_right(&self) -> Bitboard {
        Self(self.0 << 1)
    }

    #[must_use]
    pub const fn scan_forward(self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Some(Square::from_u8(self.0.trailing_zeros() as u8))
        }
    }

    #[must_use]
    pub const fn scan_backward(self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Some(Square::from_u8(63 - self.0.leading_zeros() as u8))
        }
    }

    #[must_use]
    pub const fn shift(&self, dir: Directions) -> Bitboard {
        match dir {
            Directions::Up => self.shift_up(),
            Directions::Down => self.shift_down(),
            Directions::Left => self.shift_left(),
            Directions::Right => self.shift_right(),
            Directions::UpLeft => self.shift_up().shift_left(),
            Directions::UpRight => self.shift_up().shift_right(),
            Directions::DownLeft => self.shift_down().shift_left(),
            Directions::DownRight => self.shift_down().shift_right(),
        }
    }

    #[must_use]
    pub const fn next_const(&mut self) -> Option<Square> {
        let Some(sq) = self.first_square() else {
            return None;
        };
        self.0 ^= sq.bitboard().0;
        Some(sq)
    }
}

impl core::ops::Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        self.neg()
    }
}

impl core::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}
impl core::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.or(rhs);
    }
}

impl core::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}
impl core::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.and(rhs);
    }
}

impl core::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}
impl core::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.xor(rhs);
    }
}

impl From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        Self(1 << value.as_u8())
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

#[allow(clippy::copy_iterator)]
impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_const()
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in Rank::ALL.into_iter().rev() {
            for file in File::ALL {
                let sq = Square::at(file, rank);
                if self.overlaps(sq.bitboard()) {
                    write!(f, "x")?;
                } else {
                    write!(f, "-")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, " ")
    }
}
