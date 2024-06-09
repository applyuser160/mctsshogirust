use std::ops::{BitAnd, BitOr, Shl, Shr};

use bitvec::prelude::*;

#[allow(dead_code)]
pub const LENGTH_OF_BOARD: u8 = 121;

#[allow(dead_code)]
pub const LENGTH_OF_FRAME: u8 = 11;

#[allow(dead_code)]
pub const LENGTH_OF_EDGE: u8 = 9;

#[allow(dead_code)]
pub const BIT_OF_FRAME: u128 = 340199411925109678410730024455729840000;

#[allow(dead_code)]
pub const STRING_OF_FRAME: &str = "\
11111111111\
10000000001\
10000000001\
10000000001\
10000000001\
10000000001\
10000000001\
10000000001\
10000000001\
10000000001\
11111111111";

#[allow(dead_code)]
pub const BIT_OF_PRO_ZONE_BLACK: u128 = 1124249833570304;

#[allow(dead_code)]
pub const STRING_OF_PRO_ZONE_BLACK: &str = "\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
01111111110\
01111111110\
01111111110\
00000000000";

#[allow(dead_code)]
pub const BIT_OF_PRO_ZONE_WHITE: u128 = 82954995819127820108022929011245056;

#[allow(dead_code)]
pub const STRING_OF_PRO_ZONE_WHITE: &str = "\
00000000000\
01111111110\
01111111110\
01111111110\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000";

#[allow(dead_code)]
pub const BIT_OF_LAST1_ZONE_BLACK: u128 = 267911168;

#[allow(dead_code)]
pub const STRING_OF_LAST1_ZONE_BLACK: &str = "\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
01111111110\
00000000000";

#[allow(dead_code)]
pub const BIT_OF_LAST1_ZONE_WHITE: u128 = 82914490459728028693096363257233408;

#[allow(dead_code)]
pub const STRING_OF_LAST1_ZONE_WHITE: &str = "\
00000000000\
01111111110\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000";

#[allow(dead_code)]
pub const BIT_OF_LAST2_ZONE_BLACK: u128 = 548949983232;

#[allow(dead_code)]
pub const STRING_OF_LAST2_ZONE_BLACK: &str = "\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
01111111110\
01111111110\
00000000000";

#[allow(dead_code)]
pub const BIT_OF_LAST2_ZONE_WHITE: u128 = 82954976050772817769606664215855104;

#[allow(dead_code)]
pub const STRING_OF_LAST2_ZONE_WHITE: &str = "\
00000000000\
01111111110\
01111111110\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000\
00000000000";


#[allow(dead_code)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct BitBoard {
    pub board: BitVec<u64, Msb0>,
}

impl BitBoard {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            board: bitvec![u64, Msb0; 0; 128],
        }
    }

    #[allow(dead_code)]
    pub fn from_bitboard(bitboard: Self) -> Self {
        Self {
            board: bitboard.board
        }
    }

    #[allow(dead_code)]
    pub fn from_bitvec(bitvec: BitVec<u64, Msb0>) -> Self {
        Self {
            board: bitvec
        }
    }

    #[allow(dead_code)]
    pub fn from_u128(integer: u128) -> Self {
        let mut res = BitBoard::new();
        res.board.store_be::<u128>(integer);
        return res
    }

    #[allow(dead_code)]
    pub fn from_str(string: &str) -> Self {
        let mut res = BitBoard::new();
        for (i, c) in string.chars().enumerate() {
            res.board.set(i, c == '1');
        }
        return res
    }

    #[allow(dead_code)]
    pub fn to_u128(&self) -> u128 {
        let mut res: u128 = 0;
        let base: u128 = 2;
        for i in 0..128 {
            if self.board[i] {
                res += base.pow((127 - i) as u32);
            }
        }
        return res
    }

    #[allow(dead_code)]
    pub fn get_trues(&self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::with_capacity(121);
        for i in 0..121 {
            if self.board[i] {
                res.push(i as u8);
            }
        }
        return res
    }

    #[allow(dead_code)]
    pub fn flip(&mut self) {
        self.board.iter_mut().for_each(|mut b| *b = !*b);
    }

}

impl BitAnd for BitBoard {
    type Output = Self;

    #[inline]
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self.board &= rhs.board;
        self
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    #[inline]
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self.board |= rhs.board;
        self
    }
}

impl Shr<usize> for BitBoard {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        self.board.shift_right(rhs);
        self
    }
}

impl Shl<usize> for BitBoard {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        self.board.shift_left(rhs);
        self
    }
}

#[allow(dead_code)]
pub fn generate_columns(column_no: Vec<usize>, column_count: usize) -> BitBoard {
    let mut bitboard = BitBoard::new();
    for _i in 0..LENGTH_OF_EDGE {
        for j in 0..column_count {
            bitboard.board.set(column_no[j], true);
        }
        bitboard.board.shift_right(11);
    }
    return bitboard
}

#[allow(dead_code)]
pub fn generate_column(column_no: usize) -> BitBoard {
    let mut bitboard = BitBoard::new();
    for _i in 0..LENGTH_OF_EDGE {
        bitboard.board.set(column_no, true);
        bitboard.board.shift_right(11);
    }
    return bitboard
}

// impl std::fmt::Display for BitBoard {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

//         write!(f, "{}", convert_string(*self))
//     }
// }