#[cfg(test)]

mod tests {
    use crate::{
        address::Address,
        color::ColorType,
        moves::Move,
        piece::{Piece, PieceType},
    };
    use bitvec::prelude::*;

    #[test]
    fn test_moves_new() {
        let mv = Move::new();
        assert_eq!(mv.value.len(), 16);
    }

    #[test]
    fn test_moves_from_standart() {
        /* from   0d00011 0b00001011 */
        /* to     0d00092 0b01011100 */
        /* result 0d11787 0b0010111000001011 */
        let from = Address::from_number(11);
        let to = Address::from_number(92);
        let mv = Move::from_standart(from, to, false);
        let mut bits: BitVec<u16, Msb0> = bitvec![u16, Msb0; 0; 16];
        bits.store_be(11787);
        assert_eq!(mv.value, bits);
    }

    #[test]
    fn test_moves_from_drop() {
        /* piece  0b00001111 */
        /* to     0b00110100 */
        /* result 0b1001101001000111 */
        let piece = Piece::from(ColorType::White, PieceType::Lance);
        let to = Address::from_number(52);
        let mv = Move::from_drop(piece, to);
        let mut bits = bitvec![u16, Msb0; 0; 16];
        bits.store_be(-26041);
        assert_eq!(mv.value, bits);
    }

    #[test]
    fn test_moves_from_csa() {
        /* result 0b0000110000001100 */
        let csa = String::from("1a2b");
        let mv = Move::from_csa(csa);
        let mut bits = bitvec![u16, Msb0; 0; 16];
        bits.store_be(3084);
        assert_eq!(mv.value, bits);
    }

    #[test]
    fn test_moves_get_is_drop() {
        /* result 0b0000110000001100 */
        let csa = String::from("1a2b");
        let mv = Move::from_csa(csa);
        assert_eq!(mv.get_is_drop(), false);
    }

    #[test]
    fn test_moves_get_is_promote() {
        /* result 0b0000110000001100 */
        let csa = String::from("1a2b");
        let mv = Move::from_csa(csa);
        assert_eq!(mv.get_is_promote(), false);
    }

    #[test]
    fn test_moves_get_from() {
        /* result 0b0000110000001100 */
        let csa = String::from("1a2b");
        let mv = Move::from_csa(csa);
        assert_eq!(mv.get_from(), Address::from_number(12));
    }

    #[test]
    fn test_moves_get_to() {
        /* result 0b0000110000001100 */
        let csa = String::from("1a2b");
        let mv = Move::from_csa(csa);
        assert_eq!(mv.get_to(), Address::from_number(24));
    }

    #[test]
    fn test_moves_value_and_get_piece() {
        /* result 0b1000110001001000 */
        let csa = String::from("p*2b");
        let mv = Move::from_csa(csa);
        let mut bits = bitvec![u16, Msb0; 0; 16];
        bits.store_be(-29624);
        assert_eq!(mv.value, bits);
        assert_eq!(mv.get_piece(), Piece::from_char('p'));
    }

    #[test]
    fn test_moves_to_string() {
        let csa = String::from("p*2b");
        let mv = Move::from_csa(csa.clone());
        let str = mv.to_string();
        assert_eq!(str, csa);
    }
}
