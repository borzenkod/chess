use chess::{Chessboard, START_POS, perft as _perft};

mod movegen {
    use super::*;

    fn get_board() -> Chessboard {
        Chessboard::from_fen(START_POS).unwrap()
    }

    #[test]
    fn illegal_depth() {
        let mut board = get_board();
        assert_eq!(_perft(&mut board, 0), 0);
    }

    #[test]
    fn castling() {
        let mut board = Chessboard::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let perft = _perft(&mut board, 1);

        assert_eq!(perft, 26);
    }

    #[test]
    fn promotion() {
        let mut board = Chessboard::from_fen("1K6/P7/8/8/8/8/7p/7k w - - 1 1").unwrap();
        let perft = _perft(&mut board, 1);

        assert_eq!(perft, 8);
    }

    #[test]
    fn depth_1() {
        let mut board = get_board();
        assert_eq!(perft_f(&mut board, 1), 20);
    }

    #[test]
    fn depth_2() {
        let mut board = get_board();
        assert_eq!(perft_f(&mut board, 2), 400);
    }

    #[test]
    fn depth_3() {
        let mut board = get_board();
        let perft = _perft(&mut board, 3);

        assert_eq!(perft, 8902);
    }

    #[test]
    fn depth_4() {
        let mut board = get_board();
        let perft = _perft(&mut board, 4);

        assert_eq!(perft, 197281);
    }

    #[test]
    fn depth_5() {
        let mut board = get_board();
        assert_eq!(perft_f(&mut board, 5), 4865609);
    }

    #[test]
    fn pawn_pin() {
        let mut board = Chessboard::from_fen("4k3/4r3/8/8/4P3/8/8/4K3 w - - 0 1").unwrap();
        assert_eq!(perft_f(&mut board, 1), 6);
    }

    #[test]
    fn bishop_pin() {
        let mut board = Chessboard::from_fen("4r1K1/4k1B1/8/8/8/8/8/6r1 w - - 0 1").unwrap();
        assert_eq!(perft_f(&mut board, 1), 1);
    }

    #[test]
    fn king_double() {
        let mut board = Chessboard::from_fen("4r1K1/4k3/8/8/8/8/8/6r1 w - - 0 1").unwrap();
        assert_eq!(perft_f(&mut board, 1), 1);
    }
}

#[test]
fn tricky_positions() {
    fn test(fen: &str, perft1: usize, perft2: usize, perft3: usize) {
        let mut board = Chessboard::from_fen(fen).unwrap();
        let p1 = perft_f(&mut board, 1);
        if perft1 != p1 {
            println!("{}", board);
            let mut moves = chess::generate_moves(&mut board);
            while let Some(m) = moves.pop() {
                println!("{}", m);
            }
            panic!("Expecting {} but got {}", perft1, p1);
        }
        if perft2 == 0 {
            return;
        }
        let p2 = perft_f(&mut board, 2);
        assert_eq!(p2, perft2);
        if perft3 == 0 {
            return;
        }
        let p3 = perft_f(&mut board, 3);
        assert_eq!(p3, perft3);
    }

    test("8/8/5b2/8/8/8/7k/KR5r w - - 0 1", 1, 0, 0);
    test("4k3/8/8/KPp4r/8/8/8/8 w - c6 0 1", 4, 68, 0);
    test(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        46,
        2079,
        89890,
    );
    test(
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        44,
        1486,
        0,
    );
}

fn perft_f(cb: &mut Chessboard, depth: u32) -> usize {
    let prev_cb = cb.clone();
    let perft = _perft(cb, depth);
    assert_eq!(cb.clone(), prev_cb);

    perft
}
