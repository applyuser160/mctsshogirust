#[cfg(test)]

mod tests {
    use crate::{
        address::Address,
        bitboard::{BitBoard, BIT_OF_FRAME, BIT_OF_PRO_ZONE_BLACK},
        board::Board,
        color::ColorType,
        moves::Move,
        piece::PieceType,
    };

    #[test]
    fn test_board_new() {
        let board = Board::new();
        assert_eq!(board.is_frame, BitBoard::from_u128(BIT_OF_FRAME));
        assert_eq!(
            board.able_pro[0],
            BitBoard::from_u128(BIT_OF_PRO_ZONE_BLACK)
        );
    }

    #[test]
    fn test_board_startpos() {
        let mut board = Board::new();
        board.startpos();
        assert_eq!(board.get_piece_type_from_index(12), PieceType::Lance);
        assert_eq!(board.get_color_type_from_index(12), ColorType::Black);
    }

    #[test]
    fn test_board_get_able_move_squares() {
        let mut board = Board::new();
        board.startpos();
        let result1 = board.get_able_move_squares(34);
        let mut result1_trues = result1.get_trues();
        result1_trues.sort();
        assert_eq!(result1_trues, vec![45]);
        let result2 = board.get_able_move_squares(78);
        let mut result2_trues = result2.get_trues();
        result2_trues.sort();
        assert_eq!(result2_trues, vec![67]);
        let result3 = board.get_able_move_squares(30);
        let mut result3_trues = result3.get_trues();
        result3_trues.sort();
        assert_eq!(result3_trues, vec![25, 26, 27, 28, 29, 31]);
    }

    #[test]
    fn test_board_deploy() {
        let mut board = Board::new();
        board.deploy(12, PieceType::Rook, ColorType::Black);
        let bit_movable = board.get_able_move_squares(12);
        let result = board.get_able_pro_move_squares(12, bit_movable);
        let mut result_trues = result.get_trues();
        result_trues.sort();
        assert_eq!(result_trues, vec![78, 89, 100]);
    }

    #[test]
    fn test_board_get_able_drop_squares() {
        let mut board = Board::new();
        board.startpos();
        let result1 = board.get_able_drop_squares(ColorType::Black, PieceType::Pawn);
        assert_eq!(result1.get_trues(), vec![] as Vec<u8>);
        let result2 = board.get_able_drop_squares(ColorType::White, PieceType::Knight);
        let mut result2_trues = result2.get_trues();
        result2_trues.sort();
        assert_eq!(result2_trues.len(), 34);
        assert_eq!(result2_trues[0], 45);
    }

    #[test]
    fn test_board_search_moves() {
        let mut board = Board::new();
        board.startpos();
        let result = board.search_moves(ColorType::Black);
        assert_eq!(result.len(), 30);
    }

    #[test]
    fn test_board_execute_move() {
        let mut board = Board::new();
        board.startpos();
        let from = Address::from_number(34);
        let to = Address::from_number(45);
        board.execute_move(&Move::from_standart(from, to, false));
        assert_eq!(board.get_piece_type_from_index(45), PieceType::Pawn);
        assert_eq!(board.get_color_type_from_index(45), ColorType::Black);
    }

    #[test]
    fn test_board_is_finished() {
        let mut board = Board::new();
        board.startpos();
        let from = Address::from_number(93);
        let to = Address::from_number(16);
        board.execute_move(&Move::from_standart(from, to, false));
        let result = board.is_finished();
        assert_eq!(result.0, true);
        assert_eq!(result.1, ColorType::White);
    }
}
