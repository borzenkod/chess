use types::{Bitboard, Directions, Square};

#[test]
fn dirs() {
    for dir in Directions::ALL {
        assert_eq!(dir, Directions::from_u8(dir.as_u8()))
    }
}

#[test]
fn and() {
    let a = Bitboard::from(Square::A1);
    let b = Bitboard::from(Square::B1);

    assert_eq!(a | b, 3.into());
}

#[test]
fn not() {
    let a = Bitboard::EMPTY;
    assert_eq!(!a, Bitboard::FULL)
}

#[test]
fn xor() {
    let a = Bitboard::from(1);
    let b = Bitboard::from(1);
    assert_eq!(a ^ b, Bitboard::EMPTY);
}

#[test]
fn first_square() {
    assert!(Bitboard::EMPTY.first_square().is_none());
    let a = Bitboard::from(Square::A2).first_square().unwrap();
    assert_eq!(Square::A2, a);
}

#[test]
fn stan_forward() {
    assert!(Bitboard::EMPTY.scan_forward().is_none());
    assert!(Bitboard::EMPTY.scan_backward().is_none());
    let a = Bitboard::from_squares(&[Square::C2, Square::C4]).scan_forward();
    assert_eq!(a, Some(Square::C2));
    let a = Bitboard::from_squares(&[Square::C2, Square::C4]).scan_backward();
    assert_eq!(a, Some(Square::C4));
}

#[test]
fn count_sq() {
    let board = Bitboard::EMPTY.popcnt();
    assert_eq!(board, 0);
    let board = Bitboard::FULL.popcnt();
    assert_eq!(board, 64);
}

#[test]
fn shifts() {
    let square = Square::C4;

    let bitboard = square.bitboard().shift_down();
    assert_eq!(bitboard.first_square().unwrap(), Square::C3);
    let bitboard = square.bitboard().shift(Directions::Down);
    assert_eq!(bitboard.first_square().unwrap(), Square::C3);

    let bitboard = square.bitboard().shift_up();
    assert_eq!(bitboard.first_square().unwrap(), Square::C5);
    let bitboard = square.bitboard().shift(Directions::Up);
    assert_eq!(bitboard.first_square().unwrap(), Square::C5);

    let bitboard = square.bitboard().shift_left();
    assert_eq!(bitboard.first_square().unwrap(), Square::B4);
    let bitboard = square.bitboard().shift(Directions::Left);
    assert_eq!(bitboard.first_square().unwrap(), Square::B4);

    let bitboard = square.bitboard().shift_right();
    assert_eq!(bitboard.first_square().unwrap(), Square::D4);
    let bitboard = square.bitboard().shift(Directions::Right);
    assert_eq!(bitboard.first_square().unwrap(), Square::D4);

    let bitboard = square.bitboard().shift(Directions::UpLeft);
    assert_eq!(bitboard.first_square().unwrap(), Square::B5);
    let bitboard = square.bitboard().shift(Directions::UpRight);
    assert_eq!(bitboard.first_square().unwrap(), Square::D5);

    let bitboard = square.bitboard().shift(Directions::DownLeft);
    assert_eq!(bitboard.first_square().unwrap(), Square::B3);
    let bitboard = square.bitboard().shift(Directions::DownRight);
    assert_eq!(bitboard.first_square().unwrap(), Square::D3);
}

#[test]
fn ops() {
    let bitboard = Bitboard::from(42 | 32);

    let mut bitboard2 = Bitboard::EMPTY;
    bitboard2 |= Bitboard::from(42);
    bitboard2 |= Bitboard::from(32);

    assert_eq!(bitboard, bitboard2);

    let bitboard = Bitboard::from(42 & 32);
    let bitboard3 = Bitboard::from(42) & Bitboard::from(32);

    let mut bitboard2 = Bitboard::FULL;
    bitboard2 &= Bitboard::from(42);
    bitboard2 &= Bitboard::from(32);

    assert_eq!(bitboard, bitboard2);
    assert_eq!(bitboard, bitboard3);

    let bitboard = Bitboard::from(42 ^ 32);

    let mut bitboard2 = Bitboard::EMPTY;
    bitboard2 ^= Bitboard::from(42);
    bitboard2 ^= Bitboard::from(32);

    assert_eq!(bitboard, bitboard2);
}

#[test]
fn iterators() {
    let mut i = 0;
    for _c in Bitboard::FULL {
        i += 1;
        assert!(i <= 64);
    }
    assert_eq!(i, 64);
}

#[test]
fn edge() {
    for dir in Directions::ALL {
        println!("{}", dir.edge());
    }
    assert!(Square::A8.bitboard().overlaps(Directions::Up.edge()));
    assert!(!Square::A8.bitboard().overlaps(Directions::Down.edge()));
}
