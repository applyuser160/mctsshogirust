#[cfg(test)]

mod tests {
    use crate::{
        color::ColorType,
        piece::{MoveType, Piece, PieceType},
    };

    #[test]
    fn test_piece_from_usize() {
        let piece_type = PieceType::from_usize(1);
        assert_eq!(piece_type as u8, 1);
    }

    #[test]
    fn test_piece_to_string() {
        let piece_type = PieceType::from_usize(1);
        assert_eq!(piece_type.to_string(), String::from('k'));
    }

    #[test]
    fn test_piece_new() {
        let piece = Piece::new();
        assert_eq!(piece.owner, ColorType::None);
        assert_eq!(piece.piece_type, PieceType::None);
    }

    #[test]
    fn test_piece_from() {
        let piece = Piece::from(ColorType::Black, PieceType::Dragon);
        assert_eq!(piece.owner, ColorType::Black);
        assert_eq!(piece.piece_type, PieceType::Dragon);
    }

    #[test]
    fn case05() {
        /* 0d142 = 0b01001110 */
        let piece = Piece::from_u8(78);
        assert_eq!(piece.owner, ColorType::White);
        assert_eq!(piece.piece_type, PieceType::ProPawn);
    }

    #[test]
    fn case06() {
        /* 0d142 = 0b11111111 */
        let piece = Piece::from_u8(255);
        assert_eq!(piece.owner, ColorType::White);
        assert_eq!(piece.piece_type, PieceType::None);
    }

    #[test]
    fn case07() {
        let piece = Piece::from_char('K');
        assert_eq!(piece.owner, ColorType::Black);
        assert_eq!(piece.piece_type, PieceType::King);
    }

    #[test]
    fn case07d() {
        let piece = Piece::from_char('p');
        assert_eq!(piece.owner, ColorType::White);
        assert_eq!(piece.piece_type, PieceType::Pawn);
    }

    #[test]
    fn case08() {
        let piece = Piece::from_string(String::from("+p"));
        assert_eq!(piece.owner, ColorType::White);
        assert_eq!(piece.piece_type, PieceType::ProPawn);
    }

    #[test]
    fn case09() {
        let piece = Piece::from_string(String::from("B"));
        assert_eq!(piece.owner, ColorType::Black);
        assert_eq!(piece.piece_type, PieceType::Bichop);
    }

    #[test]
    fn case10() {
        let piece = Piece::from(ColorType::Black, PieceType::Dragon);
        assert_eq!(piece.to_u8(), 9);
        assert_eq!(piece.to_string(), String::from("+R"));
    }

    #[test]
    fn case11() {
        let result = Piece::get_movetype(PieceType::King);
        assert_eq!(result[0], MoveType::Short);
        assert_eq!(result[1], MoveType::Short);
        assert_eq!(result[2], MoveType::Short);
        assert_eq!(result[3], MoveType::Short);
        assert_eq!(result[4], MoveType::Short);
        assert_eq!(result[5], MoveType::Short);
        assert_eq!(result[6], MoveType::Short);
        assert_eq!(result[7], MoveType::Short);
    }

    #[test]
    fn case12() {
        let result = Piece::get_movetype(PieceType::Bichop);
        assert_eq!(result[0], MoveType::None);
        assert_eq!(result[1], MoveType::Long);
        assert_eq!(result[2], MoveType::None);
        assert_eq!(result[3], MoveType::Long);
        assert_eq!(result[4], MoveType::None);
        assert_eq!(result[5], MoveType::Long);
        assert_eq!(result[6], MoveType::None);
        assert_eq!(result[7], MoveType::Long);
    }

    #[test]
    fn case13() {
        let result = Piece::able_pro(PieceType::Gold);
        assert_eq!(result, false);
    }

    #[test]
    fn case14() {
        let result = Piece::able_pro(PieceType::Bichop);
        assert_eq!(result, true);
    }
}
