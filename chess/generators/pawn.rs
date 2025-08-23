use types::{Bitboard, Directions, Side, Square};

pub fn init_pawn_attacks() -> Box<[[Bitboard; Side::LEN]; Square::LEN]> {
    let mut pawn_attacks = Box::new([[Bitboard::EMPTY; Side::LEN]; Square::LEN]);

    for square in Square::ALL {
        let index = square.as_u8() as usize;
        if let Some(new_square) = square.shift(Directions::UpLeft) {
            pawn_attacks[index][Side::White.as_u8() as usize] |= new_square.bitboard();
        }
        if let Some(new_square) = square.shift(Directions::UpRight) {
            pawn_attacks[index][Side::White.as_u8() as usize] |= new_square.bitboard();
        }

        if let Some(new_square) = square.shift(Directions::DownLeft) {
            pawn_attacks[index][Side::Black.as_u8() as usize] |= new_square.bitboard();
        }
        if let Some(new_square) = square.shift(Directions::DownRight) {
            pawn_attacks[index][Side::Black.as_u8() as usize] |= new_square.bitboard();
        }
    }

    pawn_attacks
}
