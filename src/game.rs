use crate::{
    bit_board::BitBoard,
    board::{Board, BoardSnapshot},
    chess_move::ChessMove,
    piece::PieceType,
    square::{Color, Rank, Square},
};
use crate::board_type::BoardType;
use crate::square::{File, Level};

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

        if self.board.turn == Color::Black {
            self.board.full_move_number += 1;
        }

        self.board.half_move_clock += 1;

        if let Some(piece_move) = chess_move.as_piece_move() {
            let piece = self
                .board
                .get_piece(BitBoard::from_square(&piece_move.source))
                .unwrap();

            if piece.piece_type == PieceType::Pawn || piece_move.is_capture(&self.board) {
                self.board.half_move_clock = 0;
            }
        }

        let result = chess_move.run(&mut self.board);
        self.board.update();

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

impl Game {
    pub fn new_sandbox( fen : String ) -> Self {

        let mut sandbox = Self {
            board: Board::new(),
            move_stack: Vec::new(),
        };

        let main_parts : Vec<&str> = fen.split('/').take(12).collect();
        let mut init_parts: Vec<[char; 4]> = Vec::new();

        let mut part : [char; 4] = [char::default(); 4];
        let mut part_count : usize = 0;

        for &part_str in main_parts.iter().as_slice() {
            let part_chars : Vec<char> = part_str.chars().collect();

            for &part_char in part_chars.iter().as_slice() {
                if VOID_CHAR_VEC.contains(&part_char) {
                    let void_size = part_char.to_digit(10).unwrap() as usize;
                    part_count += void_size;
                } else {
                    part[part_count] = part_char;
                    part_count += 1;
                }
            }

            init_parts.push(part);
            part = [char::default(); 4];
            part_count = 0;
        }

        let mut level_count : u8 = 1;
        let mut current_bitboard : BitBoard = BitBoard::A1;
        let mut upside_bitboard : BitBoard = current_bitboard.up();

        for init_part in init_parts.iter().as_slice() {

            let bitboard_level : BitBoard = match level_count  {
                1..=4 => BitBoard::WHITE,
                5..=8 => BitBoard::NEUTRAL,
                9..=12 => BitBoard::BLACK,
                _ => BitBoard::BLACK
            };

            for &init_char in init_part.iter() {
                match PIECE_CHAR_VEC.contains(&init_char) {
                    true => {
                        sandbox.board.set_piece(current_bitboard | bitboard_level, Self::check_piece(init_char), Self::check_color(init_char));
                    }
                    _ => ()
                }
                current_bitboard = current_bitboard.right();
            }

            level_count += 1;

            if level_count == 5 {
                current_bitboard = BitBoard::A3;
                upside_bitboard = current_bitboard.up();
            } else if level_count == 9 {
                current_bitboard = BitBoard::A5;
                upside_bitboard = current_bitboard.up();
            } else {
                current_bitboard = upside_bitboard;
                upside_bitboard = current_bitboard.up();
            }
        }

        let sub_parts : Vec<&str> = fen.split('/').skip(12).collect();

        if !sub_parts.is_empty() {
            let mut init_parts: Vec<[char; 4]> = Vec::new();
            let mut init_sub_board : Vec<&str> = Vec::new();
            let mut part_count : usize = 0;

            for &part_str in sub_parts.iter().as_slice() {
                let divide_str : &str = &part_str[..2];
                let sub_board_str : String = part_str[2..].to_string();

                for part_char in sub_board_str.chars() {
                    if VOID_CHAR_VEC.contains(&part_char) {
                        let void_size = part_char.to_digit(10).unwrap() as usize;
                        part_count += void_size;
                    } else {
                        part[part_count] = part_char;
                        part_count += 1;
                    }
                }

                init_sub_board.push(divide_str);
                init_parts.push(part);
                part = [char::default(); 4];
                part_count = 0;
            }

            sandbox.board.board_set[3] = (BoardType::WhiteQueen, Level::QL1);

            let mut level_count : usize = 0;
            let mut current_bitboard : BitBoard;
            let mut upside_bitboard : BitBoard;

            for (count, init_part) in init_parts.iter().enumerate() {
                let mut bitboard_level: BitBoard = BitBoard::QL1;
                if count == level_count {
                    level_count += 1;
                    let board_index = 2 + level_count;
                    bitboard_level = match init_sub_board[level_count - 1] {
                        "q1" | "Q1" => { sandbox.board.board_set[board_index] = (BoardType::WhiteQueen, Level::QL1); BitBoard::QL1 },
                        "q2" | "Q2" => { sandbox.board.board_set[board_index] = (BoardType::WhiteQueen, Level::QL2); BitBoard::QL2 },
                        "q3" | "Q3" => { sandbox.board.board_set[board_index] = (BoardType::Neutral, Level::QL3); BitBoard::QL3 },
                        "q4" | "Q4" => { sandbox.board.board_set[board_index] = (BoardType::Neutral, Level::QL4); BitBoard::QL4 },
                        "q5" | "Q5" => { sandbox.board.board_set[board_index] = (BoardType::BlackQueen, Level::QL5); BitBoard::QL5},
                        "q6" | "Q6" => { sandbox.board.board_set[board_index] = (BoardType::BlackQueen, Level::QL6); BitBoard::QL6 },
                        "k1" | "K1" => { sandbox.board.board_set[board_index] = (BoardType::WhiteKing, Level::KL1); BitBoard::KL1 },
                        "k2" | "K2" => { sandbox.board.board_set[board_index] = (BoardType::WhiteKing, Level::KL2); BitBoard::KL2 },
                        "k3" | "K3" => { sandbox.board.board_set[board_index] = (BoardType::Neutral, Level::KL3); BitBoard::KL3 },
                        "k4" | "K4" => { sandbox.board.board_set[board_index] = (BoardType::Neutral, Level::KL4); BitBoard::KL4 },
                        "k5" | "K5" => { sandbox.board.board_set[board_index] = (BoardType::BlackKing, Level::KL5); BitBoard::KL5 },
                        "k6" | "K6" => { sandbox.board.board_set[board_index] = (BoardType::BlackKing, Level::KL6); BitBoard::KL6 },
                        _ =>  BitBoard::empty()
                    }
                }

                match bitboard_level {
                    BitBoard::QL1 => { current_bitboard = BitBoard::Z0; upside_bitboard = current_bitboard.up();},
                    BitBoard::QL2 | BitBoard::QL5 => { current_bitboard = BitBoard::Z4; upside_bitboard = current_bitboard.up();},
                    BitBoard::QL3 => { current_bitboard = BitBoard::Z2; upside_bitboard = current_bitboard.up();},
                    BitBoard::QL4 => { current_bitboard = BitBoard::Z6; upside_bitboard = current_bitboard.up();},
                    BitBoard::QL6 => { current_bitboard = BitBoard::Z8; upside_bitboard = current_bitboard.up();},

                    BitBoard::KL1 => { current_bitboard = BitBoard::D0; upside_bitboard = current_bitboard.up();},
                    BitBoard::KL2 | BitBoard::KL5 => { current_bitboard = BitBoard::D4; upside_bitboard = current_bitboard.up();},
                    BitBoard::KL3 => { current_bitboard = BitBoard::D2; upside_bitboard = current_bitboard.up();},
                    BitBoard::KL4 => { current_bitboard = BitBoard::D6; upside_bitboard = current_bitboard.up();},
                    BitBoard::KL6 => { current_bitboard = BitBoard::D8; upside_bitboard = current_bitboard.up();},
                    _ => { current_bitboard = BitBoard::Z0; upside_bitboard = current_bitboard.up();},
                }

                for (square_count, &init_char) in init_part.iter().enumerate() {
                    match PIECE_CHAR_VEC.contains(&init_char) {
                        true => {
                            sandbox.board.set_piece(current_bitboard | bitboard_level, Self::check_piece(init_char), Self::check_color(init_char));
                        }
                        _ => ()
                    }
                    match square_count % 2 {
                        1 => { current_bitboard = upside_bitboard }
                        _ => current_bitboard = current_bitboard.right()
                    }
                }
            }
        }

        sandbox.board.update();

        sandbox
    }
    fn check_color(piece_char : char) -> Color {
        match piece_char.is_uppercase() {
            true => Color::White,
            false => Color::Black
        }
    }
    fn check_piece(piece_char : char) -> PieceType {
        match &piece_char {
            'P' | 'p' => PieceType::Pawn,
            'B' | 'b' => PieceType::Bishop,
            'N' | 'n' => PieceType::Knight,
            'R' | 'r' => PieceType::Rook,
            'Q' | 'q' => PieceType::Queen,
            'K' | 'k' => PieceType::King,
            _ => { PieceType::Pawn } // todo : 이게 맞나
        }
    }
    pub fn print_sandbox(&self) {

        let (first_rank_container
            , second_rank_container
            , third_rank_container
            , fourth_rank_container) = self.init_by_rank(BitBoard::BLACK_SET);

        let (first_rank_queen_container
            , second_rank_queen_container) = self.sub_init_by_rank(BitBoard::QL6_SET);
        let (first_rank_king_container
            , second_rank_king_container) = self.sub_init_by_rank(BitBoard::KL6_SET);

        println!("     \x1B[1mBlack");
        self.sub_print_by_rank(second_rank_queen_container, BitBoard::QL6);
        self.print_by_rank(fourth_rank_container, BitBoard::BLACK);
        self.sub_print_by_rank(second_rank_king_container, BitBoard::KL6);
        self.sub_print_by_rank(first_rank_queen_container, BitBoard::QL6);
        self.print_by_rank(third_rank_container, BitBoard::BLACK);
        self.sub_print_by_rank(first_rank_king_container, BitBoard::KL6);

        let (first_rank_queen_container
            , second_rank_queen_container) = self.sub_init_by_rank(BitBoard::QL5_SET);
        let (first_rank_king_container
            , second_rank_king_container) = self.sub_init_by_rank(BitBoard::KL5_SET);

        self.sub_print_by_rank(second_rank_queen_container, BitBoard::QL5);
        self.print_by_rank(second_rank_container, BitBoard::BLACK);
        self.sub_print_by_rank(second_rank_king_container, BitBoard::KL5);
        self.sub_print_by_rank(first_rank_queen_container, BitBoard::QL5);
        self.print_by_rank(first_rank_container, BitBoard::BLACK);
        self.sub_print_by_rank(first_rank_king_container, BitBoard::KL5);

        let (first_rank_container
            , second_rank_container
            , third_rank_container
            , fourth_rank_container) = self.init_by_rank(BitBoard::NEUTRAL_SET);

        let (first_rank_queen_container
            , second_rank_queen_container) = self.sub_init_by_rank(BitBoard::QL4_SET);
        let (first_rank_king_container
            , second_rank_king_container) = self.sub_init_by_rank(BitBoard::KL4_SET);

        println!("    \x1B[1mNeutral");
        self.sub_print_by_rank(second_rank_queen_container, BitBoard::QL4);
        self.print_by_rank(fourth_rank_container, BitBoard::NEUTRAL);
        self.sub_print_by_rank(second_rank_king_container, BitBoard::KL4);
        self.sub_print_by_rank(first_rank_queen_container, BitBoard::QL4);
        self.print_by_rank(third_rank_container, BitBoard::NEUTRAL);
        self.sub_print_by_rank(first_rank_king_container, BitBoard::KL4);

        let (first_rank_queen_container
            , second_rank_queen_container) = self.sub_init_by_rank(BitBoard::QL3_SET);
        let (first_rank_king_container
            , second_rank_king_container) = self.sub_init_by_rank(BitBoard::KL3_SET);

        self.sub_print_by_rank(second_rank_queen_container, BitBoard::QL3);
        self.print_by_rank(second_rank_container, BitBoard::NEUTRAL);
        self.sub_print_by_rank(second_rank_king_container, BitBoard::KL3);
        self.sub_print_by_rank(first_rank_queen_container, BitBoard::QL3);
        self.print_by_rank(first_rank_container, BitBoard::NEUTRAL);
        self.sub_print_by_rank(first_rank_king_container, BitBoard::KL3);

        let (first_rank_container
            , second_rank_container
            , third_rank_container
            , fourth_rank_container) = self.init_by_rank(BitBoard::WHITE_SET);

        let (first_rank_queen_container
            , second_rank_queen_container) = self.sub_init_by_rank(BitBoard::QL2_SET);
        let (first_rank_king_container
            , second_rank_king_container) = self.sub_init_by_rank(BitBoard::KL2_SET);

        println!("     \x1B[1mWhite");
        self.sub_print_by_rank(second_rank_queen_container, BitBoard::QL2);
        self.print_by_rank(fourth_rank_container, BitBoard::WHITE);
        self.sub_print_by_rank(second_rank_king_container, BitBoard::KL2);
        self.sub_print_by_rank(first_rank_queen_container, BitBoard::QL2);
        self.print_by_rank(third_rank_container, BitBoard::WHITE);
        self.sub_print_by_rank(first_rank_king_container, BitBoard::KL2);

        let (first_rank_queen_container
            , second_rank_queen_container) = self.sub_init_by_rank(BitBoard::QL1_SET);
        let (first_rank_king_container
            , second_rank_king_container) = self.sub_init_by_rank(BitBoard::KL1_SET);

        self.sub_print_by_rank(second_rank_queen_container, BitBoard::QL1);
        self.print_by_rank(second_rank_container, BitBoard::WHITE);
        self.sub_print_by_rank(second_rank_king_container, BitBoard::KL1);
        self.sub_print_by_rank(first_rank_queen_container, BitBoard::QL1);
        self.print_by_rank(first_rank_container, BitBoard::WHITE);
        self.sub_print_by_rank(first_rank_king_container, BitBoard::KL1);
    }

