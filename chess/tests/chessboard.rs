use chess::{Chessboard, ChessboardRaw, EMPTY_POS, KIWIPETE_POS, Move, Piece, START_POS, Square};
use types::{Castling, GameResult, WinType};

#[test]
fn new_starting() {
    let chess = ChessboardRaw::from_fen(START_POS).unwrap();
    assert_eq!(chess.get_fen(), START_POS);

    let chess = Chessboard::from_fen(START_POS).unwrap();
    assert_eq!(chess.get_fen(), START_POS);

    assert_eq!(chess.at(Square::A1), Some(Piece::WHITE_ROOK));
}

#[test]
fn game_over() {
    static FEN: &str = "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3";
    let chess = Chessboard::from_fen(FEN).unwrap();
    println!("{:#?}", chess);
    println!("{}", chess);
    assert_eq!(chess.get_fen(), FEN);
    assert_eq!(chess.result, GameResult::BlackWin(WinType::Checkmate));
}

#[test]
fn new_empty() {
    let chess = ChessboardRaw::from_fen(EMPTY_POS).unwrap();
    assert_eq!(chess.get_fen(), EMPTY_POS);
}

#[test]
fn new_kiwipete() {
    let chess = Chessboard::from_fen(KIWIPETE_POS).unwrap();
    assert_eq!(chess.get_fen(), KIWIPETE_POS);
}

#[test]
fn en_passant() {
    let mut chess = Chessboard::from_fen(START_POS).unwrap();
    chess.make_move(Move::Standart {
        piece: Piece::from_char('P').unwrap(),
        from: Square::A2,
        to: Square::A4,
    });
    assert_eq!(chess.into_raw().en_passant.unwrap(), Square::A3);
}

#[test]
fn new_half() {
    const POS_HALF: &str = "8/8/8/8/8/8/8/8 w";
    const POS_FULL: &str = "8/8/8/8/8/8/8/8 w - - 0 1";
    let chess = ChessboardRaw::from_fen(POS_HALF).unwrap();
    assert_eq!(chess.get_fen(), POS_FULL);
}

#[test]
fn into_raw() {
    let chess = ChessboardRaw::from_fen(START_POS).unwrap();
    let chess: Chessboard = chess.try_into().unwrap();
    let raw: ChessboardRaw = chess.into();
    assert_eq!(raw.get_fen(), START_POS);
}

#[test]
fn default() {
    let chess = ChessboardRaw::default();
    assert_eq!(chess.get_fen(), EMPTY_POS);
}

#[test]
fn moves() {
    fn make_move(fen: &str, want: &str, m: Move) {
        println!("{}", fen);
        println!("{:?}", m);
        let mut chess = Chessboard::from_fen(fen).unwrap();
        println!("{}", chess.get_fen());
        if !chess.make_move(m) {
            panic!()
        }
        assert_eq!(chess.get_fen(), want);
    }

    const MOVES: &[(&str, &str, Move)] = &[
        (
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            "rnQq1k1r/pp2bppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R b KQ - 0 8",
            Move::Promotion {
                from: Square::D7,
                to: Square::C8,
                promotion: Piece::WHITE_QUEEN,
            },
        ),
        (
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQ1RK1 b - - 2 8",
            Move::Castling {
                castling: Castling::KingSide,
                from: Square::E1,
                to: Square::G1,
            },
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 1 1",
            Move::Castling {
                castling: Castling::KingSide,
                from: Square::E1,
                to: Square::G1,
            },
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            "r3k2r/8/8/8/8/8/8/2KR3R b kq - 1 1",
            Move::Castling {
                castling: Castling::QueenSide,
                from: Square::E1,
                to: Square::C1,
            },
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1",
            "r4rk1/8/8/8/8/8/8/R3K2R w KQ - 1 2",
            Move::Castling {
                castling: Castling::KingSide,
                from: Square::E8,
                to: Square::G8,
            },
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1",
            "2kr3r/8/8/8/8/8/8/R3K2R w KQ - 1 2",
            Move::Castling {
                castling: Castling::QueenSide,
                from: Square::E8,
                to: Square::C8,
            },
        ),
        (
            "4k3/8/8/8/3pP3/8/8/4K3 b - e3 0 1",
            "4k3/8/8/8/8/4p3/8/4K3 w - - 0 2",
            Move::EnPassant {
                from: Square::D4,
                to: Square::E3,
            },
        ),
    ];

    for (fen, want, m) in MOVES {
        make_move(fen, want, *m);
    }
}

#[test]
fn invalid_moves() {
    let mut board = Chessboard::from_fen("4k3/8/8/8/3pP3/8/8/4K3 b - e3 0 1").unwrap();
    let m = Move::EnPassant {
        from: Square::D3,
        to: Square::E2,
    };
    assert!(!board.make_move(m));
}
