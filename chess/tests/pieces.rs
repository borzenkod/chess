use chess::Piece;

#[test]
fn pieces_bit() {
    assert_eq!(Piece::WHITE_PAWN.as_u8(), 0b0000);
    assert_eq!(Piece::WHITE_ROOK.as_u8(), 0b0001);
    assert_eq!(Piece::WHITE_KNIGHT.as_u8(), 0b0010);
    assert_eq!(Piece::WHITE_BISHOP.as_u8(), 0b0011);
    assert_eq!(Piece::WHITE_QUEEN.as_u8(), 0b0100);
    assert_eq!(Piece::WHITE_KING.as_u8(), 0b0101);

    assert_eq!(Piece::BLACK_PAWN.as_u8(), 0b1000);
    assert_eq!(Piece::BLACK_ROOK.as_u8(), 0b1001);
    assert_eq!(Piece::BLACK_KNIGHT.as_u8(), 0b1010);
    assert_eq!(Piece::BLACK_BISHOP.as_u8(), 0b1011);
    assert_eq!(Piece::BLACK_QUEEN.as_u8(), 0b1100);
    assert_eq!(Piece::BLACK_KING.as_u8(), 0b1101);

    assert_eq!(Piece::WHITE_PAWN.to_char(), 'P');
    assert_eq!(Piece::WHITE_ROOK.to_char(), 'R');
    assert_eq!(Piece::WHITE_KNIGHT.to_char(), 'N');
    assert_eq!(Piece::WHITE_BISHOP.to_char(), 'B');
    assert_eq!(Piece::WHITE_QUEEN.to_char(), 'Q');
    assert_eq!(Piece::WHITE_KING.to_char(), 'K');

    assert_eq!(Piece::BLACK_PAWN.to_char(), 'p');
    assert_eq!(Piece::BLACK_ROOK.to_char(), 'r');
    assert_eq!(Piece::BLACK_KNIGHT.to_char(), 'n');
    assert_eq!(Piece::BLACK_BISHOP.to_char(), 'b');
    assert_eq!(Piece::BLACK_QUEEN.to_char(), 'q');
    assert_eq!(Piece::BLACK_KING.to_char(), 'k');
}

#[test]
fn pieces_char() {
    assert_eq!(Piece::WHITE_PAWN, Piece::from_char('P').unwrap());
    assert_eq!(Piece::WHITE_ROOK, Piece::from_char('R').unwrap());
    assert_eq!(Piece::WHITE_KNIGHT, Piece::from_char('N').unwrap());
    assert_eq!(Piece::WHITE_BISHOP, Piece::from_char('B').unwrap());
    assert_eq!(Piece::WHITE_QUEEN, Piece::from_char('Q').unwrap());
    assert_eq!(Piece::WHITE_KING, Piece::from_char('K').unwrap());

    assert_eq!(Piece::BLACK_PAWN, Piece::from_char('p').unwrap());
    assert_eq!(Piece::BLACK_ROOK, Piece::from_char('r').unwrap());
    assert_eq!(Piece::BLACK_KNIGHT, Piece::from_char('n').unwrap());
    assert_eq!(Piece::BLACK_BISHOP, Piece::from_char('b').unwrap());
    assert_eq!(Piece::BLACK_QUEEN, Piece::from_char('q').unwrap());
    assert_eq!(Piece::BLACK_KING, Piece::from_char('k').unwrap());
}
