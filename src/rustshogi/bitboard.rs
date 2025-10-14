#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Shl, ShlAssign, Shr, ShrAssign};

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
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BitBoard {
    // data[0] is MSB, data[1] is LSB
    data: [u64; 2],
}

impl Default for BitBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl BitBoard {
    #[allow(dead_code)]
    pub fn new() -> Self {
        BitBoard { data: [0, 0] }
    }

    #[allow(dead_code)]
    pub fn from_bitboard(bitboard: Self) -> Self {
        bitboard
    }

    #[allow(dead_code)]
    pub fn from_u128(integer: u128) -> Self {
        BitBoard {
            data: [(integer >> 64) as u64, integer as u64],
        }
    }

    #[allow(dead_code)]
    pub fn from_str(string: &str) -> Self {
        let mut res = 0u128;
        let mut bit_pos = 127;
        for c in string.chars() {
            if bit_pos < 128 - LENGTH_OF_BOARD as i32 {
                break;
            }
            if c == '1' {
                res |= 1u128 << bit_pos;
                bit_pos -= 1;
            } else if c == '0' {
                bit_pos -= 1;
            }
        }
        Self::from_u128(res)
    }

    #[allow(dead_code)]
    pub fn to_u128(&self) -> u128 {
        ((self.data[0] as u128) << 64) | (self.data[1] as u128)
    }

    #[allow(dead_code)]
    pub fn get_trues(&self) -> Vec<u8> {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return unsafe { self.get_trues_avx512f() };
            }
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.get_trues_avx2() };
            }
        }
        self.get_trues_scalar()
    }

    fn get_trues_scalar(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut d0 = self.data[0];
        while d0 != 0 {
            let index = d0.leading_zeros() as u8;
            result.push(index);
            d0 &= !(1u64 << (63 - index));
        }
        let mut d1 = self.data[1];
        while d1 != 0 {
            let index = d1.leading_zeros() as u8;
            result.push(index + 64);
            d1 &= !(1u64 << (63 - index));
        }
        result
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[allow(dead_code)]
    unsafe fn get_trues_avx2(&self) -> Vec<u8> {
        self.get_trues_scalar()
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512f")]
    #[allow(dead_code)]
    unsafe fn get_trues_avx512f(&self) -> Vec<u8> {
        self.get_trues_scalar()
    }

    #[allow(dead_code)]
    pub fn get_trues_iter(&self) -> impl Iterator<Item = u8> + '_ {
        let mut d0 = self.data[0];
        let mut d1 = self.data[1];
        let mut state = 0; // 0 for d0, 1 for d1

        std::iter::from_fn(move || {
            if state == 0 {
                if d0 == 0 {
                    state = 1;
                } else {
                    let index = d0.leading_zeros() as u8;
                    d0 &= !(1u64 << (63 - index));
                    return Some(index);
                }
            }
            if state == 1 {
                if d1 == 0 {
                    return None;
                } else {
                    let index = d1.leading_zeros() as u8;
                    d1 &= !(1u64 << (63 - index));
                    return Some(index + 64);
                }
            }
            None
        })
    }

    #[allow(dead_code)]
    pub fn flip(&mut self) {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                *self = unsafe { avx512f_xor(self, &BOARD_MASK) };
                return;
            }
            if is_x86_feature_detected!("avx2") {
                *self = unsafe { avx2_xor(self, &BOARD_MASK) };
                return;
            }
            if is_x86_feature_detected!("sse2") {
                *self = unsafe { sse2_xor(self, &BOARD_MASK) };
                return;
            }
        }
        let board_mask = !((1u128 << (128 - LENGTH_OF_BOARD as u32)) - 1);
        let current = self.to_u128();
        let flipped = current ^ board_mask;
        *self = Self::from_u128(flipped);
    }
}

const BOARD_MASK_U128: u128 = !((1u128 << (128 - LENGTH_OF_BOARD as u32)) - 1);
static BOARD_MASK: BitBoard = BitBoard {
    data: [(BOARD_MASK_U128 >> 64) as u64, BOARD_MASK_U128 as u64],
};

