#[cfg(test)]

mod tests {
    use crate::bitboard::{generate_column, generate_columns, BitBoard};


    #[test]
    fn case01() {
        let bitboard = BitBoard::new();
        assert_eq!(bitboard.board[0], false);
    }

    #[test]
    fn case02() {
        let bitboard = BitBoard::from_u128(548949983232);
        assert_eq!(bitboard.board[88], false);
        assert_eq!(bitboard.board[89], true);
        assert_eq!(bitboard.board[90], true);
    }

    #[test]
    fn case03() {
        let s = "\
        00000000000\
        00000000000\
        00000000000\
        00000000000\
        00000000000\
        00000000000\
        00000000000\
        00000000000\
        01111111110\
        01111111110\
        00000000000";
        let bitboard = BitBoard::from_str(s);
        assert_eq!(bitboard.board[88], false);
        assert_eq!(bitboard.board[89], true);
        assert_eq!(bitboard.board[90], true);
    }

    #[test]
    fn case04() {
        let bitboard = BitBoard::from_u128(548949983232);
        let num = bitboard.to_u128();
        assert_eq!(num, 548949983232);
    }

    #[test]
    fn case05() {
        let bitboard = BitBoard::from_u128(548949983232);
        let trues = bitboard.get_trues();
        assert_eq!(trues[0], 89);
        assert_eq!(trues.len(), 18);
    }

    #[test]
    fn case06() {
        let mut bitboard = BitBoard::from_u128(548949983232);
        bitboard.flip();
        assert_eq!(bitboard.board[88], true);
        assert_eq!(bitboard.board[89], false);
        assert_eq!(bitboard.board[90], false);
        assert_eq!(bitboard.get_trues().len(), 103);
    }

    #[test]
    fn case07() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = BitBoard::from_u128(548949983232);
        let bb3 = bb1 & bb2.clone();
        assert_eq!(bb2.get_trues(), bb3.get_trues());
    }

    #[test]
    fn case08() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = BitBoard::from_u128(548949983232);
        let bb3 = bb1.clone() | bb2;
        assert_eq!(bb1.get_trues(), bb3.get_trues());
    }

    #[test]
    fn case09() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = bb1.clone() >> 1;
        assert_eq!(bb1.to_u128() / bb2.to_u128(), 2);
    }

    #[test]
    fn case10() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = bb1.clone() << 1;
        assert_eq!(bb2.to_u128() / bb1.to_u128(), 2);
    }

    #[test]
    fn case11() {
        let column_nos = vec![1, 2, 3];
        let bb = generate_columns(column_nos, 3);
        assert_eq!(bb.board[100], true);
        assert_eq!(bb.board[101], true);
        assert_eq!(bb.board[102], true);
        assert_eq!(bb.get_trues().len(), 27);
    }

    #[test]
    fn case12() {
        let bb = generate_column(9);
        assert_eq!(bb.board[108], true);
        assert_eq!(bb.board[97], true);
        assert_eq!(bb.board[86], true);
        assert_eq!(bb.get_trues().len(), 9);
    }
    
}