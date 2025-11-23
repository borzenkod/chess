use types::{CastlingInfo, ChessError, GameResult, Move, MoveList, WinType};

use crate::{
    get_bishop_attacks, get_bishop_pinner, get_connection_direct, get_knight_attacks, get_pawn_attacks, get_rook_attacks, get_rook_pinner, Bitboard, ChessboardRaw, MoveGen, Piece, PieceType, Side, Square, START_POS
};

/// Chessboard representation
///
/// Checks for rules.
/// Useful for games
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chessboard {
    inner: ChessboardRaw,

    pub checkers: Bitboard,
    pub pinned: Bitboard,

    pub result: GameResult,

    pub moves_cache: MoveList,
}

impl Chessboard {
    pub const START: Self = match Self::from_fen(START_POS) {
        Ok(v) => v,
        Err(_) => unreachable!()
    };

    /// Create a new chessboard from a FEN
    pub const fn from_fen(fen: &str) -> Result<Self, ChessError> {
        let raw = ChessboardRaw::from_fen(fen);
        match raw {
            Ok(raw) => Self::from_raw(raw),
            Err(e) => Err(e),
        }
    }

    pub const fn from_raw(raw: ChessboardRaw) -> Result<Self, ChessError> {
        let mut s = Self {
            inner: raw,
            checkers: Bitboard::EMPTY,
            pinned: Bitboard::EMPTY,
            result: GameResult::None,
            moves_cache: MoveList::default_const(),
        };
        // TODO: Check for the correctness of the raw board
        Self::calculate_extra(&mut s);
        Ok(s)
    }

    #[cfg(not(feature = "no_std"))]
    pub fn get_fen(&self) -> String {
        self.inner.get_fen()
    }

    pub const fn at(&self, sq: Square) -> Option<Piece> {
        self.inner.at(sq)
    }

    pub const fn get_pieces(&self, piece: Piece) -> Bitboard {
        self.inner.get_pieces(piece)
    }

    pub const fn castling(&self) -> CastlingInfo {
        self.inner.castling
    }

    pub const fn colors(&self, side: Side) -> Bitboard {
        self.inner.colors[side.as_u8() as usize]
    }

    pub const fn side(&self) -> Side {
        self.inner.side
    }

    pub const fn en_passant(&self) -> Option<Square> {
        self.inner.en_passant
    }

    pub const fn occupied(&self) -> Bitboard {
        let white = self.inner.colors[0];
        let black = self.inner.colors[1];
        white.or(black)
    }

    /// Convert to raw chessboard
    pub const fn into_raw(self) -> ChessboardRaw {
        self.inner
    }

    pub const fn get_king(&self) -> Square {
        match self
            .inner
            .get_pieces(Piece::from_side_and_type(self.inner.side, PieceType::King))
            .first_square()
        {
            Some(sq) => sq,
            None => {
                unreachable!();
            }
        }
    }

    pub const fn calculate_extra(&mut self) {
        self.calculate_checkers();
        self.calculate_pinned();
        self.calculate_result();
    }

    pub const fn calculate_checkers(&mut self) {
        let us = self.inner.side;
        let them = us.neg();

        let king = self.get_king();
        self.checkers = Bitboard::EMPTY;

        let pawn_attacks = get_pawn_attacks(us, king);
        self.checkers = self.checkers.or(self
            .inner
            .get_pieces(Piece::from_side_and_type(them, PieceType::Pawn))
            .and(pawn_attacks));

        let knight_attacks = get_knight_attacks(king);
        self.checkers = self.checkers.or(self
            .inner
            .get_pieces(Piece::from_side_and_type(them, PieceType::Knight))
            .and(knight_attacks));

        let bishop_attacks = get_bishop_attacks(king, self.occupied());
        self.checkers = self.checkers.or((self
            .inner
            .get_pieces(Piece::from_side_and_type(them, PieceType::Bishop))
            .or(self
                .inner
                .get_pieces(Piece::from_side_and_type(them, PieceType::Queen))))
        .and(bishop_attacks));

        let rook_attacks = get_rook_attacks(king, self.occupied());
        self.checkers = self.checkers.or((self
            .inner
            .get_pieces(Piece::from_side_and_type(them, PieceType::Rook))
            .or(self
                .inner
                .get_pieces(Piece::from_side_and_type(them, PieceType::Queen))))
        .and(rook_attacks));
    }

