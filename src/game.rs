use crate::{
    bit_board::BitBoard,
    board::{Board, BoardSnapshot},
    piece::PieceType,
    piece_move::PieceMove,
    square::{Color, Rank, Square},
};

pub struct Game {
    pub board: Board,
    pub move_stack: Vec<(PieceMove, BoardSnapshot)>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: Board::new(),
            move_stack: Vec::new(),
        };

        game.board
            .set_piece(BitBoard::Z0 | BitBoard::QL1, PieceType::Rook, Color::White);
        game.board
            .set_piece(BitBoard::A0 | BitBoard::QL1, PieceType::Queen, Color::White);
        game.board
            .set_piece(BitBoard::Z1 | BitBoard::QL1, PieceType::Pawn, Color::White);
        game.board
            .set_piece(BitBoard::A1 | BitBoard::QL1, PieceType::Pawn, Color::White);
        game.board.set_piece(
            BitBoard::A1 | BitBoard::WHITE,
            PieceType::Knight,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::B1 | BitBoard::WHITE,
            PieceType::Bishop,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::C1 | BitBoard::WHITE,
            PieceType::Bishop,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::D1 | BitBoard::WHITE,
            PieceType::Knight,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::A2 | BitBoard::WHITE,
            PieceType::Pawn,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::B2 | BitBoard::WHITE,
            PieceType::Pawn,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::C2 | BitBoard::WHITE,
            PieceType::Pawn,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::D2 | BitBoard::WHITE,
            PieceType::Pawn,
            Color::White,
        );
        game.board
            .set_piece(BitBoard::D0 | BitBoard::KL1, PieceType::King, Color::White);
        game.board
            .set_piece(BitBoard::E0 | BitBoard::KL1, PieceType::Rook, Color::White);
        game.board
            .set_piece(BitBoard::D1 | BitBoard::KL1, PieceType::Pawn, Color::White);
        game.board
            .set_piece(BitBoard::E1 | BitBoard::KL1, PieceType::Pawn, Color::White);

        game.board
            .set_piece(BitBoard::Z8 | BitBoard::QL6, PieceType::Pawn, Color::Black);
        game.board
            .set_piece(BitBoard::A8 | BitBoard::QL6, PieceType::Pawn, Color::Black);
        game.board
            .set_piece(BitBoard::Z9 | BitBoard::QL6, PieceType::Rook, Color::Black);
        game.board
            .set_piece(BitBoard::A9 | BitBoard::QL6, PieceType::Queen, Color::Black);
        game.board.set_piece(
            BitBoard::A7 | BitBoard::BLACK,
            PieceType::Pawn,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::B7 | BitBoard::BLACK,
            PieceType::Pawn,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::C7 | BitBoard::BLACK,
            PieceType::Pawn,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::D7 | BitBoard::BLACK,
            PieceType::Pawn,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::A8 | BitBoard::BLACK,
            PieceType::Knight,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::B8 | BitBoard::BLACK,
            PieceType::Bishop,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::C8 | BitBoard::BLACK,
            PieceType::Bishop,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::D8 | BitBoard::BLACK,
            PieceType::Knight,
            Color::Black,
        );
        game.board
            .set_piece(BitBoard::D8 | BitBoard::KL6, PieceType::Pawn, Color::Black);
        game.board
            .set_piece(BitBoard::E8 | BitBoard::KL6, PieceType::Pawn, Color::Black);
        game.board
            .set_piece(BitBoard::D9 | BitBoard::KL6, PieceType::King, Color::Black);
        game.board
            .set_piece(BitBoard::E9 | BitBoard::KL6, PieceType::Rook, Color::Black);

        game.board.update();

        game
    }

    fn pass_turn(&mut self) {
        let _ = self.board.turn != self.board.turn;
    }

    pub fn get_attack_squares(&self, square: &Square) -> Vec<Square> {
        let piece = match self.board.get_piece(BitBoard::from_square(square)) {
            Some(piece) => piece,
            None => return Vec::new(),
        };

        piece.get_attack_squares(&self.board)
    }

    pub fn legal_move(&self, piece_move: &PieceMove) -> bool {
        let bit_source = BitBoard::from_square(&piece_move.source);
        let bit_destination = BitBoard::from_square(&piece_move.destination);

        let piece = match self.board.get_piece(bit_source) {
            Some(piece) => piece,
            None => return false,
        };

        if piece.color != self.board.turn {
            return false;
        }

        let board_type = match self.board.convert_board_type(piece_move.destination.level) {
            Some(board_type) => board_type,
            None => return false,
        };

        if piece.attacks[board_type].contains(bit_destination.remove_level()) {
            return true;
        }

        false
    }

    pub fn push_move(&mut self, piece_move: PieceMove) -> Result<(), &'static str> {
        let snapshot = BoardSnapshot::new(&self.board);

        let source = BitBoard::from_square(&piece_move.source);
        let destination = BitBoard::from_square(&piece_move.destination);

        let result = self.board.move_piece(source, destination);
        self.board.update();

        self.move_stack.push((piece_move, snapshot));
        self.pass_turn();

        result
    }

    pub fn pop_move(&mut self) -> Result<PieceMove, &'static str> {
        self.pass_turn();
        match self.move_stack.pop() {
            Some((piece_move, snapshot)) => {
                snapshot.restore(&mut self.board);
                Ok(piece_move)
            }
            None => Err("Nothing to pop"),
        }
    }

    pub fn print(&self) {
        println!("White Board: ");
        for bit_square in BitBoard::WHITE_SET.iter() {
            match self.board.get_piece(bit_square | BitBoard::WHITE) {
                Some(piece) => print!("{} ", piece.get_char()),
                None => print!(". "),
            }

            if bit_square.get_rank() == Rank::Four {
                println!();
            }
        }

        println!("Neutral Board: ");
        for bit_square in BitBoard::NEUTRAL_SET.iter() {
            match self.board.get_piece(bit_square | BitBoard::NEUTRAL) {
                Some(piece) => print!("{} ", piece.get_char()),
                None => print!(". "),
            }

            if bit_square.get_rank() == Rank::Six {
                println!();
            }
        }

        println!("Black Board: ");
        for bit_square in BitBoard::BLACK_SET.iter() {
            match self.board.get_piece(bit_square | BitBoard::BLACK) {
                Some(piece) => print!("{} ", piece.get_char()),
                None => print!(". "),
            }

            if bit_square.get_rank() == Rank::Eight {
                println!();
            }
        }
    }
}
