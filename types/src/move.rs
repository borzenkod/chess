use crate::{Castling, Piece, Square};

/// The enum defining the castling rights and the player side
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    /// Like e2e4
    Standart {
        piece: Piece,
        from: Square,
        to: Square,
    },
    // Like e8Q
    Promotion {
        from: Square,
        to: Square,
        promotion: Piece,
    },
    // Like e4xd3
    EnPassant {
        from: Square,
        to: Square,
    },
    // Like O-O
    Castling {
        castling: Castling,
        from: Square,
        to: Square,
    },
    // Like Q@e8
    //
    // Used by editors to put a piece on a square
    //
    // This move exists in chess variations like Crazyhouse
    Put {
        piece: Piece,
        to: Square,
    },
    // This is a custom type used for editors to remove the piece from square
    //
    // This does not exist in the move format
    Remove {
        from: Square,
    },
}
