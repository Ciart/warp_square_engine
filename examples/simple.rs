extern crate warp_square_engine;

use warp_square_engine::{game::Game, piece_move::PieceMove, square::{File, Level, Rank, Square}};

fn main() {
    let mut game = Game::new();

    game.get_attack_squares(&Square::new(Rank::One, File::A, Level::White)).iter().for_each(|x| println!("{:?}", x));

    let piece_move = PieceMove::new(Square::new(Rank::One, File::A, Level::White), Square::new(Rank::Three, File::B, Level::White), None);

    if game.legal_move(&piece_move) {
        let _ = game.push_move(piece_move);
        println!("Success")
    }

    game.print();
}
