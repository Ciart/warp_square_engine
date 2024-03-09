#[cfg(test)]
mod single_function_unit_test {
    use crate::board::Board;
    use crate::chess_move::{BoardMove, PieceMove};
    use crate::game::Game;
    use crate::piece::{Piece, PieceType};
    use crate::square::{Color, File, Level, Rank, Square};

    #[test]
    fn check_turn_pass() {
        let mut test_game = Game::new_sandbox("4/3P/4/4/4/4/4/4/4/4/p3/4/q1P12/q622/k1p12/k622".to_string());

        assert_eq!( match test_game.board.turn {
            Color::White => true,
            _ => false
        }, true, "첫 화이트 턴 체크");

        let legal_piece_move = PieceMove::new(
            Square::new(Rank::Two, File::D, Level::White),
            Square::new(Rank::Three, File::D, Level::White),
            None
        );
        let illegal_piece_move = PieceMove::new(
            Square::new(Rank::Seven, File::A, Level::Black),
            Square::new(Rank::Six, File::A, Level::Black),
            None
        );
        let white_board_move = BoardMove::new(Level::QL1, Level::QL2, None);

        assert_eq!(test_game.legal_move(&legal_piece_move), true, "화이트 턴 화이트 기물 움직임");
        assert_ne!(test_game.legal_move(&illegal_piece_move), true, "화이트 턴 블랙 기물 움직임");
        assert_eq!(test_game.legal_move(&white_board_move), true, "화이트 턴 화이트 보드 움직임");

        let _ = test_game.push_move(legal_piece_move);

        assert_eq!( match test_game.board.turn {
            Color::Black => true,
            _ => false
        }, true, "두번째 블랙 턴 체크");

        let legal_piece_move = PieceMove::new(
            Square::new(Rank::Seven, File::A, Level::Black),
            Square::new(Rank::Six, File::A, Level::Black),
            None
        );
        let illegal_piece_move = PieceMove::new(
            Square::new(Rank::Three, File::D, Level::White),
            Square::new(Rank::Four, File::D, Level::White),
            None
        );
        let black_board_move = BoardMove::new(Level::KL1, Level::KL2, None);

        assert_eq!(test_game.legal_move(&legal_piece_move), true, "블랙 턴 블랙 기물 움직임");
        assert_ne!(test_game.legal_move(&illegal_piece_move), true, "블랙 턴 화이트 기물 움직임");
        assert_eq!(test_game.legal_move(&black_board_move), true, "블랙 턴 블랙 보드 움직임");
    }

    #[test]
    fn board_move() {
        let test_game = Game::new_sandbox("4/4/4/4/4/4/4/4/4/4/4/4/q1P12/q6PK/k122/k6B12".to_string());

        let board_move = BoardMove::new(Level::QL1, Level::QL2, None);
        assert_eq!(test_game.legal_move(&board_move), true, "보드에 pawn 1개만 있을때");
        let board_move = BoardMove::new(Level::KL1, Level::KL2, None);
        assert_eq!(test_game.legal_move(&board_move), true, "보드에 아무것도 없을때");
        let board_move = BoardMove::new(Level::KL6, Level::KL5, None);
        assert_ne!(test_game.legal_move(&board_move), true, "보드에 pawn 아닌 기물 1개 일때");
        let board_move = BoardMove::new(Level::QL6, Level::QL5, None);
        assert_ne!(test_game.legal_move(&board_move), true, "보드에 기물이 여러개 일때");

    }

