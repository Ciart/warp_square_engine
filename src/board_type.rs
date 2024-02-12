#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub enum BoardType {
    White,
    Neutral,
    Black,
    WhiteQueen,
    WhiteKing,
    BlackQueen,
    BlackKing,
}

impl BoardType {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::White,
            Self::Neutral,
            Self::Black,
            Self::WhiteQueen,
            Self::WhiteKing,
            Self::BlackQueen,
            Self::BlackKing,
        ]
        .iter()
        .copied()
    }

    pub fn is_main(&self) -> bool {
        match self {
            Self::White | Self::Neutral | Self::Black => true,
            _ => false,
        }
    }

    pub fn is_attack(&self) -> bool {
        !self.is_main()
    }
}