    pub const fn calculate_pinned(&mut self) {
        self.pinned = Bitboard::EMPTY;
        let them = self.inner.side.neg();

        let enemy_queens = self
            .inner
            .get_pieces(Piece::from_side_and_type(them, PieceType::Queen));
        let enemy_rooks = self
            .inner
            .get_pieces(Piece::from_side_and_type(them, PieceType::Rook))
            .or(enemy_queens);
        let enemy_bishops = self
            .inner
            .get_pieces(Piece::from_side_and_type(them, PieceType::Bishop))
            .or(enemy_queens);

        let color_us = self.colors(self.inner.side);
        let king_sq = self.get_king();
        let rook_pinners = enemy_rooks.and(get_rook_pinner(king_sq, self.occupied(), color_us));
        let bishop_pinners =
            enemy_bishops.and(get_bishop_pinner(king_sq, self.occupied(), color_us));

        let mut pinned = rook_pinners.or(bishop_pinners);

        while let Some(pinner_sq) = pinned.next_const() {
            self.pinned = self
                .pinned
                .or(color_us.and(get_connection_direct(pinner_sq, self.get_king())));
        }
    }

    pub const fn move_new(&self, m: types::Move) -> Self {
        let mut board = *self;
        board.make_move(m);
        board
    }

    pub const fn make_move(&mut self, m: types::Move) -> bool {
        if !self.inner.make_move(m) {
            return false;
        }
        self.calculate_extra();
        true
    }

    pub const fn generate_moves(&mut self) -> MoveList {
        if self.moves_cache.is_empty() {
            self.moves_cache = crate::move_gen::generate_moves(self);
        }
        self.moves_cache
    }

    pub const fn calculate_result(&mut self) {
        self.moves_cache = MoveList::new(self.side(), self.en_passant());
        self.generate_moves();

        if self.moves_cache.is_empty() {
            if self.checkers.is_empty() {
                self.result = GameResult::Stalemate;
            } else {
                match self.inner.side {
                    Side::White => self.result = GameResult::BlackWin(WinType::Checkmate),
                    Side::Black => self.result = GameResult::WhiteWin(WinType::Checkmate),
                }
            }
        }

        if self.inner.half_moves >= 100 {
            self.result = GameResult::FiftyMoveRule;
        }

        let white_count = self.colors(Side::White).popcnt();
        let black_count = self.colors(Side::Black).popcnt();

        let white_minor = !self.get_pieces(Piece::WHITE_BISHOP).is_empty()
            || !self.get_pieces(Piece::WHITE_KNIGHT).is_empty();
        let black_minor = !self.get_pieces(Piece::BLACK_BISHOP).is_empty()
            || !self.get_pieces(Piece::BLACK_KNIGHT).is_empty();

        match (white_count, black_count) {
            (1, 1) => self.result = GameResult::InsufficientMaterial,
            (1, 2) if black_minor => self.result = GameResult::InsufficientMaterial,
            (2, 1) if white_minor => self.result = GameResult::InsufficientMaterial,

            (2, 2) => {
                let white_bishops = self.get_pieces(Piece::WHITE_BISHOP);
                let black_bishops = self.get_pieces(Piece::BLACK_BISHOP);

                let white_bishop_white_sq = white_bishops.overlaps(Bitboard::WHITE_SQUARES);
                let white_bishop_black_sq = white_bishops.overlaps(Bitboard::BLACK_SQUARES);
                let black_bishop_white_sq = black_bishops.overlaps(Bitboard::WHITE_SQUARES);
                let black_bishop_black_sq = black_bishops.overlaps(Bitboard::BLACK_SQUARES);

                if (white_bishop_white_sq && black_bishop_white_sq)
                    || (white_bishop_black_sq && black_bishop_black_sq)
                {
                    self.result = GameResult::InsufficientMaterial;
                }
            }
            _ => (),
        }
    }

    #[cfg(not(feature = "no_std"))]
    pub fn to_vec(&self) -> Vec<Move> {
        let mut vec = Vec::new();
        let mut move_gen = MoveGen::new(self.moves_cache);
        while let Some(m) = move_gen.next_const() {
            vec.push(m);
        }

        vec
    }
}

impl TryFrom<ChessboardRaw> for Chessboard {
    type Error = ChessError;

    fn try_from(value: ChessboardRaw) -> Result<Self, Self::Error> {
        Self::from_raw(value)
    }
}

impl core::fmt::Display for Chessboard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}