    #[test]
    fn normal_move() {
        let test_game = Game::new_sandbox("4/4/2QP/4/4/4/4/4/4/4/4/4/q122/q222/k222/k622".to_string());

        let legal_piece_move = PieceMove::new(
            Square::new(Rank::Three, File::D, Level::White),
            Square::new(Rank::Four, File::D, Level::White),
            None
        );
        let illegal_piece_move = PieceMove::new(
            Square::new(Rank::Three, File::D, Level::White),
            Square::new(Rank::Four, File::C, Level::White),
            None
        );

        assert_eq!(test_game.legal_move(&legal_piece_move), true, "W pawn 같은 Level 빈 square 위치로 움직임");
        assert_ne!(test_game.legal_move(&illegal_piece_move), true, "W pawn 같은 Level 빈 square 위치로 움직임");

        let legal_piece_move = PieceMove::new(
            Square::new(Rank::Three, File::D, Level::White),
            Square::new(Rank::Five, File::D, Level::Black),
            None
        );
        let illegal_piece_move = PieceMove::new(
            Square::new(Rank::Three, File::D, Level::White),
            Square::new(Rank::Five, File::C, Level::Black),
            None
        );

        assert_eq!(test_game.legal_move(&legal_piece_move), true, "W pawn 다른 Level 빈 square 위치로 움직임");
        assert_ne!(test_game.legal_move(&illegal_piece_move), true, "W pawn 다른 Level 빈 square 위치로 움직임");

        let legal_piece_move = PieceMove::new(
            Square::new(Rank::Three, File::D, Level::White),
            Square::new(Rank::Four, File::D, Level::KL2),
            None
        );
        let illegal_piece_move = PieceMove::new(
            Square::new(Rank::Three, File::D, Level::White),
            Square::new(Rank::Five, File::E, Level::KL2),
            None
        );

        assert_eq!(test_game.legal_move(&legal_piece_move), true, "W pawn Sub_Level 빈 square 위치로 움직임");
        assert_ne!(test_game.legal_move(&illegal_piece_move), true, "W pawn Sub_Level 빈 square 위치로 움직임");

        let legal_piece_move = PieceMove::new(
            Square::new(Rank::Three, File::C, Level::White),
            Square::new(Rank::Four, File::D, Level::White),
            None
        );
        let illegal_piece_move = PieceMove::new(
            Square::new(Rank::Three, File::C, Level::White),
            Square::new(Rank::Four, File::A, Level::White),
            None
        );

        assert_eq!(test_game.legal_move(&legal_piece_move), true, "W Queen 같은 Level 빈 square 위치로 움직임");
        assert_ne!(test_game.legal_move(&illegal_piece_move), true, "W Queen 같은 Level 빈 square 위치로 움직임");

        let legal_piece_move = PieceMove::new(
            Square::new(Rank::Three, File::C, Level::White),
            Square::new(Rank::Five, File::A, Level::Black),
            None
        );
        let illegal_piece_move = PieceMove::new(
            Square::new(Rank::Three, File::C, Level::White),
            Square::new(Rank::Four, File::A, Level::Black),
            None
        );

        assert_eq!(test_game.legal_move(&legal_piece_move), true, "W Queen 다른 Level 빈 square 위치로 움직임");
        assert_ne!(test_game.legal_move(&illegal_piece_move), true, "W Queen 다른 Level 빈 square 위치로 움직임");

        let legal_piece_move = PieceMove::new(
            Square::new(Rank::Three, File::C, Level::White),
            Square::new(Rank::Five, File::E, Level::KL2),
            None
        );

        assert_eq!(test_game.legal_move(&legal_piece_move), true, "W Queen Sub_Level 빈 square 위치로 움직임");
    }

    #[test]
    fn contact_with_piece() {
        let test_game = Game::new_sandbox("1Q2/P1p1/4/4/4/2P1/1Ppp/4/4/4/4/4/q122/q222/k222/k622".to_string());

        let legal_piece_move = PieceMove::new(
            Square::new(Rank::Four, File::C, Level::Neutral),
            Square::new(Rank::Five, File::D, Level::Neutral),
            None
        );
        let illegal_piece_move = PieceMove::new(
            Square::new(Rank::Four, File::C, Level::Neutral),
            Square::new(Rank::Five, File::B, Level::Neutral),
            None
        );
        let illegal_piece_move2 = PieceMove::new(
            Square::new(Rank::Four, File::C, Level::Neutral),
            Square::new(Rank::Five, File::C, Level::Neutral),
            None
        );

        assert_eq!(test_game.legal_move(&legal_piece_move), true, "W Pawn B Pawn 공격 위치로 움직임");
        assert_ne!(test_game.legal_move(&illegal_piece_move), true, "W Pawn W Pawn 공격 위치로 움직임");
        assert_ne!(test_game.legal_move(&illegal_piece_move2), true, "W Pawn B Pawn 전진 위치로 움직임");

        let legal_piece_move = PieceMove::new(
            Square::new(Rank::One, File::B, Level::White),
            Square::new(Rank::Two, File::C, Level::White),
            None
        );
        let illegal_piece_move = PieceMove::new(
            Square::new(Rank::One, File::B, Level::White),
            Square::new(Rank::Two, File::A, Level::White),
            None
        );
        let illegal_piece_move2 = PieceMove::new(
            Square::new(Rank::One, File::B, Level::White),
            Square::new(Rank::Three, File::D, Level::White),
            None
        );

        assert_eq!(test_game.legal_move(&legal_piece_move), true, "W Queen B Pawn 공격 위치로 움직임");
        assert_ne!(test_game.legal_move(&illegal_piece_move), true, "W Queen W Pawn 공격 위치로 움직임");
        assert_ne!(test_game.legal_move(&illegal_piece_move2), true, "W Queen 공격경로에 B Pawn 있을때 그 위치 넘어로의 움직임");
    }

