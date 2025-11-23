use types::{Castling, ChessError, Directions, File, Rank};

use crate::{
    Bitboard, CastlingInfo, Chessboard, Piece, PieceType, Side, Square,
    chessboard::{FenBuilder},
};

/// Raw chessboard representation
///
/// That allows us to work with the chessboard directly, bypassing the chess rules
/// Useful with editors/sandboxes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChessboardRaw {
    pub pieces: [Bitboard; 6],
    pub colors: [Bitboard; 2],

    pub side: Side,
    pub castling: CastlingInfo,

    pub en_passant: Option<Square>,

    pub half_moves: usize,
    pub full_moves: usize,
}

impl ChessboardRaw {
    pub(super) const fn new() -> Self {
        Self {
            pieces: [Bitboard::EMPTY; 6],
            colors: [Bitboard::EMPTY; 2],
            side: Side::White,
            castling: CastlingInfo::EMPTY,
            en_passant: None,
            half_moves: 0,
            full_moves: 1,
        }
    }

    /// Create a new chessboard from a FEN
    pub const fn from_fen(fen: &str) -> Result<Self, ChessError> {
        FenBuilder::build_fen(fen)
    }

    #[cfg(not(feature = "no_std"))]
    pub fn get_fen(&self) -> String {
        FenBuilder::get_fen(self)
    }

    pub const fn get_pieces(&self, piece: Piece) -> Bitboard {
        let side = piece.side();
        let piece_type = piece.piece_type();
        let pieces = self.pieces[piece_type as usize];
        let color = self.colors[side as usize];
        pieces.and(color)
    }

    pub const fn at(&self, square: Square) -> Option<Piece> {
        let square: Bitboard = square.bitboard();
        let color = if self.colors[Side::White as usize].overlaps(square) {
            Side::White
        } else if self.colors[Side::Black as usize].overlaps(square) {
            Side::Black
        } else {
            return None;
        };

        let piece = {
            // Some clever bitboard magic
            let mut index = 0;
            index += self.pieces[PieceType::Pawn as usize].overlaps(square) as usize;
            index <<= 1;
            index += self.pieces[PieceType::Rook as usize].overlaps(square) as usize;
            index <<= 1;
            index += self.pieces[PieceType::Knight as usize].overlaps(square) as usize;
            index <<= 1;
            index += self.pieces[PieceType::Bishop as usize].overlaps(square) as usize;
            index <<= 1;
            index += self.pieces[PieceType::Queen as usize].overlaps(square) as usize;
            index <<= 1;
            index += self.pieces[PieceType::King as usize].overlaps(square) as usize;
            match index {
                0b000001 => PieceType::King,
                0b000010 => PieceType::Queen,
                0b000100 => PieceType::Bishop,
                0b001000 => PieceType::Knight,
                0b010000 => PieceType::Rook,
                0b100000 => PieceType::Pawn,
                _ => unreachable!(),
            }
        };

        Some(Piece::from_side_and_type(color, piece))
    }

