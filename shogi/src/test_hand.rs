#[cfg(test)]

mod tests {
    use crate::{
        color::ColorType,
        hand::Hand,
        piece::{Piece, PieceType},
    };

    #[test]
    fn case01() {
        let hand = Hand::new();
        assert_eq!(hand.pieces.len(), 16);
        assert_eq!(hand.counts.len(), 16);
    }

    #[test]
    fn case02() {
        let hand = Hand::new();
        let piece = hand.get_piece(ColorType::Black, PieceType::Gold);
        assert_eq!(piece, Piece::from_char('G'));
    }

    #[test]
    fn case03() {
        let mut hand = Hand::new();
        hand.add_piece(ColorType::Black, PieceType::Gold);
        let count = hand.get_count(ColorType::Black, PieceType::Gold);
        assert_eq!(count, 1);
    }

    #[test]
    fn case04() {
        let mut hand = Hand::new();
        hand.add_pieces(ColorType::White, PieceType::Pawn, 4);
        let count = hand.get_count(ColorType::White, PieceType::Pawn);
        assert_eq!(count, 4);
    }

    #[test]
    fn case05() {
        let mut hand = Hand::new();
        hand.add_pieces(ColorType::Black, PieceType::Knight, 2);
        hand.decrease_piece(ColorType::Black, PieceType::Knight);
        let count = hand.get_count(ColorType::Black, PieceType::Knight);
        assert_eq!(count, 1);
    }

    #[test]
    fn case06() {
        let mut hand = Hand::new();
        hand.add_pieces(ColorType::Black, PieceType::Knight, 2);
        hand.add_pieces(ColorType::Black, PieceType::Pawn, 9);
        hand.add_pieces(ColorType::White, PieceType::Gold, 3);
        let blacks = hand.get_player_pieces(ColorType::Black);
        let whites = hand.get_player_pieces(ColorType::White);
        assert_eq!(blacks.len(), 2);
        assert_eq!(whites.len(), 1);
        assert_eq!(blacks[0].piece_type, PieceType::Knight);
        assert_eq!(blacks[0].owner, ColorType::Black);
        assert_eq!(blacks[1].piece_type, PieceType::Pawn);
        assert_eq!(blacks[1].owner, ColorType::Black);
        assert_eq!(whites[0].piece_type, PieceType::Gold);
        assert_eq!(whites[0].owner, ColorType::White);
    }
}
