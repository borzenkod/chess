#![feature(coverage_attribute)]

mod bitboard;
mod castling;
mod castling_info;
mod error;
mod game_result;
mod magic;
mod r#move;
mod move_list;
mod piece;
mod side;
mod square;

pub use bitboard::{Bitboard, Directions};
pub use castling::Castling;
pub use castling_info::CastlingInfo;
pub use error::{ChessError, FenError};
pub use game_result::{GameResult, WinType};
pub use magic::Magic;
pub use r#move::Move;
pub use move_list::{MoveList, Moves};
pub use piece::{Piece, PieceType};
pub use side::Side;
pub use square::{File, Rank, Square};
