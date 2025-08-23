use crate::MoveList;
use crate::PieceType;

use crate::{Bitboard, Chessboard, Square, get_connection_direct, get_knight_attacks};

#[derive(Debug)]
pub struct Knight;

impl Knight {
    super::generate!(Self::generate, PieceType::Knight);

    pub const fn generate(cb: &Chessboard, sq: Square, check: bool) -> Bitboard {
        let us = cb.side();

        if cb.pinned.overlaps(sq.bitboard()) {
            return Bitboard::EMPTY;
        }

        let attacks = get_knight_attacks(sq);

        if check {
            let king_sq = cb.get_king();
            // SAFETY: if we are in check there should be a checker piece
            let checker_sq = unsafe { cb.checkers.scan_forward().unwrap_unchecked() };
            let defending = get_connection_direct(king_sq, checker_sq).or(cb.checkers);
            attacks.and(defending)
        } else {
            let our = cb.colors(us);
            attacks.and(our.neg())
        }
    }
}
