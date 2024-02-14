use crate::{
    bit_board::BitBoard,
    bit_board_set::BitBoardSet,
    board_type::BoardType,
    color_mask::ColorMask,
    piece::{Piece, PieceType},
    square::{Color, Level},
};

pub struct BoardSnapshot {
    turn: Color,
    pieces: Vec<Piece>,
    captured_pieces: Vec<Piece>,
    board_set: [(BoardType, Level); 7],
    occupied_void: BitBoardSet,
    occupied_piece: ColorMask,
    moved_pawn_two_square: Option<BitBoard>,
}

impl BoardSnapshot {
    pub fn new(board: &Board) -> Self {
        Self {
            turn: board.turn,
            pieces: board.pieces.clone(),
            captured_pieces: board.captured_pieces.clone(),
            board_set: board.board_set.clone(),
            occupied_void: board.occupied_void.clone(),
            occupied_piece: board.occupied_piece.clone(),
            moved_pawn_two_square: board.moved_pawn_two_square.clone(),
        }
    }

    pub fn restore(&self, board: &mut Board) {
        board.turn = self.turn;
        board.pieces = self.pieces.clone();
        board.captured_pieces = self.captured_pieces.clone();
        board.board_set = self.board_set.clone();
        board.occupied_void = self.occupied_void.clone();
        board.occupied_piece = self.occupied_piece.clone();
        board.moved_pawn_two_square = self.moved_pawn_two_square.clone();
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct Board {
    pub turn: Color,
    pub pieces: Vec<Piece>,
    pub captured_pieces: Vec<Piece>,
    pub board_set: [(BoardType, Level); 7],
    pub occupied_void: BitBoardSet,
    pub occupied_piece: ColorMask,
    pub moved_pawn_two_square: Option<BitBoard>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            turn: Color::White,
            pieces: Vec::new(),
            captured_pieces: Vec::new(),
            board_set: [
                (BoardType::White, Level::White),
                (BoardType::Neutral, Level::Neutral),
                (BoardType::Black, Level::Black),
                (BoardType::WhiteQueen, Level::QL1),
                (BoardType::WhiteKing, Level::KL1),
                (BoardType::BlackQueen, Level::QL6),
                (BoardType::BlackKing, Level::KL6),
            ],
            occupied_void: BitBoardSet::new(),
            occupied_piece: ColorMask::new(),
            moved_pawn_two_square: None,
        }
    }

    pub fn convert_board_type(&self, level: Level) -> Option<BoardType> {
        match level {
            Level::White => Some(BoardType::White),
            Level::Neutral => Some(BoardType::Neutral),
            Level::Black => Some(BoardType::Black),
            _ => match self.board_set.iter().find(|(_, l)| *l == level) {
                Some((board_type, _)) => Some(*board_type),
                None => None,
            },
        }
    }

    pub fn convert_level(&self, board_type: BoardType) -> Level {
        self.board_set[board_type as usize].1
    }

    /// 같은 Rank, File의 모든 Square 상태를 반환합니다.
    /// TODO: 함수 이름 변경, 반환값 정리
    pub fn get_empty_board(
        &self,
        squares: BitBoard,
        ignore_color: Option<Color>,
    ) -> Vec<(BoardType, BitBoard, bool)> {
        let mut result = Vec::new();

        let squares = squares.remove_level();
        let occupied = match ignore_color {
            Some(color) => self.occupied_piece[!color].clone(),
            None => self.occupied_piece.union(),
        };

        for board_type in BoardType::iter() {
            let level = self.convert_level(board_type);

            for square in squares.iter() {
                let is_void = !level.get_area().contains(square);

                if is_void {
                    continue;
                }

                if occupied[board_type].contains(square) {
                    result.push((board_type, square, false));
                } else {
                    result.push((board_type, square, true));
                }
            }
        }

        result
    }

    pub fn get_piece(&self, square: BitBoard) -> Option<&Piece> {
        self.pieces.iter().find(|piece| piece.position == square)
    }

    pub fn remove_piece(&mut self, square: BitBoard) -> Option<Piece> {
        match self
            .pieces
            .iter()
            .position(|piece| piece.position == square)
        {
            Some(index) => Some(self.pieces.remove(index)),
            None => None,
        }
    }

