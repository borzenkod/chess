use crate::MoveList;
use crate::PieceType;

use crate::{Bishop, Bitboard, Chessboard, Rook, Square};

#[derive(Debug)]
pub struct Queen;

impl Queen {
    super::generate!(Self::generate, PieceType::Queen);

    pub const fn generate(cb: &Chessboard, sq: Square, check: bool) -> Bitboard {
        Rook::generate(cb, sq, check).or(Bishop::generate(cb, sq, check))
    }
}
