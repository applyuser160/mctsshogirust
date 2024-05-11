use bitvec::prelude::*;

use bitvec::prelude::BitVec;

use super::address;
use super::piece;

#[allow(dead_code)]
#[derive(Clone)]
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

    #[allow(dead_code)]
    pub fn get_is_drop(&self) -> bool {
        return self.value[15]
    }

    #[allow(dead_code)]
    pub fn get_is_promote(&self) -> bool {
        return self.value[14]
    }

    #[allow(dead_code)]
    pub fn get_base(&self, left: usize, right: usize) -> u8 {
        let mut copy = self.value.clone();
        copy.shift_left(left);
        copy.shift_right(right);
        let r: std::ops::Range<usize> = 0..16;
        let f = copy.get(r).unwrap();
        return f.load::<u8>();
    }

    #[allow(dead_code)]
    pub fn get_from(&self) -> address::Address {
        let v = self.get_base(9, 9);
        return address::Address::from_number(v)
    }

    #[allow(dead_code)]
    pub fn get_to(&self) -> address::Address {
        let v = self.get_base(2, 9);
        return address::Address::from_number(v)
    }

    #[allow(dead_code)]
    pub fn get_piece(&self) -> piece::Piece {
        let v = self.get_base(9, 9);
        return piece::Piece::from_u8(v)
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
        return res
    }
}