    #[test]
    fn en_passant() {
        let mut test_game = Game::new_sandbox("4/PPPP/4/4/4/4/4/4/4/4/pppp/4".to_string());

        let piece_move = PieceMove::new(
            Square::new(Rank::Two, File::C, Level::White),
            Square::new(Rank::Four, File::C, Level::White),
            None
        );
        let _ = test_game.push_move(piece_move);

        let piece_move = PieceMove::new(
            Square::new(Rank::Seven, File::A, Level::Black),
            Square::new(Rank::Six, File::A, Level::Black),
            None
        );
        let _ = test_game.push_move(piece_move);

        let piece_move = PieceMove::new(
            Square::new(Rank::Four, File::C, Level::White),
            Square::new(Rank::Five, File::C, Level::Black),
            None
        );
        let _ = test_game.push_move(piece_move);

        let piece_move = PieceMove::new(
            Square::new(Rank::Seven, File::B, Level::Black),
            Square::new(Rank::Five, File::B, Level::Black),
            None
        );
        let _ = test_game.push_move(piece_move);

        let legal_piece_move = PieceMove::new(
            Square::new(Rank::Six, File::C, Level::Black),
            Square::new(Rank::Seven, File::B, Level::Black),
            None
        );

        assert_eq!(test_game.legal_move(&legal_piece_move), true, "화이트 피스 공격 앙파상");
    }

    #[test]
    fn castling() {
        let test_game = Game::new_sandbox("4/4/4/4/4/4/4/4/4/4/4/4/q1R12/q62r1/k1KR2/k62k1".to_string());
        test_game.print_sandbox();

        test_game.get_attack_squares(&Square::new(Rank::Zero, File::D, Level::KL1))
            .iter()
            .for_each(|x| println!("{:?}", x));
        // Zero E KL1
        // Zero A QL1
        println!(" ");
        test_game.get_attack_squares(&Square::new(Rank::Nine, File::D, Level::KL6))
            .iter()
            .for_each(|x| println!("{:?}", x));
        // Nine E KL6
        // Nine A QL6

        let white_king_side_castling = PieceMove::new(
            Square::new(Rank::Zero, File::D, Level::KL1),
            Square::new(Rank::Zero, File::E, Level::KL1),
            None
        );
        let white_queen_side_castling = PieceMove::new(
            Square::new(Rank::Zero, File::D, Level::KL1),
            Square::new(Rank::Zero, File::A, Level::QL1),
            None
        );

        assert_eq!(PieceMove::is_king_side_castling(&white_king_side_castling, &test_game.board), true, "화이트 킹 사이드 캐슬링");
        assert_eq!(PieceMove::is_queen_side_castling(&white_queen_side_castling, &test_game.board), true, "화이트 퀸 사이드 캐슬링");

        let black_king_side_castling = PieceMove::new(
            Square::new(Rank::Nine, File::D, Level::KL6),
            Square::new(Rank::Nine, File::E, Level::KL6),
            None
        );
        let black_queen_side_castling = PieceMove::new(
            Square::new(Rank::Nine, File::D, Level::KL6),
            Square::new(Rank::Nine, File::A, Level::QL6),
            None
        );

        assert_eq!(PieceMove::is_king_side_castling(&black_king_side_castling, &test_game.board), true, "블랙 킹 사이드 캐슬링");
        assert_eq!(PieceMove::is_queen_side_castling(&black_queen_side_castling, &test_game.board), true, "블랙 퀸 사이드 캐슬링");

        let test_game = Game::new_sandbox("4/4/4/b3/4/4/4/4/3B/4/4/4/q1R12/k1KR2/q62r1/k62kr".to_string());

        assert_eq!(PieceMove::is_king_side_castling(&white_king_side_castling, &test_game.board), true, "블랙 피스가 견제할때 화이트 킹 사이드 캐슬링");
        assert_eq!(PieceMove::is_queen_side_castling(&black_queen_side_castling, &test_game.board), true, "화이트 피스가 견제할때 블랙 퀸 사이드 캐슬링");

        let mut test_game = Game::new_sandbox("NBBN/4/4/4/4/4/4/4/4/4/4/nbbn/q1R12/k1KR2/q62r1/k62kr".to_string());

        assert_eq!(PieceMove::is_queen_side_castling(&white_queen_side_castling, &test_game.board), true, "중간에 피스가 있을때 화이트 퀸 사이드 캐슬링");
        assert_eq!(PieceMove::is_queen_side_castling(&black_queen_side_castling, &test_game.board), true, "중간에 피스가 있을때 블랙 퀸 사이드 캐슬링");

        let piece_move = PieceMove::new(
            Square::new(Rank::Zero, File::E, Level::KL1),
            Square::new(Rank::One, File::E, Level::KL1),
            None
        );
        let _ = test_game.push_move(piece_move);
        let piece_move = PieceMove::new(
            Square::new(Rank::One, File::E, Level::KL1),
            Square::new(Rank::Zero, File::E, Level::KL1),
            None
        );
        let _ = test_game.push_move(piece_move);

        assert_eq!(PieceMove::is_king_side_castling(&white_king_side_castling, &test_game.board), true, "이미 첫 움직임을 한 후에 화이트 킹 사이드 캐슬링");
    }

