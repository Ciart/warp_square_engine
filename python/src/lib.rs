use pyo3::prelude::*;
use ::warp_square_engine::{
    game::Game as BaseGame,
    piece::{Piece as BasePiece, PieceType},
    square::{Color, File, Level, Rank, Square as BaseSquare},
};

#[pyclass]
struct Piece(BasePiece);

#[pymethods]
impl Piece {
    fn get_piece_type(&self) -> PieceType {
        self.0.piece_type
    }

    fn get_color(&self) -> Color {
        self.0.color
    }

    fn get_square(&self) -> Square {
        let square = self.0.position.into_square();

        Square::new(square.rank, square.file, square.level)
    }

    fn get_char(&self) -> String {
        self.0.get_char().to_string()
    }
}

#[pyclass]
struct Square(BaseSquare);

#[pymethods]
impl Square {
    #[new]
    fn new(rank: Rank, file: File, level: Level) -> Self {
        Self(BaseSquare::new(rank, file, level))
    }

    fn get_rank(&self) -> Rank {
        self.0.rank
    }

    fn get_file(&self) -> File {
        self.0.file
    }

    fn get_level(&self) -> Level {
        self.0.level
    }
}

#[pyclass]
struct Game(BaseGame);

#[pymethods]
impl Game {
    #[new]
    fn new() -> Self {
        Self(BaseGame::new())
    }

    // fn get_attack_squares(&self, square: Square) -> Vec<Square> {
    //     self.get_attack_squares(square.0)
    // }
}

#[pymodule]
fn warp_square_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PieceType>()?;
    m.add_class::<Rank>()?;
    m.add_class::<File>()?;
    m.add_class::<Level>()?;
    m.add_class::<Color>()?;
    m.add_class::<Piece>()?;
    m.add_class::<Square>()?;
    m.add_class::<Game>()?;
    // m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    Ok(())
}