    pub const fn make_move(&mut self, m: types::Move) -> bool {
        let us = self.side;

        self.en_passant = None;

        let mut reset_halfmoves = false;
        match m {
            types::Move::Standard { piece, from, to } => {
                match piece.piece_type() {
                    PieceType::Pawn => {
                        let start_rank = from.rank().as_u8();
                        let end_rank = to.rank().as_u8();
                        let module = start_rank.wrapping_sub(end_rank);

                        match us {
                            Side::Black if module == 2 => {
                                self.en_passant = to.shift(Directions::Up);
                            }
                            Side::White if module == u8::MAX - 1 => {
                                self.en_passant = to.shift(Directions::Down);
                            }
                            _ => (),
                        }

                        reset_halfmoves = true;
                    }
                    PieceType::Rook => match (us, from) {
                        (Side::White, Square::A1) => {
                            self.castling.set(Castling::QueenSide, Side::White, false);
                        }
                        (Side::White, Square::H1) => {
                            self.castling.set(Castling::KingSide, Side::White, false);
                        }
                        (Side::Black, Square::A8) => {
                            self.castling.set(Castling::QueenSide, Side::Black, false);
                        }
                        (Side::Black, Square::H8) => {
                            self.castling.set(Castling::KingSide, Side::Black, false);
                        }

                        _ => (),
                    },
                    PieceType::King => {
                        self.remove_castling_for(us);
                    }
                    _ => (),
                }
                let piece_to = self.at(to);
                match piece_to {
                    None => {
                        self.move_piece_raw(from, to, piece);
                    }
                    Some(_piece_to) => {
                        reset_halfmoves = true;
                        unsafe {
                            self.remove_raw(to);
                        }
                        self.move_piece_raw(from, to, piece);
                    }
                }
            }
            types::Move::Promotion {
                from,
                to,
                promotion,
            } => {
                let piece_to = self.at(to);
                if let Some(_piece) = piece_to {
                    unsafe {
                        self.remove_raw(to);
                    }
                }
                unsafe {
                    self.remove_raw(from);
                }
                self.place_raw(to, promotion);
                reset_halfmoves = true;
            }
            types::Move::EnPassant { from, to } => {
                let Some(piece) = self.at(from) else {
                    return false;
                };
                let target = match us {
                    Side::White => to.shift(Directions::Down),
                    Side::Black => to.shift(Directions::Up),
                };
                let Some(target) = target else {
                    return false;
                };
                unsafe {
                    self.remove_raw(target);
                    self.remove_raw(from);
                }
                self.place_raw(to, piece);
                reset_halfmoves = true;
            }
            types::Move::Castling { castling, from, to } => {
                let (rook_start, rook_end) = match (us, castling) {
                    (Side::White, Castling::KingSide) => (Square::H1, Square::F1),
                    (Side::White, Castling::QueenSide) => (Square::A1, Square::D1),
                    (Side::Black, Castling::KingSide) => (Square::H8, Square::F8),
                    (Side::Black, Castling::QueenSide) => (Square::A8, Square::D8),
                };

                let rook = Piece::from_side_and_type(us, PieceType::Rook);
                self.move_piece_raw(rook_start, rook_end, rook);
                let king = Piece::from_side_and_type(us, PieceType::King);
                self.move_piece_raw(from, to, king);
                self.remove_castling_for(us);
            }
            types::Move::Put { piece, to } => {
                let piece_to = self.at(to);
                if piece_to.is_some() {
                    unsafe {
                        self.remove_raw(to);
                    }
                }
                self.place_raw(to, piece);
            }
            types::Move::Remove { from } => {
                let piece_from = self.at(from);
                if piece_from.is_some() {
                    unsafe {
                        self.remove_raw(from);
                    }
                }
            }
        };

        if reset_halfmoves {
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }

        if us.as_u8() == Side::Black.as_u8() {
            self.full_moves += 1;
        }
        self.toggle_turn();

        true
    }

    pub const fn remove_castling_for(&mut self, side: Side) {
        self.castling.set(Castling::KingSide, side, false);
        self.castling.set(Castling::QueenSide, side, false);
    }

    pub const fn move_piece_raw(&mut self, from: Square, to: Square, piece: Piece) {
        let t = piece.piece_type();
        let side = piece.side();
        let mut pieces = self.pieces[t.as_u8() as usize];
        pieces = pieces.xor(from.bitboard().or(to.bitboard()));
        self.pieces[t.as_u8() as usize] = pieces;
        let mut colors = self.colors[side as usize];
        colors = colors.xor(from.bitboard().or(to.bitboard()));
        self.colors[side as usize] = colors;
    }

    /// Removes the piece from the board
    ///
    /// # Safety
    /// Caller must ensure that the target square is not empty
    pub const unsafe fn remove_raw(&mut self, to: Square) {
        let t = unsafe { self.at(to).unwrap_unchecked().piece_type() };
        let side = unsafe { self.at(to).unwrap_unchecked().side() };
        let mut pieces = self.pieces[t.as_u8() as usize];
        pieces = pieces.xor(to.bitboard());
        self.pieces[t.as_u8() as usize] = pieces;
        let mut colors = self.colors[side as usize];
        colors = colors.xor(to.bitboard());
        self.colors[side as usize] = colors;
    }

    pub const fn place_raw(&mut self, to: Square, piece: Piece) {
        let t = piece.piece_type();
        let side = piece.side();
        let mut pieces = self.pieces[t.as_u8() as usize];
        pieces = pieces.xor(to.bitboard());
        self.pieces[t.as_u8() as usize] = pieces;
        let mut colors = self.colors[side as usize];
        colors = colors.xor(to.bitboard());
        self.colors[side as usize] = colors;
    }

    pub const fn toggle_turn(&mut self) {
        self.side = self.side.neg();
    }
}

impl Default for ChessboardRaw {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Chessboard> for ChessboardRaw {
    fn from(val: Chessboard) -> Self {
        val.into_raw()
    }
}

impl core::fmt::Display for ChessboardRaw {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for rank in Rank::ALL.into_iter().rev() {
            for file in File::ALL {
                let sq = Square::at(file, rank);
                if let Some(piece) = self.at(sq) {
                    write!(f, "{} ", piece.to_char())?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
