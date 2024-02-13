extern crate warp_square_engine;

use std::io;
use pyo3::ffi::newfunc;
use warp_square_engine::{piece_move::PieceMove, square::{File, Level, Rank, Square}};
use warp_square_engine::bit_board::BitBoard;
use warp_square_engine::board::{Board, BoardSnapshot};
use warp_square_engine::game::Game;
use warp_square_engine::piece::{Piece, PieceType};
use warp_square_engine::square::Color;

fn main() {
    show_readme();
    let fen = init_selection();
    let sandbox = Game::new_sandbox(fen);
    sandbox.print();
}
fn show_readme() {
    let summary = " Summary ! ";

    let file_item = vec![File::Z, File::A, File::B, File::C, File::D, File::E];
    let rank_item = vec![Rank::Zero, Rank::One, Rank::Two, Rank::Three, Rank::Four
                         , Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine];

    println!("{summary}");

    println!("File \u{2192} : {:?}", file_item);
    println!("Rank \u{2191} : {:?}", rank_item);
    println!(" ");
}

fn init_selection() -> String {
    let first_selection = String::from("Init board by user input");
    let second_selection = String::from("Init board by selectable options");
    let third_selection = String::from("Exit");

    println!("Let's Start SandBox mode!");
    println!("1. {first_selection}");
    println!("2. {second_selection}");
    println!("3. {third_selection}");
    println!();

    let mut input = String::new();
    let result = io::stdin()
        .read_line(&mut input);

    let selection = match result {
        Ok(_) => input.trim(),
        Err(error) => {
            eprintln!("Choose in selections by number: {}", error);
            std::process::exit(1);
        }
    };

    match selection {
        "1" => init_by_user_input(),
        "2" => std::process::exit(0),
        "3" => std::process::exit(0),
        _ => { println!("Choose in selections by number\n"); std::process::exit(0) }
    }
}


const FEN_SLASH : usize = 15;
const FEN_SIMPLIFY_SLASH : usize = 11;
const PIECE_CHAR_VEC: [char; 12] = [
    'p', 'b', 'n', 'k', 'q', 'r',
    'P', 'B', 'N', 'K', 'Q', 'R'
];
const VOID_STR_SUB_LEVEL_VEC: [&str; 12] = [
    "q1", "q2", "q3", "q4", "q5", "q6",
    "k1", "k2", "k3", "k4", "k5", "k6",
];
const VOID_CHAR_VEC: [char; 4] = ['1', '2', '3', '4'];

fn init_by_user_input() -> String {
    println!("Using Fen method");
    println!("Start at up to down & left to right");
    println!("You dont have to write level Neutral, black, white! only q1~q6, k1~k6");
    println!("White is upper case \"P\", Black is lower case \"p\" \n");
    println!("Like this : nbbn/p2p/4/4/4/4/4/4/4/4/P1P1/NBBN/q122/q622/k1pp/k6rr2");
    println!("If you dont need sub boards, you dont have to write but it will be empty");
    println!("Like this : nbbn/p2p/4/4/4/4/4/4/4/4/P1P1/NBBN");
    println!("Please write init pos by FEN : ");

    loop {
        let mut input = String::new();
        let result = io::stdin()
            .read_line(&mut input);

        let fen = match result {
            Ok(_) => input.trim(),
            Err(_) => {
                eprintln!("Some Err write exactly");
                std::process::exit(1);
            }
        };

        let slash_count = fen.matches('/').count();
        match slash_count {
            FEN_SLASH => {
                let is_valid_string = fen.is_fenric(FEN_SLASH);
                let is_valid_string_count = match fen.chars().count() {
                    43..=87 => true,
                    _ => false,
                };
                if is_valid_string & is_valid_string_count {
                    println!("Init all boards by user order.");
                    return fen.to_string();
                }
            },
            FEN_SIMPLIFY_SLASH => {
                let is_valid_string = fen.is_fenric(FEN_SIMPLIFY_SLASH);
                let is_valid_string_count = match fen.chars().count() {
                    19..=55 => true,
                    _ => false,
                };

                if is_valid_string & is_valid_string_count {
                    println!("Init main boards by user order.");
                    return fen.to_string();
                }

            }
            _ => { println!("You write wrong!\n"); continue },
        }
    }
}

pub trait CheckFen {
    fn is_fenric(&self, slash_count : usize ) -> bool;
}

impl CheckFen for &str {
    fn is_fenric(&self, slash_count : usize ) -> bool {

        let main_parts : Vec<&str> = self.split('/').take(12).collect();
        let sub_parts : Vec<&str> = match slash_count {
            FEN_SLASH => self.split('/').skip(12).collect(),
            _ => vec![]
        };

        for &part in main_parts.iter() {
            if !(1..=4).contains(&part.chars().count()) {
                eprintln!("Err : You write overflow in fen! at ..{}..", part);
                return false;
            }
            if !PIECE_CHAR_VEC.iter().any(|&piece| part.contains(piece) || part.find(piece).is_none()) {
                eprintln!("Err : You dont write correct piece type in fen! at ..{}..", part);
                return false;
            }
            if !VOID_CHAR_VEC.iter().any(|&void_count| part.contains(void_count) || part.find(void_count).is_none()) {
                eprintln!("Err : You dont write correct void in fen! at ..{}..", part);
                return false;
            }
        }

        if sub_parts.is_empty() { return true; }

        for &part in sub_parts.iter() {
            if !(4..=6).contains(&part.chars().count()) {
                println!("?? {}", &part.chars().count());
                eprintln!("Err : You write overflow in fen! at ..{}..", part);
                return false;
            }
            if !VOID_STR_SUB_LEVEL_VEC.iter().any(|&level| part[0..2].contains(level) ){
                eprintln!("Err : You dont write level or write wrong in fen! at ..{}..", part);
                return false;
            }
            if !PIECE_CHAR_VEC.iter().any(|&piece| part[2..].contains(piece) ||
                part[2..].find(piece).is_none()) {
                eprintln!("Err : You dont write correct piece type in fen! at ..{}..", part);
                return false;
            }
            if !VOID_CHAR_VEC[..2].iter().any(|&void_count| part[2..].contains(void_count) ||
                part[2..].find(void_count).is_none()) {
                eprintln!("Err : You dont write correct void in fen! at ..{}..", part);
                return false;
            }
        }

        true
    }
}

pub trait SandBoxMode {
    fn new_sandbox( fen : String ) -> Self;
    fn check_color( piece_char : char ) -> Color;
    fn check_piece( piece_char : char ) -> PieceType;
}

impl SandBoxMode for Game {
    fn new_sandbox( fen : String ) -> Self {

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
                println!("cur : {:?}, next line : {:?}", current_bitboard, upside_bitboard);
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

        if fen.split('/').skip(12)

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
}