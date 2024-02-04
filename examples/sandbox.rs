extern crate warp_square_engine;

use std::io;
use warp_square_engine::{piece_move::PieceMove, square::{File, Level, Rank, Square}};
use warp_square_engine::board::{Board, BoardSnapshot};
use warp_square_engine::piece::{Piece, PieceType};
use warp_square_engine::square::Color;

fn main() {
    show_readme();
    init_selection();
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

fn init_selection() {
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
        _ => println!("Choose in selections by number\n"),
    };
}


const FEN_SLASH : usize = 15;
const FEN_SIMPLIFY_SLASH : usize = 11;
fn init_by_user_input() {
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
                    println!("15ok");

                    break;
                }
            },
            FEN_SIMPLIFY_SLASH => {
                let is_valid_string = fen.is_fenric(FEN_SIMPLIFY_SLASH);
                let is_valid_string_count = match fen.chars().count() {
                    19..=55 => true,
                    _ => false,
                };

                if is_valid_string & is_valid_string_count {
                    println!("11ok");

                    break;
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
        let piece_str_vec = vec![
            PieceType::Pawn.get_char(Color::White),
            PieceType::Bishop.get_char(Color::White),
            PieceType::Knight.get_char(Color::White),
            PieceType::King.get_char(Color::White),
            PieceType::Queen.get_char(Color::White),
            PieceType::Rook.get_char(Color::White),

            PieceType::Pawn.get_char(Color::Black),
            PieceType::Bishop.get_char(Color::Black),
            PieceType::Knight.get_char(Color::Black),
            PieceType::King.get_char(Color::Black),
            PieceType::Queen.get_char(Color::Black),
            PieceType::Rook.get_char(Color::Black),
        ];
        let void_char_vec = vec!['1', '2', '3', '4'];

        let void_str_sub_level_vec = vec![
            "q1", "q2", "q3", "q4", "q5", "q6",
            "k1", "k2", "k3", "k4", "k5", "k6",
        ];

        let main_parts : Vec<&str> = self.split('/').take(12).collect();
        let sub_parts : Vec<&str> = match slash_count {
            FEN_SLASH => self.split(('/')).skip(12).collect(),
            _ => vec![]
        };

        for &part in main_parts.iter() {
            if !(1..=4).contains(&part.chars().count()) {
                eprintln!("Err : You write overflow in fen! at ..{}..", part);
                return false;
            }
            if !piece_str_vec.iter().any(|&piece| part.contains(piece) || part.find(piece).is_none()) {
                eprintln!("Err : You dont write correct piece type in fen! at ..{}..", part);
                return false;
            }
            if !void_char_vec.iter().any(|&void_count| part.contains(void_count) || part.find(void_count).is_none()) {
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
            if !void_str_sub_level_vec.iter().any(|&level| part[0..2].contains(level) ){
                eprintln!("Err : You dont write level or write wrong in fen! at ..{}..", part);
                return false;
            }
            if !piece_str_vec.iter().any(|&piece| part[2..].contains(piece) ||
                part[2..].find(piece).is_none()) {
                eprintln!("Err : You dont write correct piece type in fen! at ..{}..", part);
                return false;
            }
            if !void_char_vec[..2].iter().any(|&void_count| part[2..].contains(void_count) ||
                part[2..].find(void_count).is_none()) {
                eprintln!("Err : You dont write correct void in fen! at ..{}..", part);
                return false;
            }
        }

        true
    }
}

struct SandBoxMode {
    pub board: Board,
    pub move_stack: Vec<(PieceMove, BoardSnapshot)>,
}
