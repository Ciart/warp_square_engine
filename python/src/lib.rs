use pyo3::prelude::*;
use ::warp_square_engine::{
    bit_board::BitBoard, chess_move::{BoardMove, PieceMove}, game::Game as BaseGame, piece::{Piece as BasePiece, PieceType}, square::{Color, File, Level, Rank, Square}
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

    fn get_position(&self) -> u64 {
        self.0.position.bits()
    }

    fn get_square(&self) -> PySquare {
        let square = self.0.position.into_square();

        PySquare(square)
    }

    fn get_char(&self) -> String {
        self.0.get_char().to_string()
    }
}

#[pyclass]
#[pyo3(name = "PieceMove")]
struct PyPieceMove(PieceMove);

#[pymethods]
impl PyPieceMove {
    #[new]
    fn new(source: &PySquare, destination: &PySquare, promotion: Option<PieceType>) -> Self {
        Self(PieceMove::new(source.0.clone(), destination.0.clone(), promotion))
    }

    fn get_source(&self) -> PySquare {
        PySquare(self.0.source.clone())
    }

    fn get_destination(&self) -> PySquare {
        PySquare(self.0.destination.clone())
    }

    fn get_promotion(&self) -> Option<PieceType> {
        self.0.promotion
    }
}

#[pyclass]
#[pyo3(name = "BoardMove")]
struct PyBoardMove(BoardMove);

#[pymethods]
impl PyBoardMove {
    #[new]
    fn new(source: Level, destination: Level, promotion: Option<PieceType>) -> Self {
        Self(BoardMove::new(source, destination, promotion))
    }

    fn get_source(&self) -> Level {
        self.0.source
    }

    fn get_destination(&self) -> Level {
        self.0.destination
    }

    fn get_promotion(&self) -> Option<PieceType> {
        self.0.promotion
    }
}

#[pyclass]
#[pyo3(name = "Square")]
struct PySquare(Square);

#[pymethods]
impl PySquare {
    #[new]
    fn new(rank: Rank, file: File, level: Level) -> Self {
        Self(Square::new(rank, file, level))
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

    fn get_attack_squares(&self, square: &PySquare) -> Vec<PySquare> {
        self.0.get_attack_squares(&square.0).iter().map(|square| PySquare(square.clone())).collect()
    }

    fn legal_piece_move(&self, piece_move: &PyPieceMove) -> bool {
        self.0.legal_move(&piece_move.0)
    }

    fn legal_board_move(&self, board_move: &PyBoardMove) -> bool {
        self.0.legal_move(&board_move.0)
    }

    fn push_piece_move(&mut self, piece_move: &PyPieceMove) -> bool {
        self.0.push_move(piece_move.0.clone())
    }

    fn push_board_move(&mut self, board_move: &PyBoardMove) -> bool {
        self.0.push_move(board_move.0.clone())
    }

    fn pop_move(&mut self) -> bool {
        self.0.pop_move()
    }

    fn print(&self) {
        self.0.print();
    }

    fn get_turn(&self) -> Color {
        self.0.board_set.turn
    }

    fn get_full_move_number(&self) -> u32 {
        self.0.board_set.full_move_number
    }

    fn get_half_move_clock(&self) -> u32 {
        self.0.board_set.half_move_clock
    }

    fn get_piece(&self, square: &PySquare) -> Option<Piece> {
        self.0.board_set.get_piece(BitBoard::from_square(&square.0)).map(|piece| Piece(piece.clone()))
    }

    fn get_pieces(&self) -> Vec<Piece> {
        self.0.board_set.pieces.iter().map(|piece| Piece(piece.clone())).collect()
    }

    fn get_captured_pieces(&self) -> Vec<Piece> {
        self.0.board_set.captured_pieces.iter().map(|piece| Piece(piece.clone())).collect()
    }

    fn is_check(&self) -> bool {
        self.0.board_set.is_check()
    }

    fn is_checkmate(&self) -> bool {
        self.0.board_set.is_checkmate()
    }

    fn is_promotion(&self, piece_move: &PyPieceMove) -> bool {
        piece_move.0.is_promotion(&self.0.board_set)
    }

    fn is_en_passant(&self, piece_move: &PyPieceMove) -> bool {
        piece_move.0.is_en_passant(&self.0.board_set)
    }

    fn is_king_side_castling(&self, piece_move: &PyPieceMove) -> bool {
        piece_move.0.is_king_side_castling(&self.0.board_set)
    }

    fn is_queen_side_castling(&self, piece_move: &PyPieceMove) -> bool {
        piece_move.0.is_queen_side_castling(&self.0.board_set)
    }

    fn is_castling(&self, piece_move: &PyPieceMove) -> bool {
        piece_move.0.is_castling(&self.0.board_set)
    }

    fn is_capture(&self, piece_move: &PyPieceMove) -> bool {
        piece_move.0.is_capture(&self.0.board_set)
    }
}

#[pymodule]
fn warp_square_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PieceType>()?;
    m.add_class::<Rank>()?;
    m.add_class::<File>()?;
    m.add_class::<Level>()?;
    m.add_class::<Color>()?;
    m.add_class::<Piece>()?;
    m.add_class::<PyPieceMove>()?;
    m.add_class::<PySquare>()?;
    m.add_class::<Game>()?;
    Ok(())
}
