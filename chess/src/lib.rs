#![doc = include_str!("../../README.md")]
#![feature(const_destruct)]
#![cfg_attr(feature = "no_std", no_std)]
#![warn(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::nursery,
    clippy::cargo,
    clippy::indexing_slicing,
    clippy::print_stdout
)]
#![deny(
    deprecated,
    missing_debug_implementations,
    unreachable_code,
    elided_lifetimes_in_paths
)]
#![allow(clippy::indexing_slicing)]

mod chessboard;
mod move_gen;

/// The starting position
pub static START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub static EMPTY_POS: &str = "8/8/8/8/8/8/8/8 w - - 0 1";
pub static KIWIPETE_POS: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

pub use chessboard::{Chessboard, ChessboardRaw};
pub use move_gen::*;
pub use types::*;

pub static mut ERROR: Option<ChessError> = None;
