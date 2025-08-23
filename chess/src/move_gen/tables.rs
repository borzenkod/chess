use types::{Bitboard, Magic, Side, Square};

include!("../../generated.rs");

pub const fn get_pawn_attacks(side: Side, square: Square) -> Bitboard {
    PAWN_ATTACKS[square.as_u8() as usize][side.as_u8() as usize]
}

pub const fn get_rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    let key = ROOK_MAGICKS[square.as_u8() as usize].key(occupied);
    ROOK_ATTACKS[key]
}

pub const fn get_rook_pinner(
    square: Square,
    occupied: Bitboard,
    mut friendly: Bitboard,
) -> Bitboard {
    let seen = get_rook_attacks(square, occupied);

    friendly = friendly.and(seen);

    seen.xor(get_rook_attacks(square, occupied.xor(friendly)))
}

pub const fn get_bishop_pinner(
    square: Square,
    occupied: Bitboard,
    mut friendly: Bitboard,
) -> Bitboard {
    let seen = get_bishop_attacks(square, occupied);

    friendly = friendly.and(seen);

    seen.xor(get_bishop_attacks(square, occupied.xor(friendly)))
}

pub const fn get_knight_attacks(square: Square) -> Bitboard {
    KNIGHT_ATTACKS[square.as_u8() as usize]
}

pub const fn get_bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    let key = BISHOP_MAGICKS[square.as_u8() as usize].key(occupied);
    BISHOP_ATTACKS[key]
}

pub const fn get_king_attacks(square: Square) -> Bitboard {
    KING_ATTACKS[square.as_u8() as usize]
}

pub const fn get_connection_axis(start: Square, end: Square) -> Bitboard {
    AXIS_CONNECTIONS[start.as_u8() as usize][end.as_u8() as usize]
}

pub const fn get_connection_direct(start: Square, end: Square) -> Bitboard {
    DIRECT_CONNECTIONS[start.as_u8() as usize][end.as_u8() as usize]
}
