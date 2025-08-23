use std::io::Write;
use std::sync::OnceLock;

mod axis;
mod bishop;
mod blockers;
mod king;
mod knight;
mod pawn;
mod rook;
mod writer;

use types::{Bitboard, Directions, Magic, Side, Square};

pub static RAYS: OnceLock<Box<[[Bitboard; Directions::LEN]; Square::LEN]>> = OnceLock::new();
pub static AXIS_CONNECTIONS: OnceLock<Box<[[Bitboard; Square::LEN]; Square::LEN]>> =
    OnceLock::new();
pub static DIRECT_CONNECTIONS: OnceLock<Box<[[Bitboard; Square::LEN]; Square::LEN]>> =
    OnceLock::new();
pub static PAWN_ATTACKS: OnceLock<Box<[[Bitboard; Side::LEN]; Square::LEN]>> = OnceLock::new();
pub static KNIGHT_ATTACKS: OnceLock<Box<[Bitboard; Square::LEN]>> = OnceLock::new();
pub static KING_ATTACKS: OnceLock<Box<[Bitboard; Square::LEN]>> = OnceLock::new();
pub static BISHOP_MAGICKS: OnceLock<([Magic; Square::LEN], usize)> = OnceLock::new();
pub static BISHOP_ATTACKS: OnceLock<Vec<Bitboard>> = OnceLock::new();
pub static ROOK_MAGICKS: OnceLock<([Magic; Square::LEN], usize)> = OnceLock::new();
pub static ROOK_ATTACKS: OnceLock<Vec<Bitboard>> = OnceLock::new();

pub fn init() {
    RAYS.get_or_init(|| {
        println!("info=rays");
        init_rays()
    });
    AXIS_CONNECTIONS.get_or_init(|| {
        println!("info=axis");
        axis::init_axis_connections()
    });
    DIRECT_CONNECTIONS.get_or_init(|| {
        println!("info=direct");
        axis::init_direct_connections()
    });
    PAWN_ATTACKS.get_or_init(|| {
        println!("info=pawns");
        pawn::init_pawn_attacks()
    });
    KNIGHT_ATTACKS.get_or_init(|| {
        println!("info=knight");
        knight::init_knight_attacks()
    });
    KING_ATTACKS.get_or_init(|| {
        println!("info=king");
        king::init_king_attacks()
    });
    BISHOP_MAGICKS.get_or_init(|| {
        println!("info=bishop magic");
        bishop::init_bishop_magics()
    });
    BISHOP_ATTACKS.get_or_init(|| {
        println!("info=bishop");
        bishop::init_bishop_attacks()
    });
    ROOK_MAGICKS.get_or_init(|| {
        println!("info=rook magic");
        rook::init_rook_magics()
    });
    ROOK_ATTACKS.get_or_init(|| {
        println!("info=rook");
        rook::init_rook_attacks()
    });
}

pub fn write(f: &mut std::fs::File) {
    writeln!(f, "#[allow(dead_code)]").unwrap();
    writer::compiled!(f, RAYS, RAYS.get().unwrap().as_ref());
    writer::compiled!(
        f,
        AXIS_CONNECTIONS,
        AXIS_CONNECTIONS.get().unwrap().as_ref()
    );
    writer::compiled!(
        f,
        DIRECT_CONNECTIONS,
        DIRECT_CONNECTIONS.get().unwrap().as_ref()
    );
    writer::compiled!(f, PAWN_ATTACKS, PAWN_ATTACKS.get().unwrap().as_ref());
    writer::compiled!(f, KNIGHT_ATTACKS, KNIGHT_ATTACKS.get().unwrap().as_ref());
    writer::compiled!(f, KING_ATTACKS, KING_ATTACKS.get().unwrap().as_ref());
    writer::compiled!(f, BISHOP_MAGICKS, &(BISHOP_MAGICKS.get().unwrap().0));
    writer::compiled!(f, BISHOP_ATTACKS, BISHOP_ATTACKS.get().unwrap());
    writer::compiled!(f, ROOK_MAGICKS, &(ROOK_MAGICKS.get().unwrap().0));
    writer::compiled!(f, ROOK_ATTACKS, ROOK_ATTACKS.get().unwrap());
}

fn init_rays() -> Box<[[Bitboard; Directions::LEN]; Square::LEN]> {
    let mut rays = Box::new([[Bitboard::EMPTY; Directions::LEN]; Square::LEN]);

    for square in Square::ALL {
        for dir in Directions::ALL {
            let mut bitboard = square.bitboard();

            while !dir.edge().overlaps(bitboard) {
                bitboard = bitboard.shift(dir);
                rays[square.as_u8() as usize][dir.as_u8() as usize] |= bitboard;
            }
        }
    }

    rays
}

fn ray_attacks(square: Square, dir: Directions, occupied: Bitboard) -> Bitboard {
    let mut attacks = RAYS.get().unwrap()[square.as_u8() as usize][dir.as_u8() as usize];
    let blockers = attacks & occupied;
    if !blockers.is_empty() {
        let square = match dir {
            Directions::Up | Directions::UpLeft | Directions::UpRight | Directions::Right => {
                blockers.scan_forward().unwrap()
            }

            Directions::Down | Directions::Left | Directions::DownLeft | Directions::DownRight => {
                blockers.scan_backward().unwrap()
            }
        };
        attacks ^= RAYS.get().unwrap()[square.as_u8() as usize][dir.as_u8() as usize];
    }
    attacks
}
