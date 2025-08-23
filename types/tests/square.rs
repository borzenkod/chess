// Rank
mod rank {
    use types::{Bitboard, Rank};

    #[test]
    fn bitboard() {
        let rank = Rank::Eighth;
        let bitboard = Bitboard::from_u64(0xFF00000000000000);
        assert_eq!(rank.bitboard(), bitboard);
    }

    #[test]
    fn to_char() {
        for c in '1'..='8' {
            let rank = Rank::from_char(c).unwrap();
            assert_eq!(rank.to_char(), c);
        }
    }

    #[test]
    fn to_char_invalid() {
        assert!(Rank::from_char('a').is_none())
    }

    #[test]
    #[should_panic]
    fn from_u8_panic() {
        let _ = Rank::from_u8(0b1111);
    }

    #[test]
    fn from_u8() {
        let rank = Rank::Eighth;
        let rank2 = Rank::from_u8(7);
        assert_eq!(rank, rank2);
        let rank3 = Rank::from_u8_safe(7).unwrap();
        assert_eq!(rank, rank3);
        assert!(Rank::from_u8_safe(8).is_none());
    }
}

// File
mod file {
    use types::{Bitboard, File};

    #[test]
    fn bitboard() {
        let file = File::H;
        let bitboard = Bitboard::from_u64(9259542123273814144);
        assert_eq!(file.bitboard(), bitboard);
    }

    #[test]
    fn to_char() {
        for c in 'a'..='h' {
            let file = File::from_char(c).unwrap();
            assert_eq!(file.to_char(), c);
        }
        assert!(File::from_char('i').is_none())
    }

    #[test]
    fn from_u8() {
        let file = File::H;
        let file2 = File::from_u8(7);
        assert_eq!(file, file2);
    }

    #[test]
    fn from_u8_safe() {
        let file = File::H;
        let file2 = File::from_u8_safe(7).unwrap();
        assert_eq!(file, file2);
        assert!(File::from_u8_safe(8).is_none());
    }

    #[test]
    #[should_panic]
    fn from_u8_panic() {
        let _ = File::from_u8(8);
    }
}

mod square {
    use types::{File, Rank, Square};

    #[test]
    fn from_u8() {
        let square = Square::from(42);
        assert_eq!(square, Square::C6);
        assert_eq!(square.rank(), Rank::Sixth);
        assert_eq!(square.file(), File::C)
    }

    #[test]
    fn at() {
        let square = Square::at(File::from_u8(4), Rank::from_u8(2));
        assert_eq!(square.as_u8(), 20);
        assert_eq!(square, Square::E3);
    }

    #[test]
    fn to_chars() {
        let square = Square::try_from("c4").unwrap();
        let chars = ['c', '4'];
        assert_eq!(square, Square::C4);
        assert_eq!(square.to_chars(), chars);

        let _ = Square::try_from("c9").unwrap_err();
        let _ = Square::try_from("c").unwrap_err();
        let _ = Square::try_from("").unwrap_err();
        let _ = Square::try_from("i2").unwrap_err();
    }

    #[test]
    fn shift() {
        let sq = Square::E6;

        assert_eq!(sq.shift(types::Directions::Down), Some(Square::E5));
        assert_eq!(sq.shift(types::Directions::Up), Some(Square::E7));
        assert_eq!(sq.shift(types::Directions::Left), Some(Square::D6));
        assert_eq!(sq.shift(types::Directions::Right), Some(Square::F6));
        assert_eq!(sq.shift(types::Directions::UpLeft), Some(Square::D7));
        assert_eq!(sq.shift(types::Directions::DownLeft), Some(Square::D5));
        assert_eq!(sq.shift(types::Directions::UpRight), Some(Square::F7));
        assert_eq!(sq.shift(types::Directions::DownRight), Some(Square::F5));

        let sq = Square::A1;
        assert_eq!(sq.shift(types::Directions::Left), None);
        assert_eq!(sq.shift(types::Directions::Down), None);
        assert_eq!(sq.shift(types::Directions::DownLeft), None);

        let sq = sq.shift(types::Directions::Right).unwrap();
        assert_eq!(sq.shift(types::Directions::Down), None);
        assert_eq!(sq.shift(types::Directions::DownLeft), None);

        let sq = Square::A8;
        assert_eq!(sq.shift(types::Directions::Left), None);
        assert_eq!(sq.shift(types::Directions::Up), None);
        assert_eq!(sq.shift(types::Directions::UpLeft), None);

        let sq = sq.shift(types::Directions::Right).unwrap();
        // assert_eq!(sq.shift(types::Directions::Left), None);
        assert_eq!(sq.shift(types::Directions::Up), None);
        assert_eq!(sq.shift(types::Directions::UpLeft), None);
        assert_eq!(sq.shift(types::Directions::UpRight), None);

        let sq = Square::H8;
        assert_eq!(sq.shift(types::Directions::Right), None);
        assert_eq!(sq.shift(types::Directions::Up), None);
        assert_eq!(sq.shift(types::Directions::UpRight), None);

        let sq = sq.shift(types::Directions::Left).unwrap();
        assert_eq!(sq.shift(types::Directions::Up), None);
        assert_eq!(sq.shift(types::Directions::UpRight), None);
        assert_eq!(sq.shift(types::Directions::UpLeft), None);

        let sq = Square::H1;
        assert_eq!(sq.shift(types::Directions::Right), None);
        assert_eq!(sq.shift(types::Directions::Down), None);
        assert_eq!(sq.shift(types::Directions::DownRight), None);
    }
}
