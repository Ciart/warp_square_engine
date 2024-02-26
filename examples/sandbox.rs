extern crate warp_square_engine;

use std::io;
use warp_square_engine::square::{File, Rank};
use warp_square_engine::game::{Game, PIECE_CHAR_VEC, VOID_CHAR_VEC, VOID_STR_SUB_LEVEL_VEC, FEN_SIMPLIFY_SLASH, FEN_SLASH};


fn main() {
    show_readme();
    let fen = init_selection();
    let sandbox = Game::new_sandbox(fen);
    sandbox.print_sandbox();
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

fn init_by_user_input() -> String {
    println!("Using Fen method");
    println!("Start at up to down & left to right");
    println!("You dont have to write level Neutral, black, white! only q1~q6, k1~k6");
    println!("White is upper case \"P\", Black is lower case \"p\" \n");
    println!("Like this : nbbn/p2p/4/4/4/4/4/4/4/4/P1P1/NBBN/q5q1q1/q6qqqq/k1pp2/k6qqqq");
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

trait CheckFen {
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

