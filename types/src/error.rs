//! Errors that can occur when working with the chesslocal/chess

/// An error that can occur when working with the chesslocal/chess
#[derive(Debug, PartialEq, Eq)]
pub enum ChessError {
    /// Provided FEN is invalid
    InvalidFEN(FenError),
    /// Builder encountered an error
    // BuilderError(BuilderError),

    /// Castling rights are invalid
    InvalidCastlingInfo,

    /// Invalid side
    InvalidSide,
    /// Invalid square
    InvalidSquare,
    InvalidRank,
    InvalidFile,
    InvalidPiece,
}

/// Errors that occur when parsing FEN
#[derive(Debug, PartialEq, Eq)]
pub enum FenError {
    /// Invalid first part of FEN that is responsible for storing pieces
    InvalidFirstPart,
    /// Invalid second part of FEN that is responsible for storing side
    InvalidSecondPart,
    /// Invalid third part of FEN that is responsible for storing castling
    InvalidThirdPart,
    /// Invalid fourth part of FEN that is responsible for storing en passant
    InvalidFourthPart,
    /// Invalid fifth part of FEN that is responsible for storing half moves counter
    InvalidFifthPart,
    /// Invalid sixth part of FEN that is responsible for storing full moves counter
    InvalidSixthPart,
    /// Hit EOF unexpectedly
    EOF,
}
