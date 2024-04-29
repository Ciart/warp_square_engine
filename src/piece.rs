use pyo3::pyclass;

use crate::{
    bit_board::BitBoard,
    bit_board_set::BitBoardSet,
    board_set::BoardSet,
    board_type::BoardType,
    square::{Color, Rank, Square},
};

#[pyclass]
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
    pub attacks: BitBoardSet,
    pub is_moved: bool,
}

impl Piece {
    pub fn new(position: BitBoard, piece_type: PieceType, color: Color, is_moved: bool) -> Self {
        Self {
            piece_type,
            color,
            position,
            attacks: BitBoardSet::new(),
            is_moved: is_moved,
        }
    }

    pub fn get_square(&self) -> Square {
        BitBoard::into_square(&self.position)
    }

    pub fn get_char(&self) -> &'static str {
        self.piece_type.get_char(self.color)
    }

    pub fn get_attack_squares(&self, board_set: &BoardSet) -> Vec<Square> {
        let mut result = Vec::new();

        for board_type in BoardType::iter() {
            for bit_square in self.attacks[board_type].iter() {
                result.push(
                    (bit_square | board_set.convert_level(board_type).into_bit_board()).into_square(),
                );
            }
        }

        result
    }

    pub fn compute_ray_occupied(board_set: &BoardSet) -> BitBoard {
        let occupied = (board_set.occupied_piece.union() | &board_set.occupied_void).intersection();

        let board_area = board_set
            .boards
            .iter()
            .fold(BitBoard::EMPTY, |acc, board| acc | board.level.get_area());

        occupied & board_area
    }

    pub fn is_promotion_position(&self) -> bool {
        if self.piece_type != PieceType::Pawn {
            return false;
        }

        let end_rank = match self.color {
            Color::White => {
                if self.position.get_level().is_main() {
                    Rank::Eight
                } else {
                    Rank::Nine
                }
            }
            Color::Black => {
                if self.position.get_level().is_main() {
                    Rank::One
                } else {
                    Rank::Zero
                }
            }
        };

        self.position.get_rank() == end_rank
    }

    pub fn update_attacks(&mut self, board_set: &BoardSet) {
        self.attacks = match self.piece_type {
            PieceType::Pawn => self.compute_pawn_attacks(board_set),
            PieceType::Knight => self.compute_knight_attacks(board_set),
            PieceType::Bishop => self.compute_bishop_attacks(board_set),
            PieceType::Rook => self.compute_rook_attacks(board_set),
            PieceType::Queen => self.compute_queen_attacks(board_set),
            PieceType::King => self.compute_king_attacks(board_set),
        };
    }

    pub fn compute_pawn_attacks(&self, board_set: &BoardSet) -> BitBoardSet {
        let position = self.position.remove_level();
        let occupied = (board_set.occupied_piece.union() | &board_set.occupied_void).intersection();

        let mut attacks = BitBoardSet::new();

        // 이동 행마
        {
            let mut destination = position.forward(self.color);

            if !self.is_moved && !occupied.contains(destination) {
                destination |= destination.forward(self.color);
            }

            let empty_boards = board_set.get_empty_squares(destination, None);

            for (board_type, square, is_empty) in &empty_boards {
                if *is_empty {
                    attacks[*board_type] |= *square;
                }
            }
        }

        // 공격 행마
        {
            let destination =
                position.forward_left(self.color) | position.forward_right(self.color);

            let empty_boards = board_set.get_empty_squares(destination, Some(self.color));

            for (board_type, square, is_empty) in &empty_boards {
                if *is_empty {
                    // 앙파상 체크
                    if let Some(en_passant) = &board_set.en_passant {
                        if *square == en_passant.position.remove_level() {
                            attacks[*board_type] |= *square;
                        }
                    }
                } else {
                    attacks[*board_type] |= *square;
                }
            }
        }

        attacks
    }

    pub fn compute_knight_attacks(&self, board_set: &BoardSet) -> BitBoardSet {
        let position = self.position.remove_level();

        let mut attacks = BitBoardSet::new();
        let mut destination = BitBoard::EMPTY;

        destination |=
            BitBoard::from_bits_retain(position.bits() >> 21 & (!BitBoard::NINE_RANKS).bits());
        destination |=
            BitBoard::from_bits_retain(position.bits() >> 19 & (!BitBoard::ZERO_RANKS).bits());
        destination |= BitBoard::from_bits_retain(
            position.bits() >> 12 & (!(BitBoard::NINE_RANKS | BitBoard::EIGHT_RANKS)).bits(),
        );
        destination |= BitBoard::from_bits_retain(
            position.bits() >> 8 & (!(BitBoard::ZERO_RANKS | BitBoard::ONE_RANKS)).bits(),
        );
        destination |= BitBoard::from_bits_retain(
            position.bits() << 8 & (!(BitBoard::NINE_RANKS | BitBoard::EIGHT_RANKS)).bits(),
        );
        destination |= BitBoard::from_bits_retain(
            position.bits() << 12 & (!(BitBoard::ZERO_RANKS | BitBoard::ONE_RANKS)).bits(),
        );
        destination |=
            BitBoard::from_bits_retain(position.bits() << 19 & (!BitBoard::NINE_RANKS).bits());
        destination |=
            BitBoard::from_bits_retain(position.bits() << 21 & (!BitBoard::ZERO_RANKS).bits());

        let empty_boards = board_set.get_empty_squares(destination, Some(!self.color));

        for (board_type, square, is_empty) in &empty_boards {
            if *is_empty {
                attacks[*board_type] |= *square;
            }
        }

        attacks
    }

    pub fn compute_bishop_attacks(&self, board_set: &BoardSet) -> BitBoardSet {
        let position = self.position.remove_level();
        let occupied = Self::compute_ray_occupied(board_set);

        let mut attacks = BitBoardSet::new();
        let mut destination = BitBoard::EMPTY;

        destination |= position.ray(occupied, |current| current.down_left());
        destination |= position.ray(occupied, |current| current.down_right());
        destination |= position.ray(occupied, |current| current.up_left());
        destination |= position.ray(occupied, |current| current.up_right());

        let empty_boards = board_set.get_empty_squares(destination, Some(!self.color));

        for (board_type, square, is_empty) in &empty_boards {
            if *is_empty {
                attacks[*board_type] |= *square;
            }
        }

        attacks
    }

    pub fn compute_rook_attacks(&self, board_set: &BoardSet) -> BitBoardSet {
        let position = self.position.remove_level();
        let occupied = Self::compute_ray_occupied(board_set);

        let mut attacks = BitBoardSet::new();
        let mut destination = BitBoard::EMPTY;

        destination |= position.ray(occupied, |current| current.down());
        destination |= position.ray(occupied, |current| current.up());
        destination |= position.ray(occupied, |current| current.left());
        destination |= position.ray(occupied, |current| current.right());

        let empty_boards = board_set.get_empty_squares(destination, Some(!self.color));

        for (board_type, square, is_empty) in &empty_boards {
            if *is_empty {
                attacks[*board_type] |= *square;
            }
        }

        attacks
    }

    pub fn compute_queen_attacks(&self, board_set: &BoardSet) -> BitBoardSet {
        let position = self.position.remove_level();
        let occupied = Self::compute_ray_occupied(board_set);

        let mut attacks = BitBoardSet::new();
        let mut destination = BitBoard::EMPTY;

        destination |= position.ray(occupied, |current| current.down());
        destination |= position.ray(occupied, |current| current.up());
        destination |= position.ray(occupied, |current| current.left());
        destination |= position.ray(occupied, |current| current.right());
        destination |= position.ray(occupied, |current| current.down_left());
        destination |= position.ray(occupied, |current| current.down_right());
        destination |= position.ray(occupied, |current| current.up_left());
        destination |= position.ray(occupied, |current| current.up_right());

        let empty_boards = board_set.get_empty_squares(destination, Some(!self.color));

        for (board_type, square, is_empty) in &empty_boards {
            if *is_empty {
                attacks[*board_type] |= *square;
            }
        }

        attacks
    }

    pub fn compute_king_attacks(&self, board_set: &BoardSet) -> BitBoardSet {
        let position = self.position.remove_level();

        let mut attacks = BitBoardSet::new();
        let mut destination = BitBoard::EMPTY;

        destination |= position.down();
        destination |= position.up();
        destination |= position.left();
        destination |= position.right();
        destination |= position.down_left();
        destination |= position.down_right();
        destination |= position.up_left();
        destination |= position.down_right();

        let empty_boards = board_set.get_empty_squares(destination, Some(!self.color));

        for (board_type, square, is_empty) in &empty_boards {
            if *is_empty {
                attacks[*board_type] |= *square;
            }
        }

        // TODO: 판이 움직였어도 캐슬링이 가능한지? 킹 사이드 캐슬링의 결과가 어떻게 되는지?
        if !self.is_moved {
            match self.color {
                Color::White => {
                    // 킹 사이드 캐슬링
                    if let Some(right_piece) = board_set.get_piece(BitBoard::E0 | BitBoard::KL1) {
                        if !right_piece.is_moved {
                            attacks[BoardType::WhiteKing] |= BitBoard::E0;
                        }
                    }

                    // 퀸 사이드 캐슬링
                    if let Some(left_piece) = board_set.get_piece(BitBoard::Z0 | BitBoard::QL1) {
                        let is_between_empty = !board_set.occupied_piece.union()[BoardType::WhiteQueen]
                            .contains(BitBoard::D0);

                        if !left_piece.is_moved && is_between_empty {
                            attacks[BoardType::WhiteQueen] |= BitBoard::A0;
                        }
                    }
                }
                Color::Black => {
                    // 킹 사이드 캐슬링
                    if let Some(right_piece) = board_set.get_piece(BitBoard::E9 | BitBoard::KL6) {
                        if !right_piece.is_moved {
                            attacks[BoardType::BlackKing] |= BitBoard::E9;
                        }
                    }

                    // 퀸 사이드 캐슬링
                    if let Some(left_piece) = board_set.get_piece(BitBoard::Z9 | BitBoard::QL6) {
                        let is_between_empty = !board_set.occupied_piece.union()[BoardType::BlackQueen]
                            .contains(BitBoard::D9);

                        if !left_piece.is_moved && is_between_empty {
                            attacks[BoardType::BlackQueen] |= BitBoard::A9;
                        }
                    }
                }
            }
        }

        attacks
    }
}
