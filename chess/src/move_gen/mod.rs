mod generators;
mod tables;

pub use generators::{Bishop, King, Knight, Pawn, Queen, Rook};
pub use generators::{generate_moves, perft};
pub use tables::*;

use crate::{Castling, Chessboard, Move, MoveList, Piece, PieceType, Rank, Side, Square};

#[derive(Debug)]
pub struct MoveGen<'a> {
    cache: &'a mut MoveList,
    promotion: u8,
}

impl<'a> MoveGen<'a> {
    pub const fn new(cache: &'a mut MoveList) -> Self {
        Self {
            cache,
            promotion: 0
        }
    }

    pub const fn is_legal(&self, m: Move) -> bool {
        let (from_sq, to_bb) = match m {
            Move::Standard { from, to, .. }
            | Move::Promotion { from, to, .. }
            | Move::EnPassant { from, to, .. }
            | Move::Castling { from, to, .. } => (from, to.bitboard()),
            Move::Remove { .. } | Move::Put { .. } => return false,
        };

        let mut i = 0;
        loop {
            if i >= self.cache.len {
                break false;
            }
            if self.cache.moves[i].from.as_u8() == from_sq.as_u8()
                && self.cache.moves[i].bitboard.overlaps(to_bb)
            {
                break true;
            }
            i += 1;
        }
    }

    pub const fn from_str(&mut self, cb: &Chessboard, m: &str) -> Option<Move> {
        let bytes = m.as_bytes();
        if bytes.len() != 4 {
            return None;
        }

        let Some(start_sq) = Square::from_chars(bytes[0] as char, bytes[1] as char) else {
            return None;
        };
        let Some(end_sq) = Square::from_chars(bytes[2] as char, bytes[3] as char) else {
            return None;
        };

        self.from_squares(cb, start_sq, end_sq)
    }

    pub const fn from_squares(&mut self, cb: &Chessboard, start_sq: Square, end_sq: Square) -> Option<Move> {
        let Some(start_piece) = cb.at(start_sq) else {
            return None;
        };
        Some(self.get_move_unchecked(start_sq, end_sq, start_piece))
    }

    pub const fn next_const(&mut self) -> Option<Move> {
        if self.cache.is_empty() {
            return None;
        }

        self.next_const_unchecked()
    }

    const fn next_const_unchecked(&mut self) -> Option<Move> {
        let mut moves = unsafe { self.cache.last().copied().unwrap_unchecked() };

        if moves.bitboard.is_empty() {
            let m = self.cache.pop();
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
            *self.cache.last_mut().unwrap_unchecked() = moves;
        }
        Some(m)
    }

    const fn get_move_unchecked(&mut self, from: Square, to: Square, piece: Piece) -> Move {
        let us = self.cache.side;
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
                match self.cache.en_passant {
                    Some(en_sq) if en_sq.as_u8() == to.as_u8() => Move::EnPassant { from, to },
                    _ => Move::Standard { piece, from, to },
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
                Move::Standard { piece, from, to }
            }
        } else {
            Move::Standard { piece, from, to }
        }
    }

    const fn make_promotion(&mut self, from: Square, to: Square) -> Result<Move, bool> {
        let us = self.cache.side;
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
}
