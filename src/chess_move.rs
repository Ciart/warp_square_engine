use std::collections::HashMap;

use crate::{
    bit_board::BitBoard,
    board::Board,
    piece::PieceType,
    square::{Color, Level, Rank, Square},
};

pub trait ChessMove {
    fn run(&self, board: &mut Board) -> Result<(), &'static str>;

    fn legal(&self, board: &Board) -> bool;
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

    // TODO: 캡쳐 시에도 프로모션이 가능한지 조사해야 함F
    pub fn is_promotion(&self, board: &Board) -> bool {
        let piece = match board.get_piece(BitBoard::from_square(&self.source)) {
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
}

impl ChessMove for PieceMove {
    fn run(&self, board: &mut Board) -> Result<(), &'static str> {
        let source = BitBoard::from_square(&self.source);
        let destination = BitBoard::from_square(&self.destination);

        // 앙파상 확인을 위해 변수 업데이트
        if let Some(piece) = board.get_piece(source) {
            if piece.piece_type == PieceType::Pawn && source.rank_distance(&destination) == 2 {
                board.moved_pawn_two_square = Some(destination);
            } else {
                board.moved_pawn_two_square = None;
            }
        } else {
            board.moved_pawn_two_square = None;
        }

        board.move_piece(source, destination, self.promotion)
    }

    fn legal(&self, board: &Board) -> bool {
        let bit_source = BitBoard::from_square(&self.source);
        let bit_destination = BitBoard::from_square(&self.destination);

        let piece = match board.get_piece(bit_source) {
            Some(piece) => piece,
            None => return false,
        };

        if piece.color != board.turn {
            return false;
        }

        let board_type = match board.convert_board_type(self.destination.level) {
            Some(board_type) => board_type,
            None => return false,
        };

        if !piece.attacks[board_type].contains(bit_destination.remove_level()) {
            return false;
        }

        true
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
    fn run(&self, board: &mut Board) -> Result<(), &'static str> {
        board.moved_pawn_two_square = None;
        board.move_board(self.source, self.destination, self.promotion)
    }

    fn legal(&self, board: &Board) -> bool {
        // 메인 보드는 이동 불가
        if self.source.is_main() || self.destination.is_main() {
            return false;
        }

        // 이동할 보드가 없는 경우
        if board.convert_board_type(self.source).is_none() {
            return false;
        }

        // 도착지에 이미 보드가 있는 경우
        if board.convert_board_type(self.destination).is_some() {
            return false;
        }

        // TODO: 함수로 분리해야 함
        // TODO: 이웃한게 맞는지 다시 검증해야 함
        let level_map: HashMap<Level, Vec<Level>> = HashMap::from([
            (Level::QL1, vec![Level::QL2, Level::KL1]),
            (Level::QL2, vec![Level::QL1, Level::QL3, Level::KL2]),
            (Level::QL3, vec![Level::QL2, Level::QL4, Level::KL3]),
            (Level::QL4, vec![Level::QL3, Level::QL5, Level::KL4]),
            (Level::QL5, vec![Level::QL4, Level::QL6, Level::KL5]),
            (Level::QL6, vec![Level::QL5, Level::KL6]),
            (Level::KL1, vec![Level::QL1, Level::KL2]),
            (Level::KL2, vec![Level::QL2, Level::KL1, Level::KL3]),
            (Level::KL3, vec![Level::QL3, Level::KL2, Level::KL4]),
            (Level::KL4, vec![Level::QL4, Level::KL3, Level::KL5]),
            (Level::KL5, vec![Level::QL5, Level::KL4, Level::KL6]),
            (Level::KL6, vec![Level::QL6, Level::KL5]),
        ]);

        // 이동할 수 있는 레벨인지 확인
        if !level_map
            .get(&self.source)
            .unwrap()
            .contains(&self.destination)
        {
            return false;
        }

        // 기물 1개 이하 존재해야만 움직일 수 있으며 기물의 색깔과 턴이 일치해야 한다.
        // 빈 어택보드는 누구나 움직일 수 있다.
        let mut source_pieces = board
            .pieces
            .iter()
            .filter(|piece| piece.position.get_level() == self.source);

        match source_pieces.clone().count() {
            0 => true,
            1 => source_pieces.next().unwrap().color == board.turn,
            _ => false,
        }
    }
}
