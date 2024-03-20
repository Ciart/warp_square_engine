use ::warp_square_engine::{
    bit_board::BitBoard,
    chess_move::{BoardMove, PieceMove},
    game::Game,
    piece::{Piece, PieceType},
    square::{Color, File, Level, Rank, Square},
};

foreign_enum!(
    enum PieceType {
        Pawn = PieceType::Pawn,
        Knight = PieceType::Knight,
        Bishop = PieceType::Bishop,
        Rook = PieceType::Rook,
        Queen = PieceType::Queen,
        King = PieceType::King,
    }
);

foreign_enum!(
    enum Rank {
        Zero = Rank::Zero,
        One = Rank::One,
        Two = Rank::Two,
        Three = Rank::Three,
        Four = Rank::Four,
        Five = Rank::Five,
        Six = Rank::Six,
        Seven = Rank::Seven,
        Eight = Rank::Eight,
        Nine = Rank::Nine,
    }
);

foreign_enum!(
    enum File {
        Z = File::Z,
        A = File::A,
        B = File::B,
        C = File::C,
        D = File::D,
        E = File::E,
    }
);

foreign_enum!(
    enum Level {
        White = Level::White,
        Neutral = Level::Neutral,
        Black = Level::Black,
        QL1 = Level::QL1,
        QL2 = Level::QL2,
        QL3 = Level::QL3,
        QL4 = Level::QL4,
        QL5 = Level::QL5,
        QL6 = Level::QL6,
        KL1 = Level::KL1,
        KL2 = Level::KL2,
        KL3 = Level::KL3,
        KL4 = Level::KL4,
        KL5 = Level::KL5,
        KL6 = Level::KL6,
    }
);

foreign_enum!(
    enum Color {
        White = Color::White,
        Black = Color::Black,
    }
);

foreign_class!(
    class BitBoard {
        self_type BitBoard;
        private constructor = empty;
        fn BitBoard::get_rank(&self) -> Rank; alias getRank;
        fn BitBoard::get_file(&self) -> File; alias getFile;
        fn BitBoard::get_level(&self) -> Level; alias getLevel;
        fn BitBoard::from_square(square: &Square) -> BitBoard; alias fromSquare;
        fn BitBoard::into_square(&self) -> Square; alias intoSquare;
        fn BitBoard::from_hex(hex: &str) -> BitBoard; alias fromHex;
        fn BitBoard::to_hex(&self) -> String; alias toHex;
        fn BitBoard::remove_level(&self) -> BitBoard; alias removeLevel;
    }
);

foreign_class!(class Piece {
    self_type Piece;
        private constructor = empty;
        fn Piece::getPieceType(&self) -> PieceType {
            this.piece_type
        }
        fn Piece::getColor(&self) -> Color {
            this.color
        }
        fn Piece::getPosition(&self) -> BitBoard {
            this.position
        }
        fn Piece::get_square(&self) -> Square; alias getSquare;
        fn Piece::get_char(&self) -> &'static str; alias getChar;
    }
);

foreign_class!(
    class PieceMove {
        self_type PieceMove;
        constructor PieceMove::new_move(source: &Square, destination: &Square, promotion: Option<PieceType>) -> PieceMove {
            PieceMove {
                source: source.clone(),
                destination: destination.clone(),
                promotion,
            }
        }
        fn PieceMove::getSource(&self) -> Square {
            this.source.clone()
        }
        fn PieceMove::getDestination(&self) -> Square {
            this.destination.clone()
        }
        fn PieceMove::getPromotion(&self) -> Option<PieceType> {
            this.promotion.clone()
        }
    }
);

foreign_class!(
    class BoardMove {
        self_type BoardMove;
        constructor BoardMove::new(source: Level, destination: Level, promotion: Option<PieceType>) -> BoardMove;
        fn BoardMove::getSource(&self) -> Level {
            this.source.clone()
        }
        fn BoardMove::getDestination(&self) -> Level {
            this.destination.clone()
        }
        fn BoardMove::getPromotion(&self) -> Option<PieceType> {
            this.promotion.clone()
        }
    }
);

foreign_class!(
    class Square {
        self_type Square;
        constructor Square::new(rank: Rank, file: File, level: Level) -> Square;
        fn Square::getRank(&self) -> Rank {
            this.rank
        }
        fn Square::getFile(&self) -> File {
            this.file
        }
        fn Square::getLevel(&self) -> Level {
            this.level
        }
    }
);

foreign_class!(class Game {
    self_type Game;
    constructor Game::new() -> Game;
    fn Game::get_attack_squares(&self, square: &Square) -> Vec<Square>; alias getAttackSquares;
    fn Game::legalPieceMove(&self, pieceMove: &PieceMove) -> bool {
        this.legal_move(pieceMove)
    }
    fn Game::legalBoardMove(&self, boardMove: &BoardMove) -> bool {
        this.legal_move(boardMove)
    }
    fn Game::pushPieceMove(&mut self, pieceMove: &PieceMove) {
        let _ = this.push_move(pieceMove.clone());
    }
    fn Game::pushBoardMove(&mut self, boardMove: &BoardMove) {
        let _ = this.push_move(boardMove.clone());
    }
    fn Game::popMove(&mut self) {
        let _ = this.pop_move();
    }
    fn Game::print(&self);
    fn Game::getTurn(&self) -> Color {
        this.board.turn
    }
    fn Game::getFullMoveNumber(&self) -> u32 {
        this.board.full_move_number
    }
    fn Game::getHalfMoveClock(&self) -> u32 {
        this.board.half_move_clock
    }
    fn Game::getPieces(&self) -> Vec<Piece> {
        this.board.pieces.clone()
    }
    fn Game::getCapturedPieces(&self) -> Vec<Piece> {
        this.board.captured_pieces.clone()
    }
    fn Game::isCheck(&self) -> bool {
        this.board.is_check()
    }
    fn Game::isCheckmate(&self) -> bool {
        this.board.is_checkmate()
    }
    fn Game::isPromotion(&self, pieceMove: &PieceMove) -> bool {
        pieceMove.is_promotion(&this.board)
    }
    fn Game::isEnPassant(&self, pieceMove: &PieceMove) -> bool {
        pieceMove.is_en_passant(&this.board)
    }
    fn Game::isKingSideCastling(&self, pieceMove: &PieceMove) -> bool {
        pieceMove.is_king_side_castling(&this.board)
    }
    fn Game::isQueenSideCastling(&self, pieceMove: &PieceMove) -> bool {
        pieceMove.is_queen_side_castling(&this.board)
    }
    fn Game::isCastling(&self, pieceMove: &PieceMove) -> bool {
        pieceMove.is_castling(&this.board)
    }
    fn Game::isCapture(&self, pieceMove: &PieceMove) -> bool {
        pieceMove.is_capture(&this.board)
    }
});