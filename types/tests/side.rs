use std::str::FromStr as _;

use types::Side;

#[test]
fn from_char() {
    let side = Side::from_char('w').unwrap();
    assert_eq!(side, Side::White);
    assert_eq!(side.as_u8(), 0b0);
    assert_eq!(side.neg(), Side::Black);
    assert_eq!(side.to_char(), 'w');
    let side = Side::try_from('W').unwrap();
    assert_eq!(side, Side::White);
    assert_eq!(side.to_char(), 'w');
    let side = Side::from_char('b').unwrap();
    assert_eq!(side, Side::Black);
    assert_eq!(side.as_u8(), 0b1);
    assert_eq!(!side, Side::White);
    assert_eq!(side.to_char(), 'b');
    let side = Side::try_from('B').unwrap();
    assert_eq!(side, Side::Black);
    assert_eq!(side.to_char(), 'b');

    let _ = Side::from_char('a').unwrap_err();
    let _ = Side::try_from('A').unwrap_err();
}

#[test]
fn from_u8() {
    let side = Side::from_u8(0b10);
    assert_eq!(side, Side::White);
    let side = Side::from_u8(0b01);
    assert_eq!(side, Side::Black);
}

#[test]
fn from_str() {
    let side = Side::from_str("w").unwrap();
    assert_eq!(side, Side::White);
    let side = Side::try_from("w".to_string()).unwrap();
    assert_eq!(side, Side::White);
}
