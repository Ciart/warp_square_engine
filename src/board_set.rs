use crate::{
    bit_board::BitBoard,
    bit_board_set::BitBoardSet,
    board::{self, Board},
    board_type::BoardType,
    chess_move::{BoardMove, ChessMove, PieceMove},
    color_mask::ColorMask,
    piece::{Piece, PieceType},
    square::{Color, Level},
};

/// 앙파상 처리를 위한 구조체
#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct EnPassant {
    /// 잡는 폰이 갈 수 있는 위치
    pub position: BitBoard,

    // 앙파상으로 잡히는 현재 폰의 위치
    pub new_square: BitBoard,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct BoardSetSnapshot {
    turn: Color,
    full_move_number: u32,
    half_move_clock: u32,
    pieces: Vec<Piece>,
    captured_pieces: Vec<Piece>,
    boards: [Board; 7],
    occupied_void: BitBoardSet,
    occupied_piece: ColorMask,
    en_passant: Option<EnPassant>,
}

impl BoardSetSnapshot {
    pub fn new(board: &BoardSet) -> Self {
        Self {
            turn: board.turn,
            full_move_number: board.full_move_number,
            half_move_clock: board.half_move_clock,
            pieces: board.pieces.clone(),
            captured_pieces: board.captured_pieces.clone(),
            boards: board.boards.clone(),
            occupied_void: board.occupied_void.clone(),
            occupied_piece: board.occupied_piece.clone(),
            en_passant: board.en_passant.clone(),
        }
    }

    pub fn restore(&self, board: &mut BoardSet) {
        board.turn = self.turn;
        board.full_move_number = self.full_move_number;
        board.half_move_clock = self.half_move_clock;
        board.pieces = self.pieces.clone();
        board.captured_pieces = self.captured_pieces.clone();
        board.boards = self.boards.clone();
        board.occupied_void = self.occupied_void.clone();
        board.occupied_piece = self.occupied_piece.clone();
        board.en_passant = self.en_passant.clone();
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct BoardSet {
    pub turn: Color,
    pub full_move_number: u32,
    pub half_move_clock: u32,
    pub pieces: Vec<Piece>,
    pub captured_pieces: Vec<Piece>,
    pub boards: [Board; 7],
    pub occupied_void: BitBoardSet,
    pub occupied_piece: ColorMask,
    pub en_passant: Option<EnPassant>,
}

impl BoardSet {
    pub fn new() -> Self {
        Self {
            turn: Color::White,
            full_move_number: 1,
            half_move_clock: 0,
            pieces: Vec::new(),
            captured_pieces: Vec::new(),
            boards: [
                Board::new(BoardType::White, Level::White),
                Board::new(BoardType::Neutral, Level::Neutral),
                Board::new(BoardType::Black, Level::Black),
                Board::new(BoardType::WhiteQueen, Level::QL1),
                Board::new(BoardType::WhiteKing, Level::KL1),
                Board::new(BoardType::BlackQueen, Level::QL6),
                Board::new(BoardType::BlackKing, Level::KL6),
            ],
            occupied_void: BitBoardSet::new(),
            occupied_piece: ColorMask::new(),
            en_passant: None,
        }
    }

    pub fn get_board_type(&self, level: Level) -> Option<BoardType> {
        match level {
            Level::White => Some(BoardType::White),
            Level::Neutral => Some(BoardType::Neutral),
            Level::Black => Some(BoardType::Black),
            _ => match self.boards.iter().find(|board| board.level == level) {
                Some(board) => Some(board.board_type),
                None => None,
            },
        }
    }

    pub fn get_board_level(&self, board_type: BoardType) -> Level {
        self.boards[board_type as usize].level
    }

    /// 같은 Rank, File의 모든 Square 상태를 반환합니다.
    /// TODO: 함수 이름 변경, 반환값 정리
    pub fn get_empty_squares(
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
            let level = self.get_board_level(board_type);

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

    // Bridge 전용 함수
    pub fn get_pieces_with_board_type(&self, board_type: BoardType) -> Vec<Piece> {
        self.pieces
            .iter()
            .filter(|piece| self.get_board_type(piece.position.get_level()) == Some(board_type))
            .map(|piece| piece.clone())
            .collect()
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

        self.pieces.push(Piece::new(square, piece, color, false));

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

    pub fn swap_piece(
        &mut self,
        source: BitBoard,
        destination: BitBoard,
    ) -> Result<(), &'static str> {
        let source_piece = match self.remove_piece(source) {
            Some(piece) => piece,
            None => return Err("There is no piece at the source"),
        };

        let destination_piece = match self.remove_piece(destination) {
            Some(piece) => piece,
            None => return Err("There is no piece at the destination"),
        };

        self.pieces.push(Piece::new(
            source,
            destination_piece.piece_type,
            destination_piece.color,
            true,
        ));

        self.pieces.push(Piece::new(
            destination,
            source_piece.piece_type,
            source_piece.color,
            true,
        ));

        Ok(())
    }

    pub fn move_board(
        &mut self,
        source: Level,
        destination: Level,
        promotion: Option<PieceType>,
    ) -> Result<(), &'static str> {
        let board = match self.boards.iter_mut().find(|board| board.level == source) {
            Some(board) => board,
            None => return Err("There is no board at the source"),
        };

        board.level = destination;

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
                piece.is_moved = true;

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
            self.occupied_void[board_type] = !self.get_board_level(board_type).get_area();
        }

        for piece in self.pieces.iter() {
            match self.get_board_type(piece.position.get_level()) {
                Some(board_type) => {
                    self.occupied_piece[piece.color][board_type] |= piece.position.remove_level();
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

        let king_board = self.get_board_type(king.position.get_level()).unwrap();
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

        for piece in self.pieces.iter() {
            for destination in piece.get_attack_squares(self) {
                let piece_move = PieceMove::new(piece.position.into_square(), destination, None);

                if piece_move.legal(self) && !piece_move.is_self_check(self) {
                    return false;
                }
            }
        }

        for board in self
            .boards
            .iter()
            .filter(|board| board.board_type.is_attack())
        {
            let moveable_list = board.level.get_moveable_list();

            for destination in moveable_list {
                let board_move = BoardMove::new(board.level, destination, None);

                if board_move.legal(self) && !board_move.is_self_check(self) {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_stalemate(&self) -> bool {
        self.pieces.iter().all(|piece| {
            if piece.color != self.turn {
                return true;
            } else {
                piece.attacks.iter().all(|attacks| attacks.is_empty())
            }
        })
    }

    pub fn set_en_passant(&mut self, position: BitBoard, new_square: BitBoard) {
        self.en_passant = Some(EnPassant {
            position,
            new_square,
        });
    }

    pub fn remove_en_passant(&mut self) {
        self.en_passant = None;
    }
}
