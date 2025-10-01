use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Shl, ShlAssign, Shr, ShrAssign};

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
#[derive(Clone, PartialEq, Debug)]
pub struct BitBoard {
    pub board: BitVec<u64, Msb0>,
}

impl Default for BitBoard {
    fn default() -> Self {
        Self::new()
    }
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
            board: bitboard.board,
        }
    }

    #[allow(dead_code)]
    pub fn from_bitvec(bitvec: BitVec<u64, Msb0>) -> Self {
        Self { board: bitvec }
    }

    #[allow(dead_code)]
    pub fn from_u128(integer: u128) -> Self {
        let mut res = BitBoard::new();
        res.board.store_be::<u128>(integer);
        res
    }

    #[allow(dead_code)]
    pub fn from_str(string: &str) -> Self {
        let mut res = BitBoard::new();
        for (i, c) in string.chars().enumerate() {
            res.board.set(i, c == '1');
        }
        res
    }

    #[allow(dead_code)]
    pub fn to_u128(&self) -> u128 {
        let chunks = self.board.as_raw_slice();
        let mut result: u128 = 0;

        for (i, &chunk) in chunks.iter().enumerate().take(2) {
            result |= (chunk as u128) << ((1 - i) * 64);
        }

        result
    }

    #[allow(dead_code)]
    pub fn get_trues(&self) -> Vec<u8> {
        let chunks = self.board.as_raw_slice();
        let mut result = Vec::new();

        for (chunk_idx, &chunk) in chunks.iter().enumerate().take(2) {
            if chunk == 0 {
                continue;
            }

            for bit_idx in 0..64 {
                let global_bit_idx = chunk_idx * 64 + bit_idx;
                if global_bit_idx >= LENGTH_OF_BOARD as usize {
                    break;
                }

                if chunk & (1u64 << (63 - bit_idx)) != 0 {
                    result.push(global_bit_idx as u8);
                }
            }
        }

        result
    }

    #[allow(dead_code)]
    pub fn flip(&mut self) {
        let chunks = self.board.as_raw_mut_slice();

        for (i, chunk) in chunks.iter_mut().enumerate().take(2) {
            if i * 64 >= LENGTH_OF_BOARD as usize {
                break;
            }

            if i == 1 {
                let mask = ((1u64 << 57) - 1) << 7;
                *chunk = !(*chunk & mask) | (*chunk & !mask);
            } else {
                *chunk = !*chunk;
            }
        }
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();

        let self_chunks = result.board.as_raw_mut_slice();
        let rhs_chunks = rhs.board.as_raw_slice();

        for (self_chunk, rhs_chunk) in self_chunks.iter_mut().zip(rhs_chunks.iter()) {
            *self_chunk &= rhs_chunk;
        }

        result
    }
}

impl BitAnd<&BitBoard> for &BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: &BitBoard) -> Self::Output {
        let mut result = self.clone();

        let self_chunks = result.board.as_raw_mut_slice();
        let rhs_chunks = rhs.board.as_raw_slice();

        for (self_chunk, rhs_chunk) in self_chunks.iter_mut().zip(rhs_chunks.iter()) {
            *self_chunk &= rhs_chunk;
        }

        result
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        let self_chunks = self.board.as_raw_mut_slice();
        let rhs_chunks = rhs.board.as_raw_slice();

        for (self_chunk, rhs_chunk) in self_chunks.iter_mut().zip(rhs_chunks.iter()) {
            *self_chunk &= rhs_chunk;
        }
    }
}

impl BitAndAssign<&BitBoard> for BitBoard {
    fn bitand_assign(&mut self, rhs: &BitBoard) {
        let self_chunks = self.board.as_raw_mut_slice();
        let rhs_chunks = rhs.board.as_raw_slice();

        for (self_chunk, rhs_chunk) in self_chunks.iter_mut().zip(rhs_chunks.iter()) {
            *self_chunk &= rhs_chunk;
        }
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();

        let self_chunks = result.board.as_raw_mut_slice();
        let rhs_chunks = rhs.board.as_raw_slice();

        for (self_chunk, rhs_chunk) in self_chunks.iter_mut().zip(rhs_chunks.iter()) {
            *self_chunk |= rhs_chunk;
        }

        result
    }
}

impl BitOr<&BitBoard> for &BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: &BitBoard) -> Self::Output {
        let mut result = self.clone();

        let self_chunks = result.board.as_raw_mut_slice();
        let rhs_chunks = rhs.board.as_raw_slice();

        for (self_chunk, rhs_chunk) in self_chunks.iter_mut().zip(rhs_chunks.iter()) {
            *self_chunk |= rhs_chunk;
        }
        result
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        let self_chunks = self.board.as_raw_mut_slice();
        let rhs_chunks = rhs.board.as_raw_slice();

        for (self_chunk, rhs_chunk) in self_chunks.iter_mut().zip(rhs_chunks.iter()) {
            *self_chunk |= rhs_chunk;
        }
    }
}

impl BitOrAssign<&BitBoard> for BitBoard {
    fn bitor_assign(&mut self, rhs: &BitBoard) {
        let self_chunks = self.board.as_raw_mut_slice();
        let rhs_chunks = rhs.board.as_raw_slice();

        for (self_chunk, rhs_chunk) in self_chunks.iter_mut().zip(rhs_chunks.iter()) {
            *self_chunk |= rhs_chunk;
        }
    }
}

impl Shr<usize> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut res = self.clone();
        res.board.shift_right(rhs);
        res
    }
}

impl ShrAssign<usize> for BitBoard {
    fn shr_assign(&mut self, rhs: usize) {
        self.board.shift_right(rhs);
    }
}

impl Shl<usize> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut res = self.clone();
        res.board.shift_left(rhs);
        res
    }
}

impl ShlAssign<usize> for BitBoard {
    fn shl_assign(&mut self, rhs: usize) {
        self.board.shift_left(rhs);
    }
}

#[allow(dead_code)]
pub fn generate_columns(column_nos: Vec<usize>) -> BitBoard {
    let mut bitboard = BitBoard::new();
    for _i in 0..LENGTH_OF_EDGE {
        for column_no in column_nos.iter() {
            bitboard.board.set(*column_no, true);
        }
        bitboard.board.shift_right(LENGTH_OF_FRAME.into());
    }
    bitboard
}

#[allow(dead_code)]
pub fn generate_column(column_no: usize) -> BitBoard {
    let mut bitboard = BitBoard::new();
    for _i in 0..LENGTH_OF_EDGE {
        bitboard.board.set(column_no, true);
        bitboard.board.shift_right(LENGTH_OF_FRAME.into());
    }
    bitboard
}

// impl std::fmt::Display for BitBoard {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

//         write!(f, "{}", convert_string(*self))
//     }
// }