    #[test]
    fn promotion() {
        let test_game = Game::new_sandbox("4/4/bnqp/4/4/4/4/4/4/BNQP/4/4/q2pr/k222/q5PR/k522".to_string());

        let white_bishop_move = PieceMove::new(
            Square::new(Rank::Seven, File::A, Level::Black),
            Square::new(Rank::Nine, File::C, Level::Black),
            Option::from(PieceType::Queen)
        );
        let white_knight_move = PieceMove::new(
            Square::new(Rank::Seven, File::B, Level::Black),
            Square::new(Rank::Nine, File::A, Level::Black),
            Option::from(PieceType::Queen)
        );
        let white_queen_move = PieceMove::new(
            Square::new(Rank::Seven, File::C, Level::Black),
            Square::new(Rank::Nine, File::C, Level::Black),
            Option::from(PieceType::Knight)
        );
        let white_pawn_move = PieceMove::new(
            Square::new(Rank::Seven, File::D, Level::Black),
            Square::new(Rank::Nine, File::D, Level::Black),
            Option::from(PieceType::Queen)
        );
        let white_pawn_move_underpromotion = PieceMove::new(
            Square::new(Rank::Seven, File::D, Level::Black),
            Square::new(Rank::Nine, File::D, Level::Black),
            Option::from(PieceType::Bishop)
        );

        println!("{}", PieceMove::is_promotion(&white_pawn_move, &test_game.board));

        assert_ne!(PieceMove::is_promotion(&white_bishop_move, &test_game.board), true, "화이트 비숍 프로모션");
        assert_ne!(PieceMove::is_promotion(&white_knight_move, &test_game.board), true, "화이트 나이트 프로모션");
        assert_ne!(PieceMove::is_promotion(&white_queen_move, &test_game.board), true, "화이트 퀸 프로모션");
        assert_eq!(PieceMove::is_promotion(&white_pawn_move, &test_game.board), true, "화이트 폰 프로모션");
        assert_eq!(white_pawn_move_underpromotion.is_promotion(&test_game.board), true, "화이트 폰 언더 프로모션");
    }

    #[test]
    fn check() {
        let test_game = Game::new_sandbox("2K1/PP1P/2q1/4/4/4/4/4/4/4/p1pp/1k2/q122/k122/q622/k622".to_string());
        test_game.print_sandbox();

        assert_eq!(test_game.board.is_check(), true, "?");
    }

    #[test]
    fn checkmate() {
        let test_game = Game::new_sandbox("PPKP/PP1P/2q1/4/4/4/4/4/4/4/p1pp/1k2/q122/k122/q622/k622".to_string());
        test_game.print_sandbox();

        assert_eq!(test_game.board.is_checkmate(), true, "?");
    }

    fn stalemate() {

    }
}
