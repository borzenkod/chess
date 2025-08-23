mod generators;
mod tables;

pub use generators::{Bishop, King, Knight, Pawn, Queen, Rook};
pub use generators::{generate_moves, perft};
pub use tables::*;
