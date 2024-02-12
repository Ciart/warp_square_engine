use std::ops::{BitAnd, BitOr, Index, IndexMut};

use crate::{bit_board::BitBoard, board_type::BoardType};

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct BitBoardSet {
    raw: [BitBoard; 7],
}

impl BitBoardSet {
    pub fn new() -> Self {
        Self {
            raw: [BitBoard::EMPTY; 7],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &BitBoard> {
        self.raw.iter()
    }

    pub fn union(&self) -> BitBoard {
        self.raw.iter().fold(BitBoard::EMPTY, |acc, x| acc | *x)
    }

    pub fn intersection(&self) -> BitBoard {
        self.raw.iter().fold(BitBoard::all(), |acc, x| acc & *x)
    }
}

impl Index<BoardType> for BitBoardSet {
    type Output = BitBoard;

    fn index(&self, index: BoardType) -> &Self::Output {
        &self.raw[index as usize]
    }
}

impl IndexMut<BoardType> for BitBoardSet {
    fn index_mut(&mut self, index: BoardType) -> &mut Self::Output {
        &mut self.raw[index as usize]
    }
}

impl BitOr<Self> for BitBoardSet {
    type Output = BitBoardSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();

        for (i, bit_board) in result.raw.iter_mut().enumerate() {
            *bit_board |= rhs.raw[i];
        }

        result
    }
}

impl BitOr<&Self> for BitBoardSet {
    type Output = BitBoardSet;

    fn bitor(self, rhs: &Self) -> Self::Output {
        let mut result = self.clone();

        for (i, bit_board) in result.raw.iter_mut().enumerate() {
            *bit_board |= rhs.raw[i];
        }

        result
    }
}

impl BitAnd<Self> for BitBoardSet {
    type Output = BitBoardSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();

        for (i, bit_board) in result.raw.iter_mut().enumerate() {
            *bit_board &= rhs.raw[i];
        }

        result
    }
}

impl BitAnd<&Self> for BitBoardSet {
    type Output = BitBoardSet;

    fn bitand(self, rhs: &Self) -> Self::Output {
        let mut result = self.clone();

        for (i, bit_board) in result.raw.iter_mut().enumerate() {
            *bit_board &= rhs.raw[i];
        }

        result
    }
}
