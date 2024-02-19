use std::ops::{Index, IndexMut};

use crate::{bit_board_set::BitBoardSet, square::Color};

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct ColorMask {
    raw: [BitBoardSet; 2],
}

impl ColorMask {
    pub fn new() -> Self {
        Self {
            raw: [BitBoardSet::new(), BitBoardSet::new()],
        }
    }

    pub fn union(&self) -> BitBoardSet {
        self.raw.iter().fold(BitBoardSet::new(), |acc, x| acc | x)
    }
}

impl Index<Color> for ColorMask {
    type Output = BitBoardSet;

    fn index(&self, index: Color) -> &Self::Output {
        &self.raw[index as usize]
    }
}

impl IndexMut<Color> for ColorMask {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self.raw[index as usize]
    }
}
