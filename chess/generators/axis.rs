use types::{Bitboard, Square};

use crate::generators::{bishop::get_bishop_attacks_slow, rook::get_rook_attacks_slow};

pub fn init_axis_connections() -> Box<[[Bitboard; Square::LEN]; Square::LEN]> {
    let mut axis_connections = Box::new([[Bitboard::EMPTY; Square::LEN]; Square::LEN]);

    for start_square in Square::ALL {
        let start_bb = start_square.bitboard();
        for end_square in Square::ALL {
            let end_bb = end_square.bitboard();

            let bishop_attacks = get_bishop_attacks_slow(start_square, Bitboard::EMPTY) | start_bb;
            let rook_attacks = get_rook_attacks_slow(start_square, Bitboard::EMPTY) | start_bb;

            let mut connection = Bitboard::EMPTY;
            if bishop_attacks.overlaps(end_bb) {
                connection |= bishop_attacks
                    & (get_bishop_attacks_slow(end_square, Bitboard::EMPTY) | end_bb);
            }
            if rook_attacks.overlaps(end_bb) {
                connection |=
                    rook_attacks & (get_rook_attacks_slow(end_square, Bitboard::EMPTY) | end_bb);
            }

            axis_connections[start_square.as_u8() as usize][end_square.as_u8() as usize] =
                connection;
        }
    }

    axis_connections
}

pub fn init_direct_connections() -> Box<[[Bitboard; Square::LEN]; Square::LEN]> {
    let mut direct_connections = Box::new([[Bitboard::EMPTY; Square::LEN]; Square::LEN]);

    for start_square in Square::ALL {
        let start_bb = start_square.bitboard();
        for end_square in Square::ALL {
            let end_bb = end_square.bitboard();

            let occupied = start_bb | end_bb;

            let bishop_attacks = get_bishop_attacks_slow(start_square, occupied);
            let rook_attacks = get_rook_attacks_slow(start_square, occupied);
            let connection = if bishop_attacks.overlaps(end_bb) {
                let attacks = get_bishop_attacks_slow(end_square, occupied);
                attacks & bishop_attacks
            } else if rook_attacks.overlaps(end_bb) {
                let attacks = get_rook_attacks_slow(end_square, occupied);
                attacks & rook_attacks
            } else {
                Bitboard::EMPTY
            };

            direct_connections[start_square.as_u8() as usize][end_square.as_u8() as usize] =
                connection;
        }
    }

    direct_connections
}
