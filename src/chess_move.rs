use crate::{
    bit_board::BitBoard,
    board_set::BoardSet,
    piece::PieceType,
    square::{Color, Level, Rank, Square},
};

pub trait ChessMove {
    /// 이동을 실행해 보드에 적용합니다.
    /// 프로모션을 제외한 이동 특수 로직는 이곳에서 구현합니다.
    /// Board::move_piece와 Board::move_board는 최대한 단순하게 유지합니다.
    /// * `board_set` - 이동을 적용할 보드입니다.
    fn run(&self, board_set: &mut BoardSet) -> Result<(), &'static str>;

    fn legal(&self, board_set: &BoardSet) -> bool;

    fn is_self_check(&self, board_set: &BoardSet) -> bool {
        let mut board_set = board_set.clone();

        let _ = self.run(&mut board_set);

        board_set.is_check()
    }

    fn as_piece_move(&self) -> Option<&PieceMove> {
        None
    }

    fn as_board_move(&self) -> Option<&BoardMove> {
        None
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct PieceMove {
    pub source: Square,
    pub destination: Square,
    pub promotion: Option<PieceType>,
}

impl PieceMove {
    /// * `promotion` - 프로모션이 가능할 때 해당 PieceType으로 변경합니다. None이면 퀸으로 변경합니다.
    pub fn new(source: Square, destination: Square, promotion: Option<PieceType>) -> Self {
        Self {
            source,
            destination,
            promotion,
        }
    }

    pub fn is_promotion(&self, board_set: &BoardSet) -> bool {
        let piece = match board_set.get_piece(BitBoard::from_square(&self.source)) {
            Some(piece) => piece,
            None => return false,
        };

        if piece.piece_type != PieceType::Pawn {
            return false;
        }

        let end_rank = match piece.color {
            Color::White => {
                if self.destination.level.is_main() {
                    Rank::Eight
                } else {
                    Rank::Nine
                }
            }
            Color::Black => {
                if self.destination.level.is_main() {
                    Rank::One
                } else {
                    Rank::Zero
                }
            }
        };

        self.destination.rank == end_rank
    }

    pub fn is_en_passant(&self, board_set: &BoardSet) -> bool {
        // 파일이 같다면 앙파상일 수 없음
        if self.source.file == self.destination.file {
            return false;
        }

        let source = BitBoard::from_square(&self.source);
        let destination = BitBoard::from_square(&self.destination);

        let piece = match board_set.get_piece(source) {
            Some(piece) => piece,
            None => return false,
        };

        if piece.piece_type != PieceType::Pawn {
            return false;
        }

        if let Some(en_passant) = &board_set.en_passant {
            if destination == en_passant.position {
                return true;
            }
        }

        false
    }

    pub fn is_king_side_castling(&self, board_set: &BoardSet) -> bool {
        let source = BitBoard::from_square(&self.source);
        let destination = BitBoard::from_square(&self.destination);

        if destination != BitBoard::E0 | BitBoard::KL1
            && destination != BitBoard::E9 | BitBoard::KL6
        {
            return false;
        }

        let source_piece = match board_set.get_piece(source) {
            Some(piece) => piece,
            None => return false,
        };

        let destination_piece = match board_set.get_piece(destination) {
            Some(piece) => piece,
            None => return false,
        };

        if source_piece.piece_type != PieceType::King || source_piece.is_moved {
            return false;
        }

        if source_piece.color != destination_piece.color || destination_piece.is_moved {
            return false;
        }

        true
    }

    pub fn is_queen_side_castling(&self, board_set: &BoardSet) -> bool {
        let source = BitBoard::from_square(&self.source);
        let destination = BitBoard::from_square(&self.destination);

        if destination != BitBoard::A0 | BitBoard::QL1
            && destination != BitBoard::A9 | BitBoard::QL6
        {
            return false;
        }

        let source_piece = match board_set.get_piece(source) {
            Some(piece) => piece,
            None => return false,
        };

        let rook_square = match source_piece.color {
            Color::White => BitBoard::Z0 | BitBoard::QL1,
            Color::Black => BitBoard::Z9 | BitBoard::QL6,
        };

        let destination_piece = match board_set.get_piece(rook_square) {
            Some(piece) => piece,
            None => return false,
        };

        if source_piece.piece_type != PieceType::King || source_piece.is_moved {
            return false;
        }

        if source_piece.color != destination_piece.color || destination_piece.is_moved {
            return false;
        }

        true
    }

    pub fn is_castling(&self, board_set: &BoardSet) -> bool {
        self.is_king_side_castling(board_set) || self.is_queen_side_castling(board_set)
    }

    pub fn is_capture(&self, board_set: &BoardSet) -> bool {
        let destination = BitBoard::from_square(&self.destination);

        if let Some(piece) = board_set.get_piece(destination) {
            piece.color != board_set.turn
        } else {
            false
        }
    }

    pub fn get_source_piece(&self, board_set: &BoardSet) -> Option<PieceType> {
        let source = BitBoard::from_square(&self.source);

        match board_set.get_piece(source) {
            Some(piece) => Some(piece.piece_type),
            None => None,
        }
    }
}

impl ChessMove for PieceMove {
    fn run(&self, board_set: &mut BoardSet) -> Result<(), &'static str> {
        let source = BitBoard::from_square(&self.source);
        let destination = BitBoard::from_square(&self.destination);

        if self.is_en_passant(board_set) {
            board_set.remove_piece(board_set.en_passant.as_ref().unwrap().new_square);
        }

        // 앙파상 확인을 위해 변수 업데이트
        if let Some(piece) = board_set.get_piece(source) {
            if piece.piece_type == PieceType::Pawn
                && source.rank_distance(&destination) == 2
                && self.destination.level.is_main()
            {
                board_set.set_en_passant(
                    destination.backward(piece.color) | self.destination.level.into_bit_board(),
                    destination,
                )
            } else {
                board_set.remove_en_passant();
            }
        } else {
            board_set.remove_en_passant();
        }

        if self.is_king_side_castling(board_set) {
            match board_set.turn {
                Color::White => {
                    board_set.swap_piece(BitBoard::D0 | BitBoard::KL1, BitBoard::E0 | BitBoard::KL1)
                }
                Color::Black => {
                    board_set.swap_piece(BitBoard::D9 | BitBoard::KL6, BitBoard::E9 | BitBoard::KL6)
                }
            }
        } else if self.is_queen_side_castling(board_set) {
            match board_set.turn {
                Color::White => board_set
                    .swap_piece(BitBoard::D0 | BitBoard::KL1, BitBoard::Z0 | BitBoard::QL1)
                    .and_then(|_| {
                        board_set.move_piece(
                            BitBoard::Z0 | BitBoard::QL1,
                            BitBoard::A0 | BitBoard::QL1,
                            None,
                        )
                    }),
                Color::Black => board_set
                    .swap_piece(BitBoard::D9 | BitBoard::KL6, BitBoard::Z9 | BitBoard::QL6)
                    .and_then(|_| {
                        board_set.move_piece(
                            BitBoard::Z9 | BitBoard::QL6,
                            BitBoard::A9 | BitBoard::QL6,
                            None,
                        )
                    }),
            }
        } else {
            board_set.move_piece(source, destination, self.promotion)
        }
    }

    fn legal(&self, board_set: &BoardSet) -> bool {
        let bit_source = BitBoard::from_square(&self.source);
        let bit_destination = BitBoard::from_square(&self.destination);

        let piece = match board_set.get_piece(bit_source) {
            Some(piece) => piece,
            None => return false,
        };

        if piece.color != board_set.turn {
            return false;
        }

        let board_type = match board_set.get_board_type(self.destination.level) {
            Some(board_type) => board_type,
            None => return false,
        };

        if !piece.attacks[board_type].contains(bit_destination.remove_level()) {
            return false;
        }

        if self.is_self_check(board_set) {
            return false;
        }

        true
    }

    fn as_piece_move(&self) -> Option<&PieceMove> {
        Some(self)
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct BoardMove {
    pub source: Level,
    pub destination: Level,
    pub promotion: Option<PieceType>,
}

impl BoardMove {
    pub fn new(source: Level, destination: Level, promotion: Option<PieceType>) -> Self {
        Self {
            source,
            destination,
            promotion,
        }
    }
}

impl ChessMove for BoardMove {
    fn run(&self, board_set: &mut BoardSet) -> Result<(), &'static str> {
        board_set.remove_en_passant();
        board_set.move_board(self.source, self.destination, self.promotion)
    }

    fn legal(&self, board_set: &BoardSet) -> bool {
        // 메인 보드는 이동 불가
        if self.source.is_main() || self.destination.is_main() {
            return false;
        }

        // 이동할 보드가 없는 경우
        if board_set.get_board_type(self.source).is_none() {
            return false;
        }

        // 도착지에 이미 보드가 있는 경우
        if board_set.get_board_type(self.destination).is_some() {
            return false;
        }

        // 이동할 수 있는 레벨인지 확인
        if !self.source.get_moveable_list().contains(&self.destination) {
            return false;
        }

        // 기물 1개 이하 존재해야만 움직일 수 있으며 기물의 색깔과 턴이 일치해야 한다.
        // 기물은 폰이어야 한다. 폰이 아닌 경우에는 움직일 수 없다.
        // 빈 어택보드는 누구나 움직일 수 있다.
        let mut source_pieces = board_set
            .pieces
            .iter()
            .filter(|piece| piece.position.get_level() == self.source);

        let is_can_move = match source_pieces.clone().count() {
            0 => true,
            1 => {
                let piece = source_pieces.next().unwrap();
                
                piece.color == board_set.turn && piece.piece_type == PieceType::Pawn
            },
            _ => false,
        };

        if self.is_self_check(board_set) {
            return false;
        }

        is_can_move
    }

    fn as_board_move(&self) -> Option<&BoardMove> {
        Some(self)
    }
}
