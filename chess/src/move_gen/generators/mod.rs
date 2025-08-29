use crate::{
    get_bishop_attacks, get_king_attacks, get_knight_attacks, get_pawn_attacks, get_rook_attacks, Bitboard, Chessboard, MoveGen, Piece, Square
};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub use bishop::Bishop;
pub use king::King;
pub use knight::Knight;
pub use pawn::Pawn;
pub use queen::Queen;
pub use rook::Rook;
use types::{GameResult, MoveList, PieceType};

macro_rules! generate {
    ($fn:expr, $piece:expr) => {
        pub const fn generate_moves(cb: &Chessboard, moves: &mut MoveList, check: bool) {
            let us = cb.side();
            let piece = crate::Piece::from_side_and_type(us, $piece);

            let mut knights = cb.get_pieces(piece);
            loop {
                let sq = knights.next_const();
                match sq {
                    Some(sq) => {
                        let attacks = $fn(cb, sq, check);
                        moves.push(crate::Moves {
                            piece,
                            from: sq,
                            bitboard: attacks,
                        });
                    }
                    None => return,
                };
            }
        }
    };
}
pub(super) use generate;

pub const fn perft(cb: &mut Chessboard, depth: u32) -> usize {
    if depth == 0 {
        return 0;
    }

    match cb.result {
        GameResult::None => (),
        _ => return 0,
    }

    if depth == 1 {
        return cb.moves_cache.count();
    }

    let mut count = 0;
    let mut move_gen = MoveGen::new(cb.moves_cache);
    while let Some(m) = move_gen.next_const() {
        count += perft(&mut cb.move_new(m), depth - 1);
    }
    count
}

pub const fn generate_moves(cb: &mut Chessboard) -> MoveList {
    let mut moves = MoveList::new(cb.side(), cb.en_passant());

    if cb.checkers.is_empty() {
        Pawn::generate_moves(cb, &mut moves, false);
        Rook::generate_moves(cb, &mut moves, false);
        Knight::generate_moves(cb, &mut moves, false);
        Bishop::generate_moves(cb, &mut moves, false);
        Queen::generate_moves(cb, &mut moves, false);
        King::generate_moves(cb, &mut moves, false);
    } else if cb.checkers.popcnt() == 1 {
        Pawn::generate_moves(cb, &mut moves, true);
        Rook::generate_moves(cb, &mut moves, true);
        Knight::generate_moves(cb, &mut moves, true);
        Bishop::generate_moves(cb, &mut moves, true);
        Queen::generate_moves(cb, &mut moves, true);
        King::generate_moves(cb, &mut moves, true);
    } else {
        King::generate_moves(cb, &mut moves, true);
    }

    moves
}

const fn is_check(
    cb: &Chessboard,
    new_occupanicy: Bitboard,
    them: types::Side,
    target: Square,
) -> bool {
    let pawn_attack_squares = get_pawn_attacks(them.neg(), target);
    let enemy_pawns = cb.get_pieces(Piece::from_side_and_type(them, PieceType::Pawn));
    if enemy_pawns.overlaps(pawn_attack_squares) {
        return true;
    }

    let enemy_queens = cb.get_pieces(Piece::from_side_and_type(them, PieceType::Queen));

    let rook_attack_squares = get_rook_attacks(target, new_occupanicy);
    let enemy_rooks = cb.get_pieces(Piece::from_side_and_type(them, PieceType::Rook));
    let rooks_and_queens = enemy_rooks.or(enemy_queens);
    if rooks_and_queens.overlaps(rook_attack_squares) {
        return true;
    }

    let knight_attack_squares = get_knight_attacks(target);
    let enemy_knights = cb.get_pieces(Piece::from_side_and_type(them, PieceType::Knight));
    if enemy_knights.overlaps(knight_attack_squares) {
        return true;
    }

    let bishop_attack_squares = get_bishop_attacks(target, new_occupanicy);
    let enemy_bishops = cb.get_pieces(Piece::from_side_and_type(them, PieceType::Bishop));
    let bishops_and_queens = enemy_bishops.or(enemy_queens);
    if bishops_and_queens.overlaps(bishop_attack_squares) {
        return true;
    }

    let king = get_king_attacks(target);
    let enemy_king = cb.get_pieces(Piece::from_side_and_type(them, PieceType::King));
    if enemy_king.overlaps(king) {
        return true;
    }

    false
}
