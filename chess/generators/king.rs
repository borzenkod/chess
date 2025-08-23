use types::Bitboard;
use types::Directions;
use types::Square;

pub fn init_king_attacks() -> Box<[Bitboard; Square::LEN]> {
    let mut king_attacks = Box::new([Bitboard::EMPTY; Square::LEN]);

    for square in Square::ALL {
        let index = square.as_u8() as usize;
        let king_attack = &mut king_attacks[index];
        for dir in Directions::ALL {
            if let Some(new_square) = square.shift(dir) {
                *king_attack |= new_square.bitboard();
            }
        }
    }

    king_attacks
}
