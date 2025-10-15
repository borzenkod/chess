/// The enum defining the castling rights
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
pub enum Castling {
    /// King side castling (or O-O)
    KingSide = 0,
    /// Queen side castling (or O-O-O)
    QueenSide = 1,
}
