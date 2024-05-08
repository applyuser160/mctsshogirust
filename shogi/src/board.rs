use super::address;
use super::moves;

use super::bitboard::{BitBoard, BIT_OF_FRAME, BIT_OF_PRO_ZONE_BLACK, BIT_OF_PRO_ZONE_WHITE,
    BIT_OF_LAST1_ZONE_BLACK, BIT_OF_LAST1_ZONE_WHITE, BIT_OF_LAST2_ZONE_BLACK, BIT_OF_LAST2_ZONE_WHITE};
use super::color::ColorType;
use super::hand::Hand;
use super::piece::{PieceType, PIECE_TYPE_NUMBER};

#[allow(dead_code)]
pub struct Board {
    pub has_piece: BitBoard,
    pub player_prossesion: [BitBoard; ColorType::ColorNumber as usize],
    pub is_frame: BitBoard,
    pub able_pro: [BitBoard; ColorType::ColorNumber as usize],
    pub last_one: [BitBoard; ColorType::ColorNumber as usize],
    pub last_two: [BitBoard; ColorType::ColorNumber as usize],
    pub has_specific_piece: [BitBoard; PIECE_TYPE_NUMBER as usize],
    pub hand: Hand,
}

impl Board {
    fn is_a_has_specific_piece(&self, index: u8, piece_type: PieceType) -> bool {
        return self.has_specific_piece[piece_type as usize].board[index as usize]
    }

    fn drop(&mut self, index: u8) {
        self.has_piece.board.set(index as usize, false);
        for i in 0..ColorType::ColorNumber as usize {
            self.player_prossesion[i].board.set(index as usize, false);
        }
        self.has_specific_piece[PieceType::None as usize].board.set(index as usize, true);
        for i in 1..PIECE_TYPE_NUMBER as usize {
            self.has_specific_piece[i].board.set(index as usize, false);
        }
    }

    // fn move_standard(&mut self, from_index: u8, to_index: u8) {
    //     let piece_type = 
    // }

    pub fn new() -> Self {
        let mut has_specific_piece = BitBoard::from_u128(BIT_OF_FRAME);
        Self {
            has_piece: BitBoard::new(),
            player_prossesion: [BitBoard::new(), BitBoard::new()],
            is_frame: BitBoard::new(),
            able_pro: [BitBoard::from_u128(BIT_OF_PRO_ZONE_BLACK),
                BitBoard::from_u128(BIT_OF_PRO_ZONE_WHITE),],
            last_one: [BitBoard::from_u128(BIT_OF_LAST1_ZONE_BLACK),
                BitBoard::from_u128(BIT_OF_LAST1_ZONE_WHITE),],
            last_two: [BitBoard::from_u128(BIT_OF_LAST2_ZONE_BLACK),
                BitBoard::from_u128(BIT_OF_LAST2_ZONE_WHITE),],
            has_specific_piece: [has_specific_piece, BitBoard::new(), BitBoard::new(),
                BitBoard::new(), BitBoard::new(), BitBoard::new(), BitBoard::new(),
                BitBoard::new(), BitBoard::new(), BitBoard::new(), BitBoard::new(),
                BitBoard::new(), BitBoard::new(), BitBoard::new(), BitBoard::new(),],
            hand: Hand::new(),
        }
    }
}