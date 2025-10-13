use crate::address::Address;
use crate::piece::Piece;

use super::address;
use super::piece;

use pyo3::prelude::*;

#[allow(dead_code)]
#[pyclass]
#[derive(Clone, Debug)]
pub struct Move {
    pub value: u16,
}

impl Move {
    #[allow(dead_code)]
    fn is_drop(csa: &str) -> bool {
        csa.chars().nth(1) == Some('*')
    }

    #[allow(dead_code)]
    fn is_promote(csa: &str) -> bool {
        if csa.len() > 4 {
            csa.chars().nth(4) == Some('+')
        } else {
            false
        }
    }

    fn base_constructor(&mut self, from: u16, to: u16, pro: u16, drop: u16) {
        // ビット配置:
        // bit 15: drop (1 bit)
        // bit 14: promote (1 bit)
        // bits 13-7: to (7 bits)
        // bits 6-0: from/piece (7 bits)
        self.value = (drop << 15) | (pro << 14) | (to << 7) | from;
    }

    #[allow(dead_code)]
    fn standart_constructor(
        &mut self,
        from: address::Address,
        to: address::Address,
        promote: bool,
    ) {
        self.base_constructor(
            from.to_index() as u16,
            to.to_index() as u16,
            promote as u16,
            0,
        );
    }

    #[allow(dead_code)]
    fn drop_constructor(&mut self, piece: piece::Piece, to: address::Address) {
        self.base_constructor(piece.to_u8() as u16, to.to_index() as u16, 0, 1)
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { value: 0 }
    }

    #[allow(dead_code)]
    pub fn from_standart(from: address::Address, to: address::Address, promote: bool) -> Self {
        let mut res = Self::new();
        res.standart_constructor(from, to, promote);
        res
    }

    #[allow(dead_code)]
    pub fn from_drop(piece: piece::Piece, to: address::Address) -> Self {
        let mut res = Self::new();
        res.drop_constructor(piece, to);
        res
    }

    #[allow(dead_code)]
    pub fn from_csa(csa: &str) -> Self {
        let mut res = Self::new();
        let to = address::Address::from_string(&csa[2..]);
        if Self::is_drop(csa) {
            let piece = piece::Piece::from_char(csa.chars().nth(0).unwrap());
            res.drop_constructor(piece, to);
        } else {
            let from = address::Address::from_string(csa);
            res.standart_constructor(from, to, Self::is_promote(csa));
        }
        res
    }

    #[allow(dead_code)]
    pub fn get_is_drop(&self) -> bool {
        (self.value & (1 << 15)) != 0
    }

    #[allow(dead_code)]
    pub fn get_is_promote(&self) -> bool {
        (self.value & (1 << 14)) != 0
    }

    #[allow(dead_code)]
    pub fn get_from(&self) -> address::Address {
        // bits 6-0: from (7 bits)
        let v = (self.value & 0x7F) as u8;
        address::Address::from_number(v)
    }

    #[allow(dead_code)]
    pub fn get_to(&self) -> address::Address {
        // bits 13-7: to (7 bits)
        let v = ((self.value >> 7) & 0x7F) as u8;
        address::Address::from_number(v)
    }

    #[allow(dead_code)]
    pub fn get_piece(&self) -> piece::Piece {
        // ドロップ手の場合、piece情報は下位7ビット（bits 0-6）に格納
        let v = (self.value & 0x7F) as u8;
        piece::Piece::from_u8(v)
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut first = String::with_capacity(2);
        if self.get_is_drop() {
            let piece = self.get_piece();
            first.push_str(&piece.to_string());
            first.push('*');
        } else {
            let from = self.get_from();
            first.push_str(&from.to_string());
        }
        let to = self.get_to();
        let is_pro = self.get_is_promote();
        let mut res = String::with_capacity(5);
        res.push_str(&first);
        res.push_str(&to.to_string());
        if is_pro {
            res.push('+');
        }
        res
    }
}

#[pymethods]
impl Move {
    #[new]
    #[pyo3(signature = (csa = None, from_address = None, to_address = None, promote = false, piece = None))]
    pub fn new_for_python(
        csa: Option<String>,
        from_address: Option<Address>,
        to_address: Option<Address>,
        promote: bool,
        piece: Option<Piece>,
    ) -> Self {
        if let Some(csa) = csa {
            Self::from_csa(csa.as_str())
        } else if let Some(from) = from_address {
            Self::from_standart(from, to_address.unwrap(), promote)
        } else if let Some(piece) = piece {
            Self::from_drop(piece, to_address.unwrap())
        } else {
            Self::new()
        }
    }

    pub fn __repr__(&self) -> String {
        format!("Move(csa={})", self.to_string())
    }

    pub fn __str__(&self) -> String {
        format!("Move(csa={})", self.to_string())
    }

    pub fn __eq__(&self, other: &Self) -> bool {
        self.value == other.value
    }

    pub fn __ne__(&self, other: &Self) -> bool {
        self.value != other.value
    }

    #[allow(dead_code)]
    #[pyo3(name = "is_drop")]
    pub fn python_is_drop(&self) -> bool {
        self.get_is_drop()
    }

    #[allow(dead_code)]
    #[pyo3(name = "is_promote")]
    pub fn python_is_promote(&self) -> bool {
        self.get_is_promote()
    }

    #[allow(dead_code)]
    #[pyo3(name = "get_from")]
    pub fn python_get_from(&self) -> address::Address {
        self.get_from()
    }

    #[allow(dead_code)]
    #[pyo3(name = "get_to")]
    pub fn python_get_to(&self) -> address::Address {
        self.get_to()
    }

    #[allow(dead_code)]
    #[pyo3(name = "get_piece")]
    pub fn python_get_piece(&self) -> piece::Piece {
        self.get_piece()
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
