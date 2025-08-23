use types::{File, Rank};

use crate::{ChessError, ChessboardRaw, FenError, Piece, Side, Square};

const IGNORE_WARNINGS: bool = true;

pub struct FenBuilder;

impl FenBuilder {
    pub fn get_fen(cr: &ChessboardRaw) -> String {
        let mut fen = String::with_capacity(80);

        let mut gap;
        for rank in (0..8).rev() {
            gap = 0;
            for file in 0..8 {
                let square = Square::at(File::from_u8(file), Rank::from_u8(rank));

                if let Some(piece) = cr.at(square) {
                    if gap > 0 {
                        fen.push_str(&gap.to_string());
                        gap = 0;
                    }

                    fen.push(piece.to_char());
                } else {
                    gap += 1;
                }
            }
            if gap != 0 {
                fen.push_str(&gap.to_string());
            }
            if rank != 0 {
                fen.push('/');
            }
        }

        fen.push(' ');
        fen.push(cr.side.to_char());

        fen.push(' ');
        let chars = cr.castling.to_chars();
        for i in 0..chars.0 {
            fen.push(chars.1[i]);
        }

        fen.push(' ');
        if let Some(en_passant) = cr.en_passant {
            let sq = en_passant.to_chars();
            fen.push(sq[0]);
            fen.push(sq[1]);
        } else {
            fen.push('-');
        }

        fen.push(' ');
        fen.push_str(&cr.half_moves.to_string());

        fen.push(' ');
        fen.push_str(&cr.full_moves.to_string());

        fen
    }

    pub const fn build_fen(fen: &str) -> Result<ChessboardRaw, ChessError> {
        let chars = fen.as_bytes();
        let mut board = ChessboardRaw::new();
        board.full_moves = 0;

        let mut i = 0;
        let mut index = Square::A8.as_u8();
        let mut part = 1;
        let mut prev = 0;
        loop {
            if part == 7 {
                return Ok(board);
            }
            if i >= chars.len() {
                if IGNORE_WARNINGS {
                    if board.full_moves == 0 {
                        board.full_moves = 1;
                    }
                    return Ok(board);
                } else {
                    return Err(ChessError::InvalidFEN(FenError::EOF));
                }
            }
            let char = chars[i];
            i += 1;

            if char == b' ' {
                part += 1;
                prev = char;
                continue;
            }

            match part {
                1 => {
                    if !Self::parse_first_part(char, &mut index, &mut board) {
                        return Err(ChessError::InvalidFEN(FenError::InvalidFirstPart));
                    }
                }
                2 => {
                    if !Self::parse_second_part(char, &mut board) {
                        return Err(ChessError::InvalidFEN(FenError::InvalidSecondPart));
                    }
                }
                3 => {
                    if !Self::parse_third_part(char, &mut board) {
                        return Err(ChessError::InvalidFEN(FenError::InvalidThirdPart));
                    }
                }
                4 if prev != b' ' => {
                    if !Self::parse_fourth_part(char, prev, &mut board) {
                        return Err(ChessError::InvalidFEN(FenError::InvalidFourthPart));
                    }
                }
                4 => {
                    if char == b'-' {
                        board.en_passant = None
                    }
                }
                5 => {
                    if !Self::parse_fifth_part(char, &mut board) {
                        return Err(ChessError::InvalidFEN(FenError::InvalidFifthPart));
                    }
                }
                6 => {
                    if !Self::parse_sixth_part(char, &mut board) {
                        return Err(ChessError::InvalidFEN(FenError::InvalidSixthPart));
                    }
                }

                _ => unreachable!(),
            }

            prev = char;
        }
    }

    #[rustfmt::skip]
    const fn parse_first_part(char: u8, index: &mut u8, cb: &mut ChessboardRaw) -> bool {
        match char {
            b'p' | b'r' | b'n' | b'b' | b'q' | b'k' |
            b'P' | b'R' | b'N' | b'B' | b'Q' | b'K' => {
                let square = Square::from_u8(*index);
                let piece = match Piece::from_char(char as char) {
                    Some(p) => p,
                    None => unreachable!()
                };
                cb.place_raw(square, piece);
                *index += 1;
            }
            b'1'..=b'8' => {
                *index += char - b'0';
            }
            b'/' => {
                *index -= 16;
            }
            _ => {
                return false;
            }
        }
        true
    }

    const fn parse_second_part(char: u8, cb: &mut ChessboardRaw) -> bool {
        match char {
            b'w' => {
                cb.side = Side::White;
            }
            b'b' => {
                cb.side = Side::Black;
            }
            _ => {
                return false;
            }
        }

        true
    }

    const fn parse_third_part(char: u8, cb: &mut ChessboardRaw) -> bool {
        cb.castling.update(char)
    }

    const fn parse_fourth_part(char: u8, prev: u8, cb: &mut ChessboardRaw) -> bool {
        let square = Square::from_chars(prev as char, char as char);
        cb.en_passant = square;

        square.is_some()
    }

    const fn parse_fifth_part(char: u8, cb: &mut ChessboardRaw) -> bool {
        match char {
            b'0'..=b'9' => {
                let number = char - b'0';
                cb.half_moves *= 10;
                cb.half_moves += number as usize;
            }

            _ => {
                return false;
            }
        }

        true
    }

    const fn parse_sixth_part(char: u8, cb: &mut ChessboardRaw) -> bool {
        match char {
            b'0'..=b'9' => {
                let number = char - b'0';
                cb.full_moves *= 10;
                cb.full_moves += number as usize;
            }

            _ => {
                return false;
            }
        }

        true
    }
}
