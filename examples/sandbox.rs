extern crate warp_square_engine;

use std::io;
use warp_square_engine::{game::Game, piece_move::PieceMove, square::{File, Level, Rank, Square}};

enum SelectableOption {

}

fn main() {

    show_readme();
    init_selection();
    /*

    / 15
    */
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

fn init_by_user_input() {

    println!("Using Fen method");
    println!("Start at up to down & left to right");
    println!("You dont have to write level Neutral, black, white! only q1~q6, k1~k6");
    println!("White is upper case \"P\", Black is lower case \"p\" \n");
    println!("Like this : nbbn/p2p/4/4/4/4/4/4/4/4/P1P1/NBBN/q122/q622/k1pp/k6rr2");
    println!("If you dont need sub boards, you dont have to write but it will be empty");
    println!("Like this : nbbn/p2p/4/4/4/4/4/4/4/4/P1P1/NBBN");

    println!("Please write init pos by FEN");

    let mut input: String::new();
    let result = std::io::stdin()
        .read_line(&mut input);

    let fen = match result {
        Ok(_) => input.trim(),
        Err(error) => {
            eprintln!("Some Err write exactly");
            std::process::exit(1);
        }
    };
    // 잘못 쓴거 체크해서 루프 돌기
    // 4개일때 3개일때 2개일때 1개일때
}