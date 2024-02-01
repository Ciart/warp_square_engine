use crate::{bit_board::BitBoard, board::Board, piece::PieceType, square::{Level, Square}};

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
    pub fn new(source: Square, destination: Square, promotion: Option<PieceType>) -> Self {
        Self {
            source,
            destination,
            promotion,
        }
    }
}

impl ChessMove for PieceMove {
    fn run(&self, board: &mut Board) -> Result<(), &'static str> {
        let source = BitBoard::from_square(&self.source);
        let destination = BitBoard::from_square(&self.destination);

        let result = board.move_piece(source, destination);
        board.update();

        result
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

        if piece.attacks[board_type].contains(bit_destination.remove_level()) {
            return true;
        }

        false
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct BoardMove {
    pub source: Level,
    pub destination: Level,
}

impl BoardMove {
    pub fn new(source: Level, destination: Level) -> Self {
        Self {
            source,
            destination,
        }
    }
}

impl ChessMove for BoardMove {
    fn run(&self, board: &mut Board) -> Result<(), &'static str> {
        board.move_board(self.source, self.destination)
    }

    fn legal(&self, board: &Board) -> bool {
        todo!()
    }
}
