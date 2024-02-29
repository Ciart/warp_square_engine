#[cfg(test)]
mod single_function_unit_test {
    use crate::bit_board::BitBoard;
    use crate::chess_move::{BoardMove, PieceMove};
    use crate::game::Game;
    use crate::square::{File, Level, Rank, Square};
    use super::*;

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

        test_game.print_sandbox();
    }

    #[test]
    fn board_move() {
        let mut test_game = Game::new_sandbox("4/4/4/4/4/4/4/4/4/4/4/4/q1P12/q6PK/k122/k6B12".to_string());
        test_game.print_sandbox();

        let board_move = BoardMove::new(Level::QL1, Level::QL2, None);
        assert_eq!(test_game.legal_move(&board_move), true, "보드에 pawn 1개만 있을때");
        let board_move = BoardMove::new(Level::KL1, Level::KL2, None);
        assert_eq!(test_game.legal_move(&board_move), true, "보드에 아무것도 없을때");
        let board_move = BoardMove::new(Level::KL6, Level::KL5, None);
        assert_ne!(test_game.legal_move(&board_move), true, "보드에 pawn 아닌 기물 1개 일때");
        let board_move = BoardMove::new(Level::QL6, Level::QL5, None);
        assert_ne!(test_game.legal_move(&board_move), true, "보드에 기물이 여러개 일때");

    }
}
