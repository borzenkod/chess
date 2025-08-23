use crate::PieceType;

use crate::MoveList;
use crate::{
    Bitboard, Chessboard, Square, get_bishop_attacks, get_connection_axis, get_connection_direct,
};

#[derive(Debug)]
pub struct Bishop;

impl Bishop {
    super::generate!(Self::generate, PieceType::Bishop);

    pub const fn generate(cb: &Chessboard, sq: Square, check: bool) -> Bitboard {
        let us = cb.side();

        let attacks = get_bishop_attacks(sq, cb.occupied());

        if check {
            let king_sq = cb.get_king();
            // SAFETY: if we are in check there should be a checker piece
            let checker_sq = unsafe { cb.checkers.scan_forward().unwrap_unchecked() };
            let defending = get_connection_direct(king_sq, checker_sq).or(cb.checkers);

            if sq.bitboard().overlaps(cb.pinned) {
                let pinned_axis = get_connection_axis(king_sq, sq);
                attacks.and(defending).and(pinned_axis)
            } else {
                attacks.and(defending)
            }
        } else {
            let our_pieces = cb.colors(us);
            if sq.bitboard().overlaps(cb.pinned) {
                let king_sq = cb.get_king();
                let pinned_axis = get_connection_axis(king_sq, sq);
                attacks.and(our_pieces.neg()).and(pinned_axis)
            } else {
                attacks.and(our_pieces.neg())
            }
        }
    }
}