    fn init_by_rank (&self, bit_board_set : BitBoard) -> (Vec<BitBoard>, Vec<BitBoard>, Vec<BitBoard>, Vec<BitBoard>) {
        let mut first_rank_container: Vec<BitBoard> = vec![];
        let mut second_rank_container : Vec<BitBoard> = vec![];
        let mut third_rank_container : Vec<BitBoard> = vec![];
        let mut fourth_rank_container : Vec<BitBoard> = vec![];

        for (i, bit_square) in bit_board_set.iter().enumerate() {
            match i % 4 {
                0 => first_rank_container.push(bit_square),
                1 => second_rank_container.push(bit_square),
                2 => third_rank_container.push(bit_square),
                3 => fourth_rank_container.push(bit_square),
                _ => ()
            }
        }

        (first_rank_container, second_rank_container, third_rank_container, fourth_rank_container)
    }

    fn sub_init_by_rank (&self, bit_board_set : BitBoard) -> (Vec<BitBoard>, Vec<BitBoard>) {
        let mut first_rank_container: Vec<BitBoard> = vec![];
        let mut second_rank_container : Vec<BitBoard> = vec![];

        for (i, bit_square) in bit_board_set.iter().enumerate() {
            match i % 2 {
                0 => first_rank_container.push(bit_square),
                1 => second_rank_container.push(bit_square),
                _ => ()
            }
        }

        (first_rank_container, second_rank_container)
    }

