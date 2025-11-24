use crate::{Castling, ChessError, Side};

/// The enum defining the castling rights and the player side
///
/// Involves bit manipulation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CastlingInfo(u8);

impl CastlingInfo {
    pub const EMPTY: Self = Self::from_u8(0);
    pub const DEFAULT: Self = Self::from_u8(0b1111);

    /// Returns the [`CastlingInfo`] as u8
    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        self.0 & 15
    }

    /// Creates new [`CastlingInfo`] from u8
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        Self(value & 15)
    }

    /// Set the [`CastlingInfo`] value for a given [`Side`] and [`Castling`]
    pub const fn set(&mut self, castling: Castling, side: Side, value: bool) {
        let value = value as u8;
        match (castling, side) {
            (Castling::KingSide, Side::White) => self.0 = (self.0 & !1) | value,
            (Castling::KingSide, Side::Black) => self.0 = (self.0 & !2) | value << 1,
            (Castling::QueenSide, Side::White) => self.0 = (self.0 & !4) | value << 2,
            (Castling::QueenSide, Side::Black) => self.0 = (self.0 & !8) | value << 3,
        }
    }

    #[must_use]
    pub const fn is_set(&self, castling: Castling, side: Side) -> bool {
        let value = match (castling, side) {
            (Castling::KingSide, Side::White) => self.0 & 1,
            (Castling::KingSide, Side::Black) => self.0 & 2,
            (Castling::QueenSide, Side::White) => self.0 & 4,
            (Castling::QueenSide, Side::Black) => self.0 & 8,
        };
        value != 0
    }

    #[must_use]
    pub const fn to_chars(&self) -> (usize, [char; 4]) {
        let mut used = 0;
        let mut chars = ['\0', '\0', '\0', '\0'];

        if self.0 & 1 != 0 {
            chars[used] = 'K';
            used += 1;
        }
        if self.0 & 4 != 0 {
            chars[used] = 'Q';
            used += 1;
        }
        if self.0 & 2 != 0 {
            chars[used] = 'k';
            used += 1;
        }
        if self.0 & 8 != 0 {
            chars[used] = 'q';
            used += 1;
        }
        if used == 0 {
            used = 1;
            chars[0] = '-';
        }

        (used, chars)
    }

    #[must_use]
    pub const fn from_char(char: u8) -> Option<(Castling, Side)> {
        match char {
            b'K' => Some((Castling::KingSide, Side::White)),
            b'Q' => Some((Castling::QueenSide, Side::White)),
            b'k' => Some((Castling::KingSide, Side::Black)),
            b'q' => Some((Castling::QueenSide, Side::Black)),
            _ => None,
        }
    }

    pub const fn update(&mut self, char: u8) -> bool {
        if char == b'-' {
            return true;
        }
        match Self::from_char(char) {
            Some((c, s)) => {
                self.set(c, s, true);
                true
            }
            None => false,
        }
    }
}

impl From<u8> for CastlingInfo {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

impl core::fmt::Display for CastlingInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let chars = self.to_chars();
        for c in chars.1 {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for CastlingInfo {
    type Error = ChessError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut castling = Self::EMPTY;
        for c in value.chars() {
            match c {
                'K' => {
                    castling.set(Castling::KingSide, Side::White, true);
                }
                'Q' => {
                    castling.set(Castling::QueenSide, Side::White, true);
                }
                'k' => {
                    castling.set(Castling::KingSide, Side::Black, true);
                }
                'q' => {
                    castling.set(Castling::QueenSide, Side::Black, true);
                }
                '-' => {
                    return Ok(CastlingInfo::EMPTY);
                }
                _ => {
                    return Err(ChessError::InvalidCastlingInfo);
                }
            }
        }
        Ok(castling)
    }
}
