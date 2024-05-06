use super::piece;
use super::color;

#[allow(dead_code)]
pub struct Hand {
    pub pieces: [piece::Piece; (piece::NOT_PRO_PIECE_TYPE_NUMBER * color::ColorType::ColorNumber as u8) as usize],
    pub counts: [u8; (piece::NOT_PRO_PIECE_TYPE_NUMBER * color::ColorType::ColorNumber as u8) as usize],
}

impl Hand {
    #[allow(dead_code)]
    fn calc_index(color_type: color::ColorType, piece_type: piece::PieceType) -> u16 {
        return color_type as u16 * piece::NOT_PRO_PIECE_TYPE_NUMBER as u16 + piece_type as u16 - 1
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut res = Self {
            pieces: [piece::Piece::new(); (piece::NOT_PRO_PIECE_TYPE_NUMBER * color::ColorType::ColorNumber as u8) as usize],
            counts: [0; (piece::NOT_PRO_PIECE_TYPE_NUMBER * color::ColorType::ColorNumber as u8) as usize],
        };
        for j in color::ColorType::Black as usize..color::ColorType::ColorNumber as usize {
            for i in piece::PieceType::King as usize..piece::NOT_PRO_PIECE_TYPE_NUMBER as usize {
                let index = j * piece::NOT_PRO_PIECE_TYPE_NUMBER as usize + i - 1;
                res.pieces[index] = piece::Piece::from(
                    color::ColorType::from_u8(j as u8), piece::PieceType::from_usize(i));
                res.counts[index] = 0;
            }
        }
        return res
    }

    #[allow(dead_code)]
    pub fn get_piece(&self, color_type: color::ColorType, piece_type: piece::PieceType) -> piece::Piece {
        let index = Self::calc_index(color_type, piece_type);
        return self.pieces[index as usize]
    }


}