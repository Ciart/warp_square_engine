use crate::{
    bit_board::BitBoard,
    board::{Board, BoardSnapshot},
    chess_move::ChessMove,
    piece::PieceType,
    square::{Color, Rank, Square},
};

pub struct Game {
    pub board: Board,
    pub move_stack: Vec<(Box<dyn ChessMove>, BoardSnapshot)>,
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

    pub fn legal_move(&self, chess_move: &impl ChessMove) -> bool {
        chess_move.legal(&self.board)
    }

    pub fn push_move(&mut self, chess_move: impl ChessMove + 'static) -> Result<(), &'static str> {
        let snapshot = BoardSnapshot::new(&self.board);

        let result = chess_move.run(&mut self.board);

        self.move_stack.push((Box::new(chess_move), snapshot));
        self.pass_turn();

        result
    }

    pub fn pop_move(&mut self) -> Result<Box<dyn ChessMove>, &'static str> {
        self.pass_turn();
        match self.move_stack.pop() {
            Some((chess_move, snapshot)) => {
                snapshot.restore(&mut self.board);
                Ok(chess_move)
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
