use crate::color::ColorType;
use bitvec::prelude::*;

use super::direction;

use strum_macros::EnumIter;

use pyo3::prelude::*;

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
#[pyclass(eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, EnumIter)]
#[repr(usize)]
pub enum PieceType {
    None = 0,
    King,
    Gold,
    Rook,
    Bichop,
    Silver,
    Knight,
    Lance,
    Pawn,
    Dragon,
    Horse,
    ProSilver,
    ProKnight,
    ProLance,
    ProPawn,
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
            12 => PieceType::ProKnight,
            13 => PieceType::ProLance,
            14 => PieceType::ProPawn,
            _ => PieceType::None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PieceType::None => String::from(' '),
            PieceType::King => String::from('k'),
            PieceType::Gold => String::from('g'),
            PieceType::Rook => String::from('r'),
            PieceType::Bichop => String::from('b'),
            PieceType::Silver => String::from('s'),
            PieceType::Knight => String::from('n'),
            PieceType::Lance => String::from('l'),
            PieceType::Pawn => String::from('p'),
            _ => String::from(' '),
        }
    }
}

#[pymethods]
impl PieceType {
    #[new]
    pub fn new(value: u8) -> Self {
        Self::from_usize(value as usize)
    }

    pub fn __ptr__(&self) -> String {
        format!("rustshogi.PieceType({})", *self as u8)
    }
}

#[allow(dead_code)]
pub enum HandPiece {
    HPawn,
    HLance,
    HKnight,
    HSilver,
    HGold,
    HBishop,
    HRook,
    HandPieceNum,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MoveType {
    None,
    Short,
    Hop,
    Long,
}

#[allow(dead_code)]
#[pyclass]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Piece {
    #[pyo3(get, set)]
    pub owner: ColorType,
    #[pyo3(get, set)]
    pub piece_type: PieceType,
}

impl Default for Piece {
    fn default() -> Self {
        Self::new()
    }
}

impl Piece {
    #[allow(dead_code)]
    pub fn convert_string(piece_type: PieceType, owner: ColorType) -> String {
        let mut result = String::with_capacity(3);
        let piece_type_df: PieceType;
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
            piece = piece.to_ascii_uppercase();
        }

