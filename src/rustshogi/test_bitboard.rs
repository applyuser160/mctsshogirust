#[cfg(test)]

mod tests {
    use crate::bitboard::{generate_column, generate_columns, BitBoard};

    #[test]
    fn test_bitboard_new() {
        let bitboard = BitBoard::new();
        assert_eq!((bitboard.0 >> (127 - 0)) & 1, 0);
    }

    #[test]
    fn test_bitboard_from_u128() {
        let bitboard = BitBoard::from_u128(548949983232);
        assert_eq!((bitboard.0 >> (127 - 88)) & 1, 0);
        assert_eq!((bitboard.0 >> (127 - 89)) & 1, 1);
        assert_eq!((bitboard.0 >> (127 - 90)) & 1, 1);
    }

    #[test]
    fn test_bitboard_from_str() {
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
        assert_eq!((bitboard.0 >> (127 - 88)) & 1, 0);
        assert_eq!((bitboard.0 >> (127 - 89)) & 1, 1);
        assert_eq!((bitboard.0 >> (127 - 90)) & 1, 1);
    }

    #[test]
    fn test_bitboard_to_u128() {
        let bitboard = BitBoard::from_u128(1124249833570304);
        let num = bitboard.to_u128();
        assert_eq!(num, 1124249833570304);
    }

    #[test]
    fn test_bitboard_get_trues() {
        let bitboard = BitBoard::from_u128(548949983232);
        let mut trues = bitboard.get_trues();
        trues.sort();
        assert_eq!(trues[0], 89);
        assert_eq!(trues.len(), 18);
    }

    #[test]
    fn test_bitboard_flip() {
        let mut bitboard = BitBoard::from_u128(548949983232);
        bitboard.flip();
        assert_eq!((bitboard.0 >> (127 - 88)) & 1, 1);
        assert_eq!((bitboard.0 >> (127 - 89)) & 1, 0);
        assert_eq!((bitboard.0 >> (127 - 90)) & 1, 0);
        assert_eq!(bitboard.get_trues().len(), 103);
    }

    #[test]
    fn test_bitboard_bitand() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = BitBoard::from_u128(548949983232);
        let bb3 = bb1 & bb2.clone();
        assert_eq!(bb2.get_trues(), bb3.get_trues());
    }

    #[test]
    fn test_bitboard_bitand_with_ref() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = BitBoard::from_u128(548949983232);
        let bb3 = &bb1 & &bb2;
        assert_eq!(bb2.get_trues(), bb3.get_trues());
    }

    #[test]
    fn test_bitboard_bitand_assign() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let mut bb2 = BitBoard::from_u128(548949983232);
        bb2 &= bb1;
        assert_eq!(bb2.to_u128(), 548949983232);
    }

    #[test]
    fn test_bitboard_bitand_assign_with_ref() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let mut bb2 = BitBoard::from_u128(548949983232);
        bb2 &= &bb1;
        assert_eq!(bb2.to_u128(), 548949983232);
    }

    #[test]
    fn test_bitboard_bitor() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = BitBoard::from_u128(548949983232);
        let bb3 = bb1.clone() | bb2;
        assert_eq!(bb1.get_trues(), bb3.get_trues());
    }

    #[test]
    fn test_bitboard_bitor_with_ref() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = BitBoard::from_u128(548949983232);
        let bb3 = &bb1 | &bb2;
        assert_eq!(bb1.get_trues(), bb3.get_trues());
    }

    #[test]
    fn test_bitboard_bitor_assign() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let mut bb2 = BitBoard::from_u128(548949983232);
        bb2 |= bb1;
        assert_eq!(bb2.to_u128(), 1124249833570304);
    }

    #[test]
    fn test_bitboard_bitor_assign_with_ref() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let mut bb2 = BitBoard::from_u128(548949983232);
        bb2 |= &bb1;
        assert_eq!(bb2.to_u128(), 1124249833570304);
    }

    #[test]
    fn test_bitboard_shiftright() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = bb1.clone() >> 1;
        assert_eq!(bb1.to_u128() / bb2.to_u128(), 2);
    }

    #[test]
    fn test_bitboard_shiftleft() {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = bb1.clone() << 1;
        assert_eq!(bb2.to_u128() / bb1.to_u128(), 2);
    }

    #[test]
    fn test_bitboard_shiftright_assign() {
        let mut bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = bb1.clone();
        bb1 >>= 1;
        assert_eq!(bb2.to_u128() / bb1.to_u128(), 2);
    }

    #[test]
    fn test_bitboard_shiftleft_assign() {
        let mut bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = bb1.clone();
        bb1 <<= 1;
        assert_eq!(bb1.to_u128() / bb2.to_u128(), 2);
    }

    #[test]
    fn test_bitboard_generate_columns_1() {
        let column_nos = vec![0, 1, 2];
        let bb = generate_columns(column_nos);
        assert_eq!((bb.0 >> (127 - 12)) & 1, 1); // col 1, row 1
        assert_eq!((bb.0 >> (127 - 13)) & 1, 1); // col 2, row 1
        assert_eq!((bb.0 >> (127 - 14)) & 1, 1); // col 3, row 1
        assert_eq!((bb.0 >> (127 - 102)) & 1, 1); // col 3, row 9
        assert_eq!(bb.get_trues().len(), 27);
    }

    #[test]
    fn test_bitboard_generate_columns_2() {
        let bb = generate_column(8); // Corresponds to the 9th column
        assert_eq!((bb.0 >> (127 - 20)) & 1, 1); // 9th col, 1st row
        assert_eq!((bb.0 >> (127 - 31)) & 1, 1); // 9th col, 2nd row
        assert_eq!((bb.0 >> (127 - 108)) & 1, 1); // 9th col, 9th row
        assert_eq!(bb.get_trues().len(), 9);
    }
}
