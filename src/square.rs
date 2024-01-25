use std::{mem::transmute, ops::Not};

use pyo3::pyclass;

use crate::bit_board::BitBoard;

#[pyclass]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub enum Rank {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Rank {
    pub fn from_u8(i: u8) -> Self {
        unsafe { transmute(i.clamp(Self::Zero as u8, Self::Nine as u8)) }
    }

    pub fn down(&self) -> Self {
        Self::from_u8(*self as u8 - 1)
    }

    pub fn up(&self) -> Self {
        Self::from_u8(*self as u8 + 1)
    }
}

pub const NUM_RANKS: u8 = 10;

#[pyclass]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub enum File {
    Z,
    A,
    B,
    C,
    D,
    E,
}

impl File {
    pub fn from_u8(i: u8) -> Self {
        unsafe { transmute(i.clamp(Self::Z as u8, Self::E as u8)) }
    }

    pub fn left(&self) -> Self {
        Self::from_u8(*self as u8 - 1)
    }

    pub fn right(&self) -> Self {
        Self::from_u8(*self as u8 + 1)
    }
}

pub const NUM_FILES: u8 = 6;

#[pyclass]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub enum Level {
    White = 1,
    Neutral,
    Black,
    QL1,
    QL2,
    QL3,
    QL4,
    QL5,
    QL6,
    KL1,
    KL2,
    KL3,
    KL4,
    KL5,
    KL6,
}

impl Level {
    pub fn from_u8(i: u8) -> Self {
        unsafe { transmute(i.clamp(Self::White as u8, Self::KL6 as u8)) }
    }

    /// BitBoard Level 영역 값으로 변환
    pub fn into_bit_board(&self) -> BitBoard {
        BitBoard::from_bits_retain((*self as u64) << BitBoard::LEVEL_SHIFT)
    }

    /// BitBoard 존재 가능 영역
    pub fn get_bit_board(&self) -> BitBoard {
        match self {
            Level::White => BitBoard::WHITE_SET,
            Level::Neutral => BitBoard::NEUTRAL_SET,
            Level::Black => BitBoard::BLACK_SET,
            Level::QL1 => BitBoard::QL1_SET,
            Level::QL2 => BitBoard::QL2_SET,
            Level::QL3 => BitBoard::QL3_SET,
            Level::QL4 => BitBoard::QL4_SET,
            Level::QL5 => BitBoard::QL5_SET,
            Level::QL6 => BitBoard::QL6_SET,
            Level::KL1 => BitBoard::KL1_SET,
            Level::KL2 => BitBoard::KL2_SET,
            Level::KL3 => BitBoard::KL3_SET,
            Level::KL4 => BitBoard::KL4_SET,
            Level::KL5 => BitBoard::KL5_SET,
            Level::KL6 => BitBoard::KL6_SET,
        }
    }
}

#[pyclass]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::White, Self::Black].iter().copied()
    }
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct Square {
    pub rank: Rank,
    pub file: File,
    pub level: Level,
}

impl Square {
    pub fn new(rank: Rank, file: File, level: Level) -> Square {
        Square { rank, file, level }
    }

    pub fn down(&self) -> Self {
        Self::new(self.rank.down(), self.file, self.level)
    }

    pub fn up(&self) -> Self {
        Self::new(self.rank.up(), self.file, self.level)
    }

    pub fn left(&self) -> Self {
        Self::new(self.rank, self.file.left(), self.level)
    }

    pub fn right(&self) -> Self {
        Self::new(self.rank, self.file.right(), self.level)
    }

    pub fn forward(&self, color: Color) -> Self {
        match color {
            Color::White => self.up(),
            Color::Black => self.down(),
        }
    }

    pub fn backward(&self, color: Color) -> Self {
        match color {
            Color::White => self.down(),
            Color::Black => self.up(),
        }
    }
}
