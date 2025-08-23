use types::{Bitboard, Magic};

#[test]
fn magic() {
    let magic: Magic = Magic::new(
        Bitboard::from_u64(18049651735527936),
        2535528071299584,
        58,
        0,
    );
    let key: usize = magic.key(Bitboard::FULL);

    assert_eq!(key, 23)
}
