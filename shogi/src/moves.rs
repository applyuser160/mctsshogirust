use std::ops::BitOr;

use bitvec::prelude::*;

use bitvec::prelude::BitVec;
use bitvec::order::Lsb0;

use super::address;
use super::piece;
use super::color;

#[allow(dead_code)]
pub struct Move {
    pub value: BitVec<u16, Msb0>,
}

impl Move {
    #[allow(dead_code)]
    fn is_drop(csa: String) -> bool {
        let csa_vec = csa.chars().collect::<Vec<char>>();
        return csa_vec[1] == '*'
    }

    #[allow(dead_code)]
    fn is_promote(csa: String) -> bool {
        let csa_vec = csa.chars().collect::<Vec<char>>();
        return csa_vec[4] == '+'
    }

    fn base_constructor(&mut self, from: u16, to: u16, pro: u16, drop: u16) {
        let mut bit_from = bitvec![u16, Msb0; 0; 16];
        let mut bit_to = bitvec![u16, Msb0; 0; 16];
        let mut bit_pro = bitvec![u16, Msb0; 0; 16];
        let mut bit_drop = bitvec![u16, Msb0; 0; 16];
        bit_from.store_be::<u16>(from);
        bit_to.store_be::<u16>(to);
        bit_pro.store_be::<u16>(pro);
        bit_drop.store_be::<u16>(drop);

        bit_drop.shift_left(15);
        bit_pro.shift_left(14);
        bit_to.shift_left(7);

        self.value = bit_from | bit_to | bit_pro | bit_drop;
    }

    #[allow(dead_code)]
    fn standart_constructor(&mut self, from: address::Address, to: address::Address, promote: bool) {
        self.base_constructor(from.to_index() as u16, to.to_index() as u16, promote as u16, 1);
    }

    #[allow(dead_code)]
    fn drop_constructor(&mut self, piece: piece::Piece, to: address::Address) {
        self.base_constructor(piece.to_u8() as u16, to.to_index() as u16, 0, 1)
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            value: bitvec![u16, Msb0; 0; 16],
        }
    }

    #[allow(dead_code)]
    pub fn from_standart(from: address::Address, to: address::Address, promote: bool) -> Self {
        let mut res = Self::new();
        res.standart_constructor(from, to, promote);
        return res
    }

    #[allow(dead_code)]
    pub fn from_drop(piece: piece::Piece, to: address::Address) -> Self {
        let mut res = Self::new();
        res.drop_constructor(piece, to);
        return res
    }

    #[allow(dead_code)]
    pub fn from_csa(csa: String) -> Self {
        let csa_vec = csa.chars().collect::<Vec<char>>();
        let mut res = Self::new();
        let clone_csa = csa.clone();
        let to = address::Address::from_string(&csa[2..]);
        if Self::is_drop(csa) {
            let piece = piece::Piece::from_char(csa_vec[0]);
            res.drop_constructor(piece, to);
        } else {
            let from = address::Address::from_string(&clone_csa);
            res.standart_constructor(from, to, Self::is_promote(clone_csa));
        }
        return res
    }
}