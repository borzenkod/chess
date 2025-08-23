use types::CastlingInfo;

#[test]
fn default() {
    let castling = CastlingInfo::DEFAULT;
    assert_eq!(castling.as_u8(), 0b1111);
    assert_eq!(castling.to_string(), "KQkq");
}

#[test]
fn from_u8() {
    let castling = CastlingInfo::from(0b1111u8);
    assert_eq!(castling.as_u8(), 0b1111);
    assert_eq!(castling.to_string(), "KQkq");
}

#[test]
fn from_str() {
    let castling = CastlingInfo::try_from("KQkq").unwrap();
    assert_eq!(castling.as_u8(), 0b1111);
    let castling = CastlingInfo::try_from("-").unwrap();
    assert_eq!(castling.as_u8(), 0b0000);
    let _ = CastlingInfo::try_from("s").unwrap_err();
}

#[test]
fn from_char() {
    let mut castling = CastlingInfo::EMPTY;
    assert!(castling.update(b'k'));
    assert!(castling.update(b'K'));
    assert!(castling.update(b'q'));
    assert!(castling.update(b'Q'));
    assert!(!castling.update(b'I'));
}
