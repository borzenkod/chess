use types::{Bitboard, File, Rank, Square};

pub fn init_knight_attacks() -> Box<[Bitboard; Square::LEN]> {
    let mut knight_attacks = Box::new([Bitboard::EMPTY; Square::LEN]);

    for square in Square::ALL {
        let index = square.as_u8() as usize;

        if square.rank() != Rank::Eighth && square.file() > File::B {
            knight_attacks[index] |= square.bitboard().shift_up().shift_left().shift_left();
        }

        if square.rank() != Rank::Eighth && square.file() < File::G {
            knight_attacks[index] |= square.bitboard().shift_up().shift_right().shift_right();
        }

        if square.rank() != Rank::First && square.file() > File::B {
            knight_attacks[index] |= square.bitboard().shift_down().shift_left().shift_left();
        }

        if square.rank() != Rank::First && square.file() < File::G {
            knight_attacks[index] |= square.bitboard().shift_down().shift_right().shift_right();
        }

        if square.file() != File::A && square.rank() < Rank::Seventh {
            knight_attacks[index] |= square.bitboard().shift_left().shift_up().shift_up();
        }

        if square.file() != File::A && square.rank() > Rank::Second {
            knight_attacks[index] |= square.bitboard().shift_left().shift_down().shift_down();
        }

        if square.file() != File::H && square.rank() < Rank::Seventh {
            knight_attacks[index] |= square.bitboard().shift_right().shift_up().shift_up();
        }

        if square.file() != File::H && square.rank() > Rank::Second {
            knight_attacks[index] |= square.bitboard().shift_right().shift_down().shift_down();
        }
    }

    knight_attacks
}
