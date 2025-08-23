use crate::MoveList;
use crate::PieceType;

use crate::Bitboard;
use crate::Chessboard;
use crate::Square;
use crate::get_connection_axis;
use crate::get_connection_direct;
use crate::get_rook_attacks;

#[derive(Debug)]
pub struct Rook;

impl Rook {
    super::generate!(Self::generate, PieceType::Rook);

    pub const fn generate(cb: &Chessboard, sq: Square, check: bool) -> Bitboard {
        let us = cb.side();

        let attacks = get_rook_attacks(sq, cb.occupied());
        let king_sq = cb.get_king();

        if check {
            // SAFETY: if we are in check there should be a checker piece
            let checkers_sq = unsafe { cb.checkers.scan_forward().unwrap_unchecked() };
            let defending = get_connection_direct(king_sq, checkers_sq).or(cb.checkers);

            let pinned = if sq.bitboard().overlaps(cb.pinned) {
                get_connection_axis(king_sq, sq)
            } else {
                Bitboard::FULL
            };
            attacks.and(pinned).and(defending)
        } else {
            let our = cb.colors(us);
            let pinned_axis = if sq.bitboard().overlaps(cb.pinned) {
                get_connection_axis(king_sq, sq)
            } else {
                Bitboard::FULL
            };
            attacks.and(our.neg()).and(pinned_axis)
        }
    }
}