#[cfg(target_arch = "x86_64")]
unsafe fn sse2_xor(a: &BitBoard, b: &BitBoard) -> BitBoard {
    let a_vec = _mm_loadu_si128(a.data.as_ptr() as *const __m128i);
    let b_vec = _mm_loadu_si128(b.data.as_ptr() as *const __m128i);
    let result = _mm_xor_si128(a_vec, b_vec);
    let mut output = [0u64; 2];
    _mm_storeu_si128(output.as_mut_ptr() as *mut __m128i, result);
    BitBoard { data: output }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn avx512f_bitor(a: &BitBoard, b: &BitBoard) -> BitBoard {
    let a_vec = _mm_loadu_si128(a.data.as_ptr() as *const __m128i);
    let b_vec = _mm_loadu_si128(b.data.as_ptr() as *const __m128i);
    let result = _mm_or_si128(a_vec, b_vec);
    let mut output = [0u64; 2];
    _mm_storeu_si128(output.as_mut_ptr() as *mut __m128i, result);
    BitBoard { data: output }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn avx512f_bitand(a: &BitBoard, b: &BitBoard) -> BitBoard {
    let a_vec = _mm_loadu_si128(a.data.as_ptr() as *const __m128i);
    let b_vec = _mm_loadu_si128(b.data.as_ptr() as *const __m128i);
    let result = _mm_and_si128(a_vec, b_vec);
    let mut output = [0u64; 2];
    _mm_storeu_si128(output.as_mut_ptr() as *mut __m128i, result);
    BitBoard { data: output }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn avx512f_xor(a: &BitBoard, b: &BitBoard) -> BitBoard {
    let a_vec = _mm_loadu_si128(a.data.as_ptr() as *const __m128i);
    let b_vec = _mm_loadu_si128(b.data.as_ptr() as *const __m128i);
    let result = _mm_xor_si128(a_vec, b_vec);
    let mut output = [0u64; 2];
    _mm_storeu_si128(output.as_mut_ptr() as *mut __m128i, result);
    BitBoard { data: output }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn avx2_xor(a: &BitBoard, b: &BitBoard) -> BitBoard {
    let a_vec = _mm_loadu_si128(a.data.as_ptr() as *const __m128i);
    let b_vec = _mm_loadu_si128(b.data.as_ptr() as *const __m128i);
    let result = _mm_xor_si128(a_vec, b_vec);
    let mut output = [0u64; 2];
    _mm_storeu_si128(output.as_mut_ptr() as *mut __m128i, result);
    BitBoard { data: output }
}

#[cfg(target_arch = "x86_64")]
unsafe fn sse2_bitand(a: &BitBoard, b: &BitBoard) -> BitBoard {
    let a_vec = _mm_loadu_si128(a.data.as_ptr() as *const __m128i);
    let b_vec = _mm_loadu_si128(b.data.as_ptr() as *const __m128i);
    let result = _mm_and_si128(a_vec, b_vec);
    let mut output = [0u64; 2];
    _mm_storeu_si128(output.as_mut_ptr() as *mut __m128i, result);
    BitBoard { data: output }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn avx2_bitand(a: &BitBoard, b: &BitBoard) -> BitBoard {
    let a_vec = _mm_loadu_si128(a.data.as_ptr() as *const __m128i);
    let b_vec = _mm_loadu_si128(b.data.as_ptr() as *const __m128i);
    let result = _mm_and_si128(a_vec, b_vec);
    let mut output = [0u64; 2];
    _mm_storeu_si128(output.as_mut_ptr() as *mut __m128i, result);
    BitBoard { data: output }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return unsafe { avx512f_bitand(&self, &rhs) };
            }
            if is_x86_feature_detected!("avx2") {
                return unsafe { avx2_bitand(&self, &rhs) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { sse2_bitand(&self, &rhs) };
            }
        }
        BitBoard {
            data: [self.data[0] & rhs.data[0], self.data[1] & rhs.data[1]],
        }
    }
}

impl BitAnd<&BitBoard> for &BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: &BitBoard) -> Self::Output {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return unsafe { avx512f_bitand(self, rhs) };
            }
            if is_x86_feature_detected!("avx2") {
                return unsafe { avx2_bitand(self, rhs) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { sse2_bitand(self, rhs) };
            }
        }
        BitBoard {
            data: [self.data[0] & rhs.data[0], self.data[1] & rhs.data[1]],
        }
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.data[0] &= rhs.data[0];
        self.data[1] &= rhs.data[1];
    }
}

