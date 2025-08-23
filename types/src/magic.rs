use crate::Bitboard;

#[derive(Debug)]
pub struct Magic {
    pub mask: Bitboard,
    pub magic: u64,
    pub shift: u8,
    pub offset: usize,
}

impl Magic {
    pub const EMPTY: Self = Self {
        mask: Bitboard::EMPTY,
        magic: 0,
        shift: 0,
        offset: 0,
    };

    pub const fn new(mask: Bitboard, magic: u64, shift: u8, offset: usize) -> Self {
        Self {
            mask,
            magic,
            shift,
            offset,
        }
    }

    pub const fn key(&self, occupied: Bitboard) -> usize {
        Self::calculate_key(occupied, self.mask, self.magic, self.shift) + self.offset
    }

    pub const fn calculate_key(occupied: Bitboard, mask: Bitboard, magic: u64, shift: u8) -> usize {
        let masked = occupied.and(mask);
        let hash = masked.as_u64().wrapping_mul(magic);
        let key = hash >> shift;
        key as usize
    }
}
