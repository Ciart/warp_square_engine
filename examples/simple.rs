extern crate warp_square_engine;

use warp_square_engine::{chess_move::{BoardMove, PieceMove}, game::Game, square::{File, Level, Rank, Square}};

fn main() {
    let mut game = Game::new();

    game.get_attack_squares(&Square::new(Rank::One, File::A, Level::White)).iter().for_each(|x| println!("{:?}", x));

    let piece_move = PieceMove::new(Square::new(Rank::One, File::A, Level::White), Square::new(Rank::Three, File::B, Level::White), None);

    if game.legal_move(&piece_move) {
        let _ = game.push_move(piece_move);
        println!("Success")
    }

    game.push_move(BoardMove::new(Level::QL1, Level::QL2));

    game.print();
}
