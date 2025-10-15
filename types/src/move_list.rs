use crate::{Bitboard, Piece, PieceType, Rank, Side, Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Moves {
    pub piece: Piece,
    pub from: Square,
    pub bitboard: Bitboard,
}

impl Moves {
    pub const EMPTY: Self = Self {
        piece: Piece::WHITE_PAWN,
        from: Square::A1,
        bitboard: Bitboard::EMPTY,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MoveList {
    pub moves: [Moves; 18],
    pub len: usize,
    pub side: Side,
    pub promotion: u8,
    pub en_passant: Option<Square>,
}

impl MoveList {
    #[must_use]
    pub const fn new(side: Side, en_passant: Option<Square>) -> Self {
        Self {
            moves: [Moves::EMPTY; 18],
            side,
            en_passant,
            len: 0,
            promotion: 0,
        }
    }

    #[must_use]
    pub const fn default_const() -> Self {
        Self::new(Side::White, None)
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn push(&mut self, m: Moves) {
        if !m.bitboard.is_empty() {
            self.moves[self.len] = m;
            self.len += 1;
        }
    }

    pub const fn pop(&mut self) -> Option<Moves> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let m = self.moves[self.len];
        Some(m)
    }

    #[must_use]
    pub const fn last(&self) -> Option<&Moves> {
        if self.len == 0 {
            return None;
        }
        Some(&self.moves[self.len - 1])
    }

    pub const fn last_mut(&mut self) -> Option<&mut Moves> {
        if self.len == 0 {
            return None;
        }
        Some(&mut self.moves[self.len - 1])
    }


    #[must_use]
    pub const fn count(&self) -> usize {
        let mut count = 0;

        let mut i = 0;
        loop {
            if i == self.len {
                break;
            }

            let m = &self.moves[i];

            if let PieceType::Pawn = m.piece.piece_type() {
                let promotion = match m.piece.side() {
                    crate::Side::White => Rank::Eighth,
                    crate::Side::Black => Rank::First,
                };
                let promotion = promotion.bitboard();
                let promotions = m.bitboard.and(promotion).popcnt();

                let normal = m.bitboard.and(promotion.neg()).popcnt();

                count += normal as usize;
                count += (promotions * 4) as usize;
            } else {
                count += m.bitboard.popcnt() as usize;
            }

            i += 1;
        }

        count
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self::default_const()
    }
}

impl std::fmt::Display for Moves {
    #[cfg_attr(feature="nightly", coverage(off))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "-- Moves for {{ p: {:?}, from: {:?} }}",
            self.piece, self.from
        )?;
        writeln!(f, "{}", self.bitboard)
    }
}

impl std::fmt::Display for MoveList {
    #[cfg_attr(feature="nightly", coverage(off))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.len {
            writeln!(f, "{}", self.moves[i])?;
        }
        Ok(())
    }
}
