#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameResult {
    WhiteWin(WinType),
    BlackWin(WinType),
    Stalemate,
    FiftyMoveRule,
    DrawOffer,
    InsufficientMaterial,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WinType {
    Checkmate,
    Surrounded,
    Resign,
}
