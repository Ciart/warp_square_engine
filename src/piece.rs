use crate::{
    bit_board::BitBoard,
    game::Game,
    piece_move::PieceMove,
    square::{Color, Square},
};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const NUM_PIECES: usize = 6;

impl PieceType {
    pub fn compute_moves(&self, square: &Square, game: &Game) -> Vec<PieceMove> {
        match self {
            Self::Pawn => self.compute_pawn_moves(square, game),
            Self::Knight => self.compute_knight_moves(square, game),
            Self::Bishop => self.compute_bishop_moves(square, game),
            Self::Rook => self.compute_rook_moves(square, game),
            Self::Queen => self.compute_queen_moves(square, game),
            Self::King => self.compute_king_moves(square, game),
        }
    }

    fn compute_pawn_moves(&self, square: &Square, game: &Game) -> Vec<PieceMove> {
        todo!()
    }

    fn compute_knight_moves(&self, square: &Square, game: &Game) -> Vec<PieceMove> {
        todo!()
    }

    fn compute_bishop_moves(&self, square: &Square, game: &Game) -> Vec<PieceMove> {
        todo!()
    }

    fn compute_rook_moves(&self, square: &Square, game: &Game) -> Vec<PieceMove> {
        todo!()
    }

    fn compute_queen_moves(&self, square: &Square, game: &Game) -> Vec<PieceMove> {
        [
            self.compute_bishop_moves(square, game),
            self.compute_rook_moves(square, game),
        ]
        .concat()
    }

    fn compute_king_moves(&self, square: &Square, game: &Game) -> Vec<PieceMove> {
        todo!()
    }

    pub fn get_char(&self, color: Color) -> &'static str {
        match color {
            Color::White => match self {
                Self::Pawn => "P",
                Self::Knight => "N",
                Self::Bishop => "B",
                Self::Rook => "R",
                Self::Queen => "Q",
                Self::King => "K",
            },
            Color::Black => match self {
                Self::Pawn => "p",
                Self::Knight => "n",
                Self::Bishop => "b",
                Self::Rook => "r",
                Self::Queen => "q",
                Self::King => "k",
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub position: BitBoard,
    pub attacks: BitBoard,
}

impl Piece {
    pub fn new(position: BitBoard, piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type,
            color,
            position,
            attacks: BitBoard::EMPTY,
        }
    }

    // pub fn compute_moves(&self, game: &Game) -> Vec<PieceMove> {
    //     self.piece_type.compute_moves(&self.position.to_square(), game)
    // }

    pub fn get_square(&self) -> Square {
        BitBoard::into_square(self.position)
    }

    pub fn get_char(&self) -> &'static str {
        self.piece_type.get_char(self.color)
    }
}