    fn print_by_rank (&self, rank_vec : Vec<BitBoard>, bit_board_level : BitBoard ) {
        for &bit_square in rank_vec.iter() {
            match self.board.get_piece(bit_square | bit_board_level) {
                Some(piece) => print!("{} ", piece.get_char()),
                None => print!(". "),
            }
        }
    }

    fn sub_print_by_rank (&self, rank_vec : Vec<BitBoard>, bit_board_level : BitBoard ) {
        if !self.board.board_set.map(|(_, level)| level).contains(&bit_board_level.get_level()) {
            for &bit_square in rank_vec.iter() {
                print!("  ");

                if bit_square.get_file() == File::E {
                    println!();
                }
            }
            return ;
        }

        for &bit_square in rank_vec.iter() {
            match self.board.get_piece(bit_square | bit_board_level) {
                Some(piece) => print!("{} ", piece.get_char()),
                None => print!(". "),
            }

            if bit_square.get_file() == File::E {
                println!();
            }
        }
    }
}
pub const FEN_SLASH : usize = 15;
pub const FEN_SIMPLIFY_SLASH : usize = 11;
pub const PIECE_CHAR_VEC : [char; 12] = [
    'p', 'b', 'n', 'k', 'q', 'r',
    'P', 'B', 'N', 'K', 'Q', 'R'
];
pub const VOID_STR_SUB_LEVEL_VEC : [&str; 12] = [
    "q1", "q2", "q3", "q4", "q5", "q6",
    "k1", "k2", "k3", "k4", "k5", "k6",
];
pub const VOID_CHAR_VEC: [char; 4] = ['1', '2', '3', '4'];

#[cfg(test)]
mod a {
    use crate::chess_move::{BoardMove, PieceMove};
    use super::*;

    #[test]
    fn test_legal() {
        let mut game = Game::new();

        game.get_attack_squares(&Square::new(Rank::One, File::A, Level::White))
            .iter()
            .for_each(|x| println!("{:?}", x));

        let piece_move = PieceMove::new(
            Square::new(Rank::One, File::A, Level::White),
            Square::new(Rank::Nine, File::B, Level::White),
            None,
        );

        /*if game.legal_move(&piece_move) {
            let _ = game.push_move(piece_move);
        }*/

        assert!(game.legal_move(&piece_move), "temp");


        // 움직임
        // err 경우랑 알맞은 경우 에 따른 테스트 별도 필요

        /*/*let _ = game.push_move(BoardMove::new(Level::QL1, Level::QL2, None));

        println!("{}", BitBoard::A2.rank_distance(&BitBoard::B3));*/

        game.print();*/
    }
}