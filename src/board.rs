use crate::{board_type::BoardType, square::Level};

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct Board {
    pub board_type: BoardType,
    pub level: Level,
}

impl Board {
    pub fn new(board_type: BoardType, level: Level) -> Self {
        Self { board_type, level }
    }
}
