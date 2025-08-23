use types::Bitboard;

pub struct Blockers {
    mask: Bitboard,
    n_combinations: u32,
    current_combinations: u32,
}

impl Blockers {
    pub fn new(mask: Bitboard) -> Self {
        Self {
            mask,
            n_combinations: 2u32.pow(mask.popcnt()),
            current_combinations: 0,
        }
    }
}

impl Iterator for Blockers {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_combinations == self.n_combinations {
            return None;
        }

        let mut mask_combinations = Bitboard::EMPTY;
        for (index, square) in self.mask.into_iter().enumerate() {
            if self.current_combinations & (1 << index) != 0 {
                mask_combinations |= square.bitboard();
            }
        }

        self.current_combinations += 1;
        Some(mask_combinations)
    }
}
