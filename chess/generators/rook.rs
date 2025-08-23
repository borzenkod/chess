use fastrand::Rng;
use types::{Bitboard, Directions, File, Magic, Rank, Square};

use crate::generators::{ROOK_MAGICKS, blockers::Blockers, ray_attacks};

#[rustfmt::skip]
const ROOK_BITS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

pub fn init_rook_magics() -> ([Magic; Square::LEN], usize) {
    let mut magics = [Magic::EMPTY; Square::LEN];
    let mut offset: usize = 0;
    let mut rng = Rng::with_seed(0xBEEFDEAD);

    for (index, square) in Square::ALL.into_iter().enumerate() {
        println!("info=rook magic {}", square.as_u8());
        let mut attack_table = [Bitboard::EMPTY; 4096];

        let mask = get_rook_mask(square);
        let shift = 64 - ROOK_BITS[square.as_u8() as usize];
        let blockers: Vec<_> = Blockers::new(mask)
            .map(|b| (b, get_rook_attacks_slow(square, b)))
            .collect();

        let mut candidate;
        let mut max_key;
        let mut tries = 0;
        let mut max = 0;
        loop {
            max_key = 0;
            candidate = rng.u64(0..=u64::MAX);
            candidate &= rng.u64(0..=u64::MAX);
            candidate &= rng.u64(0..=u64::MAX);
            attack_table.fill(Bitboard::EMPTY);
            let mut max_now = 0;

            let mut is_valid = true;
            for (blocker, attacks) in &blockers {
                let key = Magic::calculate_key(*blocker, mask, candidate, shift);
                max_key = max_key.max(key);

                if attack_table[key].is_empty() {
                    attack_table[key] = *attacks;
                } else if attack_table[key] != *attacks {
                    is_valid = false;
                    break;
                }
                max_now += 1;
            }

            if is_valid {
                break;
            } else {
                max = max.max(max_now);
            }
            tries += 1;
            if tries % 100000 == 0 {
                println!(
                    "info=rook magic {} tries {} {{max {}, key {}}}",
                    square.as_u8(),
                    tries,
                    max,
                    max_key
                );
            }
        }

        magics[index] = Magic::new(mask, candidate, shift, offset);
        offset += max_key + 1;
    }

    (magics, offset)
}

pub fn init_rook_attacks() -> Vec<Bitboard> {
    let mut rook_attacks = vec![Bitboard::EMPTY; ROOK_MAGICKS.get().unwrap().1];

    for square in Square::ALL {
        let blockers = Blockers::new(get_rook_mask(square));
        for blocker in blockers {
            let attacks = get_rook_attacks_slow(square, blocker);
            let key = (ROOK_MAGICKS.get().unwrap().0[square.as_u8() as usize]).key(blocker);
            rook_attacks[key] = attacks;
        }
    }

    rook_attacks
}

fn get_rook_mask(square: Square) -> Bitboard {
    let mut mask = get_rook_attacks_slow(square, Bitboard::EMPTY);

    if square.rank() != Rank::First {
        mask &= !Rank::First.bitboard();
    }

    if square.rank() != Rank::Eighth {
        mask &= !Rank::Eighth.bitboard();
    }

    if square.file() != File::A {
        mask &= !File::A.bitboard();
    }

    if square.file() != File::H {
        mask &= !File::H.bitboard();
    }

    mask
}

pub fn get_rook_attacks_slow(square: Square, occupied: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    attacks |= ray_attacks(square, Directions::Up, occupied);
    attacks |= ray_attacks(square, Directions::Down, occupied);
    attacks |= ray_attacks(square, Directions::Left, occupied);
    attacks |= ray_attacks(square, Directions::Right, occupied);

    attacks
}
