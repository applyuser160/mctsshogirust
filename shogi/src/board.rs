use super::moves;

use super::bitboard::{BitBoard, BIT_OF_FRAME, BIT_OF_PRO_ZONE_BLACK, BIT_OF_PRO_ZONE_WHITE,
    BIT_OF_LAST1_ZONE_BLACK, BIT_OF_LAST1_ZONE_WHITE, BIT_OF_LAST2_ZONE_BLACK, BIT_OF_LAST2_ZONE_WHITE};
use super::color::ColorType;
use super::hand::Hand;
use super::piece::{PieceType, PIECE_TYPE_NUMBER};
use crate::address::Address;

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
    #[allow(dead_code)]
    fn is_a_has_specific_piece(&self, index: u8, piece_type: PieceType) -> bool {
        return self.has_specific_piece[piece_type as usize].board[index as usize]
    }

    #[allow(dead_code)]
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
        has_specific_piece.board.iter_mut().for_each(|mut b| *b = !*b);
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

    #[allow(dead_code)]
    pub fn deploy(&mut self, index: u8, piece_type: PieceType, color: ColorType) {
        self.has_piece.board.set(index as usize, true);
        for i in 0..ColorType::ColorNumber as usize {
            self.player_prossesion[i].board.set(i, color == ColorType::from_u8(i as u8));
        }
        for i in 0..PIECE_TYPE_NUMBER as usize {
            self.has_specific_piece[i].board.set(i, piece_type == PieceType::from_usize(i));
        }
    }

    #[allow(dead_code)]
    pub fn startpos(&mut self) {
        self.deploy(Address::from_numbers(1, 1).to_index(), PieceType::Lance, ColorType::White);
        self.deploy(Address::from_numbers(2, 1).to_index(), PieceType::Knight, ColorType::White);
        self.deploy(Address::from_numbers(3, 1).to_index(), PieceType::Silver, ColorType::White);
        self.deploy(Address::from_numbers(4, 1).to_index(), PieceType::Gold, ColorType::White);
        self.deploy(Address::from_numbers(5, 1).to_index(), PieceType::King, ColorType::White);
        self.deploy(Address::from_numbers(6, 1).to_index(), PieceType::Gold, ColorType::White);
        self.deploy(Address::from_numbers(7, 1).to_index(), PieceType::Silver, ColorType::White);
        self.deploy(Address::from_numbers(8, 1).to_index(), PieceType::Knight, ColorType::White);
        self.deploy(Address::from_numbers(9, 1).to_index(), PieceType::Lance, ColorType::White);
        self.deploy(Address::from_numbers(2, 2).to_index(), PieceType::Bichop, ColorType::White);
        self.deploy(Address::from_numbers(8, 2).to_index(), PieceType::Rook, ColorType::White);
        self.deploy(Address::from_numbers(1, 3).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(2, 3).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(3, 3).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(4, 3).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(5, 3).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(6, 3).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(7, 3).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(8, 3).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(9, 3).to_index(), PieceType::Pawn, ColorType::White);

        self.deploy(Address::from_numbers(1, 9).to_index(), PieceType::Lance, ColorType::Black);
        self.deploy(Address::from_numbers(2, 9).to_index(), PieceType::Knight, ColorType::Black);
        self.deploy(Address::from_numbers(3, 9).to_index(), PieceType::Silver, ColorType::Black);
        self.deploy(Address::from_numbers(4, 9).to_index(), PieceType::Gold, ColorType::Black);
        self.deploy(Address::from_numbers(5, 9).to_index(), PieceType::King, ColorType::Black);
        self.deploy(Address::from_numbers(6, 9).to_index(), PieceType::Gold, ColorType::Black);
        self.deploy(Address::from_numbers(7, 9).to_index(), PieceType::Silver, ColorType::Black);
        self.deploy(Address::from_numbers(8, 9).to_index(), PieceType::Knight, ColorType::Black);
        self.deploy(Address::from_numbers(9, 9).to_index(), PieceType::Lance, ColorType::Black);
        self.deploy(Address::from_numbers(8, 8).to_index(), PieceType::Bichop, ColorType::Black);
        self.deploy(Address::from_numbers(2, 8).to_index(), PieceType::Rook, ColorType::Black);
        self.deploy(Address::from_numbers(1, 7).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(2, 7).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(3, 7).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(4, 7).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(5, 7).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(6, 7).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(7, 7).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(8, 7).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(9, 7).to_index(), PieceType::Pawn, ColorType::Black);

        self.hand = Hand::new();
    }

    #[allow(dead_code)]
    pub fn get_piece_type_from_index(&self, index: u8) -> PieceType {
        for i in 0..PIECE_TYPE_NUMBER as usize {
            if self.is_a_has_specific_piece(index, PieceType::from_usize(i)) {
                return PieceType::from_usize(i)
            }
        }
        return PieceType::None
    }

    #[allow(dead_code)]
    pub fn get_color_type_from_index(&self, index: u8) -> ColorType {
        let has_a_piece = self.has_piece.board[index as usize];
        let is_black = self.player_prossesion[ColorType::Black as usize].board[index as usize];
        if !has_a_piece {
            return ColorType::None
        } else if is_black {
            return ColorType::Black
        } else {
            return ColorType::White
        }
    }
}