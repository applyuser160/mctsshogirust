#[cfg(test)]

mod tests {
    use crate::{color::ColorType, piece::{Piece, PieceType}};


    #[test]
    fn case01() {
        let piece_type = PieceType::from_usize(1);
        assert_eq!(piece_type as u8, 1);
    }

    #[test]
    fn case02() {
        let piece_type = PieceType::from_usize(1);
        assert_eq!(piece_type.to_string(), String::from('k'));
    }

    #[test]
    fn case03() {
        let piece = Piece::new();
        assert_eq!(piece.owner, ColorType::None);
        assert_eq!(piece.piece_type, PieceType::None);
    }

    #[test]
    fn case04() {
        let piece = Piece::from(ColorType::Black, PieceType::Dragon);
        assert_eq!(piece.owner, ColorType::Black);
        assert_eq!(piece.piece_type, PieceType::Dragon);
    }
    
}