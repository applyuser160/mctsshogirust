#[cfg(test)]
mod tests {
    use crate::bitboard::BitBoard;

    #[test]
    fn test_bitboard_new() {
        let bitboard = BitBoard::new();
        assert_eq!(bitboard.get(0), false);
    }

    #[test]
    fn test_bitboard_from_u128() {
        let bitboard = BitBoard::from_u128(1 << 30 | 1 << 31);
        assert_eq!(bitboard.get(88), false);
        assert_eq!(bitboard.get(89), true);
        assert_eq!(bitboard.get(90), true);
    }

    #[test]
    fn test_bitboard_to_u128() {
        let bitboard = BitBoard::from_u128(1 << 30 | 1 << 31);
        assert_eq!(bitboard.to_u128(), 1 << 30 | 1 << 31);
    }

    #[test]
    fn test_bitboard_from_str() {
        let string = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000011000000000000000000000000000000";
        let bitboard = BitBoard::from_str(string);
        assert_eq!(bitboard.get(88), false);
        assert_eq!(bitboard.get(89), true);
        assert_eq!(bitboard.get(90), true);
    }

    #[test]
    fn test_bitboard_get_trues() {
        let bitboard = BitBoard::from_u128(1 << 30 | 1 << 31);
        let mut trues = bitboard.get_trues();
        trues.sort();
        assert_eq!(trues, vec![89, 90]);
    }

    #[test]
    fn test_bitboard_not() {
        let mut bitboard = BitBoard::from_u128(1 << 30 | 1 << 31);
        bitboard = !bitboard;
        assert_eq!(bitboard.get(88), true);
        assert_eq!(bitboard.get(89), false);
        assert_eq!(bitboard.get(90), false);
    }

    #[test]
    fn test_bitboard_bitand() {
        let bb1 = BitBoard::from_u128(1 << 30 | 1 << 31);
        let bb2 = BitBoard::from_u128(1 << 31);
        let bb3 = bb1 & bb2;
        assert_eq!(bb3.get(89), true);
        assert_eq!(bb3.get(90), false);
    }

    #[test]
    fn test_bitboard_bitand_assign() {
        let mut bb1 = BitBoard::from_u128(1 << 30 | 1 << 31);
        let bb2 = BitBoard::from_u128(1 << 31);
        bb1 &= bb2;
        assert_eq!(bb1.get(89), true);
        assert_eq!(bb1.get(90), false);
    }

    #[test]
    fn test_bitboard_bitor() {
        let bb1 = BitBoard::from_u128(1 << 30);
        let bb2 = BitBoard::from_u128(1 << 31);
        let bb3 = bb1 | bb2;
        assert_eq!(bb3.get(89), true);
        assert_eq!(bb3.get(90), true);
    }

    #[test]
    fn test_bitboard_bitor_assign() {
        let mut bb1 = BitBoard::from_u128(1 << 30);
        let bb2 = BitBoard::from_u128(1 << 31);
        bb1 |= bb2;
        assert_eq!(bb1.get(89), true);
        assert_eq!(bb1.get(90), true);
    }

    #[test]
    fn test_bitboard_shr() {
        let bb1 = BitBoard::from_u128(1 << 31);
        let bb2 = bb1 >> 1;
        assert_eq!(bb2.get(89), false);
        assert_eq!(bb2.get(90), true);
    }

    #[test]
    fn test_bitboard_shl() {
        let bb1 = BitBoard::from_u128(1 << 30);
        let bb2 = bb1 << 1;
        assert_eq!(bb2.get(89), true);
        assert_eq!(bb2.get(90), false);
    }

    #[test]
    fn test_bitboard_shr_assign() {
        let mut bb = BitBoard::from_u128(1 << 31);
        bb >>= 1;
        assert_eq!(bb.get(89), false);
        assert_eq!(bb.get(90), true);
    }

    #[test]
    fn test_bitboard_shl_assign() {
        let mut bb = BitBoard::from_u128(1 << 30);
        bb <<= 1;
        assert_eq!(bb.get(89), true);
        assert_eq!(bb.get(90), false);
    }

    #[test]
    fn test_generate_column() {
        let bb = crate::bitboard::generate_column(1);
        let mut trues = bb.get_trues();
        trues.sort();
        assert_eq!(trues, vec![12, 23, 34, 45, 56, 67, 78, 89, 100]);
    }
}
