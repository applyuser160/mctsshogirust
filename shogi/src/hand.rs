use super::piece;
use super::color;

#[allow(dead_code)]
pub struct Hand {
    pub pieces: [piece::Piece; (piece::NOT_PRO_PIECE_TYPE_NUMBER * color::ColorType::ColorNumber as u8) as usize],
    pub counts: [u8; (piece::NOT_PRO_PIECE_TYPE_NUMBER * color::ColorType::ColorNumber as u8) as usize],
}

impl Hand {
    
}