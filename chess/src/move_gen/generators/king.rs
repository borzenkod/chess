use crate::MoveList;
use crate::{Castling, PieceType};

use crate::{Bitboard, Chessboard, Square, get_king_attacks};

#[derive(Debug)]
pub struct King;

impl King {
    super::generate!(Self::generate, PieceType::King);

    const fn generate_castling(cb: &Chessboard, check: bool) -> Bitboard {
        if check {
            return Bitboard::EMPTY;
        }

        let mut castling = Bitboard::EMPTY;

        castling = castling.or(Self::generate_king_side_castling(cb));
        castling = castling.or(Self::generate_queen_side_castling(cb));

        castling
    }

    const fn generate_king_side_castling(cb: &Chessboard) -> Bitboard {
        if !cb.castling().is_set(Castling::KingSide, cb.side()) {
            return Bitboard::EMPTY;
        }

        const ENSURE_EMPTY: [Bitboard; 2] = [
            Bitboard::from_squares(&[Square::F1, Square::G1]), // white
            Bitboard::from_squares(&[Square::F8, Square::G8]), // black
        ];

        let mut empty_sq = ENSURE_EMPTY[cb.side() as usize];

        if cb.occupied().overlaps(empty_sq) {
            return Bitboard::EMPTY;
        }

        let them = cb.side().neg();

        while let Some(sq) = empty_sq.next_const() {
            if super::is_check(cb, cb.occupied(), them, sq) {
                return Bitboard::EMPTY;
            }
        }

        const TARGET: [Square; 2] = [Square::G1, Square::G8];

        TARGET[cb.side() as usize].bitboard()
    }

    const fn generate_queen_side_castling(cb: &Chessboard) -> Bitboard {
        if !cb.castling().is_set(Castling::QueenSide, cb.side()) {
            return Bitboard::EMPTY;
        }

        const UNCHECKED_SQUARES: [Bitboard; 2] = [
            Bitboard::from_squares(&[Square::C1, Square::D1]), // white
            Bitboard::from_squares(&[Square::C8, Square::D8]), // black
        ];

        const ENSURE_EMPTY: [Bitboard; 2] = [Square::B1.bitboard(), Square::B8.bitboard()];

        let mut unchecked_sq = UNCHECKED_SQUARES[cb.side() as usize];
        let empty_sq = ENSURE_EMPTY[cb.side() as usize].or(unchecked_sq);

        if cb.occupied().overlaps(empty_sq) {
            return Bitboard::EMPTY;
        }

        let them = cb.side().neg();

        while let Some(sq) = unchecked_sq.next_const() {
            if super::is_check(cb, cb.occupied(), them, sq) {
                return Bitboard::EMPTY;
            }
        }

        const TARGET: [Square; 2] = [Square::C1, Square::C8];

        TARGET[cb.side() as usize].bitboard()
    }

    pub const fn generate(cb: &Chessboard, sq: Square, check: bool) -> Bitboard {
        let us = cb.side();
        let them = us.neg();

        let mut attacks = get_king_attacks(sq);

        attacks = attacks.and(cb.colors(us).neg());

        let mut attacks_clone = attacks;

        while let Some(target) = attacks_clone.next_const() {
            let new_occupanicy = cb.occupied().xor(sq.bitboard());
            if super::is_check(cb, new_occupanicy, them, target) {
                attacks = attacks.xor(target.bitboard());
            }
        }

        attacks = attacks.or(Self::generate_castling(cb, check));

        attacks
    }
}