impl BitAndAssign<&BitBoard> for BitBoard {
    fn bitand_assign(&mut self, rhs: &BitBoard) {
        self.data[0] &= rhs.data[0];
        self.data[1] &= rhs.data[1];
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn sse2_bitor(a: &BitBoard, b: &BitBoard) -> BitBoard {
    let a_vec = _mm_loadu_si128(a.data.as_ptr() as *const __m128i);
    let b_vec = _mm_loadu_si128(b.data.as_ptr() as *const __m128i);
    let result = _mm_or_si128(a_vec, b_vec);
    let mut output = [0u64; 2];
    _mm_storeu_si128(output.as_mut_ptr() as *mut __m128i, result);
    BitBoard { data: output }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn avx2_bitor(a: &BitBoard, b: &BitBoard) -> BitBoard {
    let a_vec = _mm_loadu_si128(a.data.as_ptr() as *const __m128i);
    let b_vec = _mm_loadu_si128(b.data.as_ptr() as *const __m128i);
    let result = _mm_or_si128(a_vec, b_vec);
    let mut output = [0u64; 2];
    _mm_storeu_si128(output.as_mut_ptr() as *mut __m128i, result);
    BitBoard { data: output }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return unsafe { avx512f_bitor(&self, &rhs) };
            }
            if is_x86_feature_detected!("avx2") {
                return unsafe { avx2_bitor(&self, &rhs) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { sse2_bitor(&self, &rhs) };
            }
        }
        BitBoard {
            data: [self.data[0] | rhs.data[0], self.data[1] | rhs.data[1]],
        }
    }
}

impl BitOr<&BitBoard> for &BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: &BitBoard) -> Self::Output {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return unsafe { avx512f_bitor(self, &rhs) };
            }
            if is_x86_feature_detected!("avx2") {
                return unsafe { avx2_bitor(self, &rhs) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { sse2_bitor(self, &rhs) };
            }
        }
        BitBoard {
            data: [self.data[0] | rhs.data[0], self.data[1] | rhs.data[1]],
        }
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data[0] |= rhs.data[0];
        self.data[1] |= rhs.data[1];
    }
}

impl BitOrAssign<&BitBoard> for BitBoard {
    fn bitor_assign(&mut self, rhs: &BitBoard) {
        self.data[0] |= rhs.data[0];
        self.data[1] |= rhs.data[1];
    }
}

impl Shr<usize> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        if rhs >= 128 {
            BitBoard { data: [0, 0] }
        } else if rhs >= 64 {
            BitBoard {
                data: [0, self.data[0] >> (rhs - 64)],
            }
        } else if rhs > 0 {
            BitBoard {
                data: [
                    self.data[0] >> rhs,
                    (self.data[1] >> rhs) | (self.data[0] << (64 - rhs)),
                ],
            }
        } else {
            self
        }
    }
}

impl ShrAssign<usize> for BitBoard {
    fn shr_assign(&mut self, rhs: usize) {
        *self = *self >> rhs;
    }
}

impl Shl<usize> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        if rhs >= 128 {
            BitBoard { data: [0, 0] }
        } else if rhs >= 64 {
            BitBoard {
                data: [self.data[1] << (rhs - 64), 0],
            }
        } else if rhs > 0 {
            BitBoard {
                data: [
                    (self.data[0] << rhs) | (self.data[1] >> (64 - rhs)),
                    self.data[1] << rhs,
                ],
            }
        } else {
            self
        }
    }
}

impl ShlAssign<usize> for BitBoard {
    fn shl_assign(&mut self, rhs: usize) {
        *self = *self << rhs;
    }
}

#[allow(dead_code)]
pub fn generate_columns(column_nos: Vec<usize>) -> BitBoard {
    let mut bitboard = BitBoard::new();
    let mut first_row_mask = BitBoard::new();
    for column_no in column_nos {
        // Assumes column_no is 0-indexed (0-8) for the 9 playable columns
        // The first playable row starts at index 12 (1*11 + 1)
        let index = 11 + (column_no + 1);
        first_row_mask |= BitBoard::from_u128(1u128 << (127 - index));
    }

    for _r in 0..LENGTH_OF_EDGE {
        bitboard |= first_row_mask;
        first_row_mask >>= LENGTH_OF_FRAME as usize;
    }
    bitboard
}

#[allow(dead_code)]
pub fn generate_column(column_no: usize) -> BitBoard {
    // Assumes column_no is 0-indexed (0-8) for the 9 playable columns
    let mut bitboard = BitBoard::new();
    let index = 11 + (column_no + 1);
    let mut mask = BitBoard::from_u128(1u128 << (127 - index));
    for _r in 0..LENGTH_OF_EDGE {
        bitboard |= mask;
        mask >>= LENGTH_OF_FRAME as usize;
    }
    bitboard
}

// impl std::fmt::Display for BitBoard {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

//         write!(f, "{}", convert_string(*self))
//     }
// }
