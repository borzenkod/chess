/// The enum defining the castling rights
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Castling {
    /// King side castling (or O-O)
    KingSide = 0,
    /// Queen side castling (or O-O-O)
    QueenSide = 1,
}