    pub fn set_piece(&mut self, square: BitBoard, piece: PieceType, color: Color) -> Option<Piece> {
        let old_piece = self.remove_piece(square);

        self.pieces.push(Piece::new(square, piece, color));

        old_piece
    }

    pub fn move_piece(
        &mut self,
        source: BitBoard,
        destination: BitBoard,
        promotion: Option<PieceType>,
    ) -> Result<(), &'static str> {
        let mut piece = match self.remove_piece(source) {
            Some(piece) => piece,
            None => return Err("There is no piece at the source"),
        };

        let _captured_piece = match self.remove_piece(destination) {
            Some(piece) => self.captured_pieces.push(piece),
            None => (),
        };

        piece.position = destination;
        piece.is_moved = true;

        if piece.is_promotion_position() {
            piece.piece_type = promotion.unwrap_or(PieceType::Queen);
        }

        self.pieces.push(piece);

        Ok(())
    }

    pub fn move_board(
        &mut self,
        source: Level,
        destination: Level,
        promotion: Option<PieceType>,
    ) -> Result<(), &'static str> {
        let board = match self
            .board_set
            .iter_mut()
            .find(|(_, level)| *level == source)
        {
            Some(board) => board,
            None => return Err("There is no board at the source"),
        };

        board.1 = destination;

        for piece in self.pieces.iter_mut() {
            if piece.position.get_level() == source {
                let position = piece.position.remove_level();

                let source_area = source.get_area();
                let destination_area = destination.get_area();

                let shift = (destination_area.bits().trailing_zeros() as i32)
                    - (source_area.bits().trailing_zeros() as i32);

                let new_position = match shift.is_positive() {
                    true => BitBoard::from_bits_retain(position.bits() << shift),
                    false => BitBoard::from_bits_retain(position.bits() >> shift.abs()),
                };

                piece.position = new_position | destination.into_bit_board();

                if piece.is_promotion_position() {
                    piece.piece_type = promotion.unwrap_or(PieceType::Queen);
                }
            }
        }

        Ok(())
    }

    pub fn validate_square(&self, square: BitBoard) -> bool {
        let level = BitBoard::into_square(&square).level;
        let square = square.remove_level();

        level.get_area().contains(square)
    }

    pub fn update_occupied(&mut self) {
        self.occupied_void = BitBoardSet::new();
        self.occupied_piece = ColorMask::new();

        for board_type in BoardType::iter() {
            self.occupied_void[board_type] = !self.convert_level(board_type).get_area();
        }

        for piece in self.pieces.iter() {
            match self.convert_board_type(piece.position.get_level()) {
                Some(board_type) => {
                    self.occupied_piece[piece.color][board_type] |= piece.position;
                }
                None => (),
            }
        }
    }

    pub fn update(&mut self) {
        self.update_occupied();

        let mut pieces = self.pieces.clone();

        for piece in pieces.iter_mut() {
            piece.update_attacks(&self);
        }

        self.pieces = pieces;
    }

    pub fn find_king(&self, color: Color) -> Option<&Piece> {
        self.pieces
            .iter()
            .find(|piece| piece.piece_type == PieceType::King && piece.color == color)
    }

    pub fn is_check(&self) -> bool {
        let king = match self.find_king(self.turn) {
            Some(piece) => piece,
            None => return false,
        };

        let king_board = self.convert_board_type(king.position.get_level()).unwrap();
        let king_position = king.position.remove_level();

        for piece in &self.pieces {
            if piece.attacks[king_board].contains(king_position) {
                return true;
            }
        }

        false
    }

    pub fn is_checkmate(&self) -> bool {
        if !self.is_check() {
            return false;
        }

        let mut pieces = self.pieces.clone();

        for piece in pieces.iter_mut() {
            if piece.color != self.turn {
                continue;
            }

            let piece_board = self.convert_board_type(piece.position.get_level()).unwrap();

            for attack in piece.attacks[piece_board].iter() {
                let mut board = (*self).clone();

                board.move_piece(piece.position, attack, None).unwrap();
                board.update();

                if !board.is_check() {
                    return false;
                }
            }
        }

        true
    }
}
