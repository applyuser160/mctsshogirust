use super::color;
use super::piece;

use pyo3::prelude::*;

#[allow(dead_code)]
#[pyclass]
#[derive(Clone, PartialEq, Debug)]
pub struct Hand {
    #[pyo3(get, set)]
    pub pieces: [piece::Piece;
        (piece::NOT_PRO_PIECE_TYPE_NUMBER * color::ColorType::ColorNumber as u8) as usize],
    #[pyo3(get, set)]
    pub counts:
        [u8; (piece::NOT_PRO_PIECE_TYPE_NUMBER * color::ColorType::ColorNumber as u8) as usize],
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

impl Hand {
    #[allow(dead_code)]
    fn calc_index(color_type: color::ColorType, piece_type: piece::PieceType) -> u16 {
        color_type as u16 * piece::NOT_PRO_PIECE_TYPE_NUMBER as u16 + piece_type as u16 - 1
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut res = Self {
            pieces: [piece::Piece::new();
                (piece::NOT_PRO_PIECE_TYPE_NUMBER * color::ColorType::ColorNumber as u8) as usize],
            counts: [0; (piece::NOT_PRO_PIECE_TYPE_NUMBER * color::ColorType::ColorNumber as u8)
                as usize],
        };
        for j in color::ColorType::Black as usize..color::ColorType::ColorNumber as usize {
            for i in piece::PieceType::King as usize..=piece::NOT_PRO_PIECE_TYPE_NUMBER as usize {
                let index = j * piece::NOT_PRO_PIECE_TYPE_NUMBER as usize + i - 1;
                res.pieces[index] = piece::Piece::from(
                    color::ColorType::from_u8(j as u8),
                    piece::PieceType::from_usize(i),
                );
                res.counts[index] = 0;
            }
        }
        res
    }

    #[allow(dead_code)]
    pub fn get_piece(
        &self,
        color_type: color::ColorType,
        piece_type: piece::PieceType,
    ) -> piece::Piece {
        let index = Self::calc_index(color_type, piece_type);
        self.pieces[index as usize]
    }

    #[allow(dead_code)]
    pub fn get_count(&self, color_type: color::ColorType, piece_type: piece::PieceType) -> u8 {
        let index = Self::calc_index(color_type, piece_type);
        self.counts[index as usize]
    }

    #[allow(dead_code)]
    pub fn add_piece(&mut self, color_type: color::ColorType, piece_type: piece::PieceType) {
        let index = Self::calc_index(color_type, piece_type);
        self.counts[index as usize] += 1;
    }

    #[allow(dead_code)]
    pub fn add_pieces(
        &mut self,
        color_type: color::ColorType,
        piece_type: piece::PieceType,
        count: u8,
    ) {
        let index = Self::calc_index(color_type, piece_type);
        self.counts[index as usize] += count;
    }

    #[allow(dead_code)]
    pub fn decrease_piece(&mut self, color_type: color::ColorType, piece_type: piece::PieceType) {
        let index = Self::calc_index(color_type, piece_type);
        self.counts[index as usize] -= 1;
    }

    #[allow(dead_code)]
    pub fn get_player_pieces(&self, color_type: color::ColorType) -> Vec<piece::Piece> {
        let mut res: Vec<piece::Piece> = Vec::new();
        for i in piece::PieceType::King as usize..=piece::NOT_PRO_PIECE_TYPE_NUMBER as usize {
            let index = Self::calc_index(color_type, piece::PieceType::from_usize(i));
            if self.counts[index as usize] > 0 {
                res.push(self.pieces[index as usize]);
            }
        }
        res
    }
}

#[pymethods]
impl Hand {
    #[new]
    pub fn new_for_python() -> Self {
        Self::new()
    }

    pub fn __ptr__(&self) -> String {
        format!(
            "rustshogi.Hand(pieces={:?}, counts={:?})",
            self.pieces, self.counts
        )
    }
}
