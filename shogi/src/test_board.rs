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
    fn case01() {
        let board = Board::new();
        assert_eq!(board.is_frame, BitBoard::from_u128(BIT_OF_FRAME));
        assert_eq!(
            board.able_pro[0],
            BitBoard::from_u128(BIT_OF_PRO_ZONE_BLACK)
        );
    }

    #[test]
    fn case02() {
        let mut board = Board::new();
        board.startpos();
        assert_eq!(board.get_piece_type_from_index(12), PieceType::Lance);
        assert_eq!(board.get_color_type_from_index(12), ColorType::Black);
    }

    #[test]
    fn case03() {
        let mut board = Board::new();
        board.startpos();
        let result1 = board.get_able_move_squares(34);
        assert_eq!(result1.get_trues(), vec![45]);
        let result2 = board.get_able_move_squares(78);
        assert_eq!(result2.get_trues(), vec![67]);
        let result1 = board.get_able_move_squares(30);
        assert_eq!(result1.get_trues(), vec![25, 26, 27, 28, 29, 31]);
    }

    #[test]
    fn case04() {
        let mut board = Board::new();
        board.deploy(12, PieceType::Rook, ColorType::Black);
        let bit_movable = board.get_able_move_squares(12);
        let result = board.get_able_pro_move_squares(12, bit_movable);
        assert_eq!(result.get_trues(), vec![78, 89, 100]);
    }

    #[test]
    fn case05() {
        let mut board = Board::new();
        board.startpos();
        let result1 = board.get_able_drop_squares(ColorType::Black, PieceType::Pawn);
        assert_eq!(result1.get_trues(), vec![]);
        let result2 = board.get_able_drop_squares(ColorType::White, PieceType::Knight);
        assert_eq!(result2.get_trues().len(), 34);
        assert_eq!(result2.get_trues()[0], 45);
    }

    #[test]
    fn case06() {
        let mut board = Board::new();
        board.startpos();
        let result = board.serch_moves(ColorType::Black);
        assert_eq!(result.len(), 30);
    }

    #[test]
    fn case07() {
        let mut board = Board::new();
        board.startpos();
        let from = Address::from_number(34);
        let to = Address::from_number(45);
        board.execute_move(&Move::from_standart(from, to, false));
        assert_eq!(board.get_piece_type_from_index(45), PieceType::Pawn);
        assert_eq!(board.get_color_type_from_index(45), ColorType::Black);
    }

    #[test]
    fn case08() {
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