        result.push(piece);
        result
    }

    #[allow(dead_code)]
    fn convert_from_string(&mut self, character: char) {
        let mut character = character;
        if character as u8 > 83 {
            self.owner = ColorType::White;
            character = character.to_ascii_uppercase();
        } else {
            self.owner = ColorType::Black;
        }

        match character {
            'K' => self.piece_type = PieceType::King,
            'G' => self.piece_type = PieceType::Gold,
            'R' => self.piece_type = PieceType::Rook,
            'B' => self.piece_type = PieceType::Bichop,
            'S' => self.piece_type = PieceType::Silver,
            'N' => self.piece_type = PieceType::Knight,
            'L' => self.piece_type = PieceType::Lance,
            'P' => self.piece_type = PieceType::Pawn,
            _ => self.piece_type = PieceType::None,
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
    pub fn from(color_type: ColorType, piece_type: PieceType) -> Self {
        Self {
            owner: color_type,
            piece_type,
        }
    }

    #[allow(dead_code)]
    pub fn from_u8(num: u8) -> Self {
        let mut bits = bitvec![u8, Msb0; 0; 8];
        bits.store_be::<u8>(num);
        let owner = ColorType::from_u8(bits[1] as u8);
        let mut piece_type: u8 = 0;
        let base: u8 = 2;
        for i in 2..8 {
            if bits[i] {
                piece_type += base.pow((7 - i) as u32);
            }
        }
        Self {
            owner,
            piece_type: PieceType::from_usize(piece_type as usize),
        }
    }

    #[allow(dead_code)]
    pub fn from_char(character: char) -> Self {
        let mut res = Self::new();
        res.convert_from_string(character);
        res
    }

    #[allow(dead_code)]
    pub fn from_string(str: &str) -> Self {
        let mut promote: u8 = 0;
        let piece_str: char;
        if str.chars().nth(0) == Some('+') {
            promote = PROMOTE_CHANGE;
            piece_str = str.chars().nth(1).unwrap();
        } else {
            piece_str = str.chars().nth(0).unwrap();
        }

        let mut res = Self::new();
        res.convert_from_string(piece_str);
        res.piece_type = PieceType::from_usize(res.piece_type as usize + promote as usize);
        res
    }

    #[allow(dead_code)]
    pub fn to_u8(&self) -> u8 {
        let mut res = self.piece_type as u8;
        res += (self.owner as u8) << 6;
        res
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        Self::convert_string(self.piece_type, self.owner)
    }

    #[allow(dead_code)]
    pub fn get_movetype(
        piece_type: PieceType,
    ) -> [MoveType; direction::DirectionName::DirectionNameNumber as usize] {
        match piece_type {
            PieceType::None => [
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
            ],
            PieceType::King => [
                MoveType::Short,
                MoveType::Short,
                MoveType::Short,
                MoveType::Short,
                MoveType::Short,
                MoveType::Short,
                MoveType::Short,
                MoveType::Short,
            ],
            PieceType::Gold => [
                MoveType::Short,
                MoveType::Short,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
                MoveType::Short,
            ],
            PieceType::Rook => [
                MoveType::Long,
                MoveType::None,
                MoveType::Long,
                MoveType::None,
                MoveType::Long,
                MoveType::None,
                MoveType::Long,
                MoveType::None,
            ],
            PieceType::Bichop => [
                MoveType::None,
                MoveType::Long,
                MoveType::None,
                MoveType::Long,
                MoveType::None,
                MoveType::Long,
                MoveType::None,
                MoveType::Long,
            ],
            PieceType::Silver => [
                MoveType::Short,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
            ],
            PieceType::Knight => [
                MoveType::None,
                MoveType::Hop,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::Hop,
            ],
            PieceType::Lance => [
                MoveType::Long,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
            ],
            PieceType::Pawn => [
                MoveType::Short,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
                MoveType::None,
            ],
            PieceType::Dragon => [
                MoveType::Long,
                MoveType::Short,
                MoveType::Long,
                MoveType::Short,
                MoveType::Long,
                MoveType::Short,
                MoveType::Long,
                MoveType::Short,
            ],
            PieceType::Horse => [
                MoveType::Short,
                MoveType::Long,
                MoveType::Short,
                MoveType::Long,
                MoveType::Short,
                MoveType::Long,
                MoveType::Short,
                MoveType::Long,
            ],
            PieceType::ProSilver => [
                MoveType::Short,
                MoveType::Short,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
                MoveType::Short,
            ],
            PieceType::ProKnight => [
                MoveType::Short,
                MoveType::Short,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
                MoveType::Short,
            ],
            PieceType::ProLance => [
                MoveType::Short,
                MoveType::Short,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
                MoveType::Short,
            ],
            PieceType::ProPawn => [
                MoveType::Short,
                MoveType::Short,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
                MoveType::None,
                MoveType::Short,
                MoveType::Short,
            ],
        }
    }

    #[allow(dead_code)]
    pub fn able_pro(piece_type: PieceType) -> bool {
        match piece_type {
            PieceType::Rook => true,
            PieceType::Bichop => true,
            PieceType::Silver => true,
            PieceType::Knight => true,
            PieceType::Lance => true,
            PieceType::Pawn => true,
            _ => false,
        }
    }
}

#[pymethods]
impl Piece {
    #[new]
    pub fn new_for_python(owner: ColorType, piece_type: PieceType) -> Self {
        Self { owner, piece_type }
    }

    pub fn __ptr__(&self) -> String {
        format!("rustshogi.Piece({})", self.to_string())
    }
}

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
