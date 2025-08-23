use types::{Piece, PieceType, Side};

#[test]
fn invalid() {
    assert!(Piece::from_char('a').is_none());
    Piece::try_from(0b1111u8).unwrap_err();
}

#[test]
#[should_panic]
fn from_u8_panic() {
    let _ = Piece::from_u8(0b1111);
}

#[test]
fn from_u8() {
    let piece = Piece::try_from(0b1010u8).unwrap();
    assert_eq!(piece.as_u8(), 0b1010);
}

#[test]
fn from() {
    let piece: Piece = (Side::White, PieceType::Pawn).into();
    let piece2: Piece = (PieceType::Pawn, Side::White).into();
    assert_eq!(piece, piece2);
}

#[test]
fn r#type() {
    let pawn = PieceType::from_u8(0);
    assert_eq!(pawn, PieceType::Pawn);
}
