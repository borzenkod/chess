use crate::{Bitboard, Castling, Move, Piece, PieceType, Rank, Side, Square};

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
    pub const fn new(side: Side, en_passant: Option<Square>) -> Self {
        Self {
            moves: [Moves::EMPTY; 18],
            side,
            en_passant,
            len: 0,
            promotion: 0,
        }
    }

    pub const fn default_const() -> Self {
        Self::new(Side::White, None)
    }

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

    pub const fn next_const(&mut self) -> Option<Move> {
        if self.is_empty() {
            return None;
        }

        self.next_const_unchecked()
    }

    const fn next_const_unchecked(&mut self) -> Option<Move> {
        let mut moves = unsafe { self.last().copied().unwrap_unchecked() };

        if moves.bitboard.is_empty() {
            let m = self.pop();
            if m.is_some() {
                return self.next_const();
            } else {
                return None;
            }
        }

        let from = moves.from;
        let to = unsafe { moves.bitboard.scan_forward().unwrap_unchecked() };
        let piece = moves.piece;

        let m = self.get_move_unchecked(from, to, piece);
        if self.promotion == 0 {
            moves.bitboard = moves.bitboard.xor(to.bitboard());
        }
        unsafe {
            *self.last_mut().unwrap_unchecked() = moves;
        }
        Some(m)
    }

    const fn get_move_unchecked(&mut self, from: Square, to: Square, piece: Piece) -> Move {
        let us = self.side;
        if piece.piece_type().as_u8() == PieceType::Pawn.as_u8() {
            let end_rank = match us {
                Side::White => Rank::Eighth,
                Side::Black => Rank::First,
            };

            if to.rank().as_u8() == end_rank.as_u8() {
                if self.promotion == 0 || self.promotion > 4 {
                    self.promotion = 1;
                }
                match self.make_promotion(from, to) {
                    Ok(m) => m,
                    Err(_b) => {
                        unreachable!()
                    }
                }
            } else {
                match self.en_passant {
                    Some(en_sq) if en_sq.as_u8() == to.as_u8() => Move::EnPassant { from, to },
                    _ => Move::Standart { piece, from, to },
                }
            }
        } else if piece.piece_type().as_u8() == PieceType::King.as_u8() {
            let (castle_start, ks_end, qs_end) = match us {
                Side::White => (Square::E1, Square::G1, Square::C1),
                Side::Black => (Square::E8, Square::G8, Square::C8),
            };
            if from.as_u8() == castle_start.as_u8() && to.as_u8() == ks_end.as_u8() {
                Move::Castling {
                    from,
                    to,
                    castling: Castling::KingSide,
                }
            } else if from.as_u8() == castle_start.as_u8() && to.as_u8() == qs_end.as_u8() {
                Move::Castling {
                    from,
                    to,
                    castling: Castling::QueenSide,
                }
            } else {
                Move::Standart { piece, from, to }
            }
        } else {
            Move::Standart { piece, from, to }
        }
    }

    const fn make_promotion(&mut self, from: Square, to: Square) -> Result<Move, bool> {
        let us = self.side;
        let m = match self.promotion {
            0 => return Err(true),
            1 => Move::Promotion {
                from,
                to,
                promotion: Piece::from_side_and_type(us, PieceType::Knight),
            },
            2 => Move::Promotion {
                from,
                to,
                promotion: Piece::from_side_and_type(us, PieceType::Bishop),
            },
            3 => Move::Promotion {
                from,
                to,
                promotion: Piece::from_side_and_type(us, PieceType::Rook),
            },
            4 => Move::Promotion {
                from,
                to,
                promotion: Piece::from_side_and_type(us, PieceType::Queen),
            },

            _ => return Err(false),
        };
        self.promotion += 1;
        if self.promotion == 5 {
            self.promotion = 0;
        }
        Ok(m)
    }

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
    #[coverage(off)]
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
    #[coverage(off)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.len {
            writeln!(f, "{}", self.moves[i])?;
        }
        Ok(())
    }
}
