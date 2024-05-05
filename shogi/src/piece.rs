use crate::color::ColorType;

use super::piece;
use super::direction;

// extern crate num;

#[allow(dead_code)]
pub const PROMOTE: u8 = 8;
#[allow(dead_code)]
pub const PIECE_TYPE_NUMBER: u8 = 15;
#[allow(dead_code)]
pub const NOT_PRO_PIECE_TYPE_NUMBER: u8 = 8;
#[allow(dead_code)]
pub const PROMOTE_CHANGE: u8 = 6;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
pub enum PieceType {
    None = 0, King, Gold,
    Rook, Bichop, Silver, Knight, Lance, Pawn,
    Dragon, Horse, ProSilver, PriKnight, ProLance, ProPawn,
}

impl PieceType {
    pub fn from_usize(n: usize) -> PieceType {
        match n {
            0 => PieceType::None,
            1 => PieceType::King,
            2 => PieceType::Gold,
            3 => PieceType::Rook,
            4 => PieceType::Bichop,
            5 => PieceType::Silver,
            6 => PieceType::Knight,
            7 => PieceType::Lance,
            8 => PieceType::Pawn,
            9 => PieceType::Dragon,
            10 => PieceType::Horse,
            11 => PieceType::ProSilver,
            12 => PieceType::PriKnight,
            13 => PieceType::ProLance,
            14 => PieceType::ProPawn,
            _ => PieceType::None,
        }
    }
}

#[allow(dead_code)]
pub enum HandPiece {
    HPawn, HLance, HKnight, HSilver, HGold, HBishop, HRook, HandPieceNum
}

#[allow(dead_code)]
pub enum MoveType {
    None, Short, Hop, Long
}

#[allow(dead_code)]
pub struct Piece {
    pub owner: ColorType,
    pub piece_type: PieceType,
}

impl Piece {
    #[allow(dead_code)]
    fn convert_string(piece_type: PieceType, owner: ColorType) -> String {
        let mut result = String::with_capacity(3);
        let mut piece_type_df: PieceType;
        if (piece_type as u8) > PROMOTE {
            result.push('+');
            piece_type_df = PieceType::from_usize(piece_type as usize - PROMOTE_CHANGE as usize)
        } else {
            piece_type_df = piece_type;
        }
        
        let mut piece: char;
        match piece_type_df {
            PieceType::None => piece = ' ',
            PieceType::King => piece = 'k',
            PieceType::Gold => piece = 'g',
            PieceType::Rook => piece = 'r',
            PieceType::Bichop => piece = 'b',
            PieceType::Silver => piece = 's',
            PieceType::Knight => piece = 'n',
            PieceType::Lance => piece = 'l',
            PieceType::Pawn => piece = 'p',
            _ => piece = ' ',
        }

        if owner == ColorType::Black {
            piece = (piece as u8 - 32) as char;
        }
        
        result.push(piece);
        result.push('\0');
        return result
    }

    #[allow(dead_code)]
    fn convert_from_string(&mut self, string: char) {
        let mut copied_string = string.clone();
        if string as u8 > 83 {
            self.owner = ColorType::White;
            copied_string = (copied_string as u8 - 32) as char;
        } else {
            self.owner = ColorType::Black;
        }

        match copied_string {
            'K' => self.piece_type = PieceType::King,
            'G' => self.piece_type = PieceType::Gold,
            'R' => self.piece_type = PieceType::Rook,
            'B' => self.piece_type = PieceType::Bichop,
            'S' => self.piece_type = PieceType::Silver,
            'N' => self.piece_type = PieceType::Knight,
            'L' => self.piece_type = PieceType::Lance,
            'P' => self.piece_type = PieceType::Pawn,
            _ => self.piece_type = PieceType::None
        }
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            owner: ColorType::None,
            piece_type: PieceType::None,
        }
    }

    #[allow(dead_code)]
    pub fn from_string(string: String) -> Self {
        let mut promote: u8 = 0;
        let mut piece_str;
        if string[0] == '+' {

        }
        let mut res = Self {
            
        }
    }
}