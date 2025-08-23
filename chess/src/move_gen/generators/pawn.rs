use crate::{
    Bitboard, Chessboard, Piece, PieceType, Rank, Side, Square, get_bishop_attacks,
    get_connection_axis, get_connection_direct, get_pawn_attacks, get_rook_attacks,
};

use crate::MoveList;

#[derive(Debug)]
pub struct Pawn;

impl Pawn {
    super::generate!(Self::generate, PieceType::Pawn);

    const fn pawn_non_captures(
        cb: &Chessboard,
        sq: Square,
        sqb: Bitboard,
        pinned: bool,
    ) -> Bitboard {
        let mut result = Bitboard::EMPTY;

        let pinned_forward = if pinned {
            cb.get_king().file().as_u8() == sq.file().as_u8()
        } else {
            false
        };

        if !pinned || pinned_forward {
            let (forward, start) = match cb.side() {
                Side::White => (sqb.shift_up(), Rank::Second),
                Side::Black => (sqb.shift_down(), Rank::Seventh),
            };

            if !forward.overlaps(cb.occupied()) {
                result = result.or(forward);
                if sq.rank().as_u8() == start.as_u8() {
                    let double = match cb.side() {
                        Side::White => sqb.shift_up().shift_up(),
                        Side::Black => sqb.shift_down().shift_down(),
                    };

                    if !double.overlaps(cb.occupied()) {
                        result = result.or(double);
                    }
                }
            }
        }

        result
    }

    const fn pawn_en_passant(cb: &Chessboard, sq: Square, pinned: bool) -> Bitboard {
        let mut result = Bitboard::EMPTY;
        let them = cb.side().neg();

        let pinned_forward = if pinned {
            cb.get_king().file().as_u8() == sq.file().as_u8()
        } else {
            false
        };

        if !pinned_forward {
            let mut attacks = get_pawn_attacks(cb.side(), sq);
            if pinned {
                attacks = attacks.and(get_connection_axis(cb.get_king(), sq));
            }

            result = result.or(attacks.and(cb.colors(them)));

            let en_passant = match cb.en_passant() {
                Some(en_passant) => en_passant.bitboard(),
                None => Bitboard::EMPTY,
            };

            if attacks.overlaps(en_passant) {
                let attacked = match cb.side() {
                    Side::White => en_passant.shift_down(),
                    Side::Black => en_passant.shift_up(),
                };

                let new_occupied_bitboard =
                    cb.occupied().xor(sq.bitboard().or(en_passant).or(attacked));

                let bishop = get_bishop_attacks(cb.get_king(), new_occupied_bitboard);
                let enemy_bishops = cb
                    .get_pieces(Piece::from_side_and_type(them, PieceType::Bishop))
                    .or(cb.get_pieces(Piece::from_side_and_type(them, PieceType::Queen)));

                let rook = get_rook_attacks(cb.get_king(), new_occupied_bitboard);
                let enemy_rooks = cb
                    .get_pieces(Piece::from_side_and_type(them, PieceType::Rook))
                    .or(cb.get_pieces(Piece::from_side_and_type(them, PieceType::Queen)));

                if !enemy_rooks.overlaps(rook) && !enemy_bishops.overlaps(bishop) {
                    result = result.or(en_passant);
                }
            }
        }

        result
    }

    pub const fn generate(cb: &Chessboard, sq: Square, check: bool) -> Bitboard {
        let sqb = sq.bitboard();

        let mut result = Bitboard::EMPTY;

        let is_pinned = sqb.overlaps(cb.pinned);

        result = result.or(Self::pawn_non_captures(cb, sq, sqb, is_pinned));
        result = result.or(Self::pawn_en_passant(cb, sq, is_pinned));

        if check {
            let king_sq = cb.get_king();
            // SAFETY: if we are in check there should be a checker piece
            let checker_sq = unsafe { cb.checkers.scan_forward().unwrap_unchecked() };
            let defending = get_connection_direct(king_sq, checker_sq).or(cb.checkers);
            result = result.and(defending);
        }

        result
    }
}
