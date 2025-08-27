use types::{Bitboard, MoveList, Moves, Piece, Square};

#[test]
fn empty() {
    let ml = MoveList::default();
    assert_eq!(ml.count(), 0);
}

#[test]
fn push_empty() {
    let mut ml = MoveList::default();
    let moves = Moves::EMPTY;
    ml.push(moves);
    assert_eq!(ml.count(), 0);
}

#[test]
fn push_one() {
    let mut ml = MoveList::default();
    let moves = Moves {
        piece: Piece::WHITE_PAWN,
        from: Square::C2,
        bitboard: Bitboard::from_squares(&[Square::C3, Square::C4]),
    };
    ml.push(moves);
    assert_eq!(ml.count(), 2);
}

#[test]
fn push_one_black() {
    let mut ml = MoveList::default();
    let moves = Moves {
        piece: Piece::BLACK_PAWN,
        from: Square::C7,
        bitboard: Bitboard::from_squares(&[Square::C6, Square::C5]),
    };
    ml.push(moves);
    assert_eq!(ml.count(), 2);
}

#[test]
fn last() {
    let mut ml = MoveList::default();
    let moves = Moves {
        piece: Piece::WHITE_PAWN,
        from: Square::C2,
        bitboard: Bitboard::from_squares(&[Square::C3, Square::C4]),
    };
    ml.push(moves);
    let moves = Moves {
        piece: Piece::WHITE_KNIGHT,
        from: Square::B1,
        bitboard: Bitboard::from_squares(&[Square::A3, Square::C3]),
    };
    ml.push(moves);

    assert_eq!(ml.count(), 4);

    assert_eq!(ml.last().unwrap().from, Square::B1);
    let last = ml.last_mut().unwrap();
    last.from = Square::E2;
    assert_eq!(ml.last().unwrap().from, Square::E2);

    let pop = ml.pop().unwrap();
    assert_eq!(pop.from, Square::E2);

    let pop = ml.pop().unwrap();
    assert_eq!(pop.from, Square::C2);

    assert!(ml.is_empty());
    assert_eq!(ml.pop(), None);
    assert_eq!(ml.last(), None);
    assert_eq!(ml.last_mut(), None);
}
