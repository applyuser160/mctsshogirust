use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, ShlAssign, Shr, ShrAssign};

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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BitBoard(pub u128);

impl Default for BitBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitBoard {
    #[allow(dead_code)]
    pub fn new() -> Self {
        BitBoard(0)
    }

    #[allow(dead_code)]
    pub fn from_u128(integer: u128) -> Self {
        BitBoard(integer)
    }

    #[allow(dead_code)]
    pub fn from_str(string: &str) -> Self {
        let mut res = 0;
        for (i, c) in string.chars().enumerate() {
            if c == '1' {
                res |= 1 << (LENGTH_OF_BOARD as usize - 1 - i);
            }
        }
        BitBoard(res)
    }

    #[allow(dead_code)]
    pub fn to_u128(&self) -> u128 {
        self.0
    }

    #[allow(dead_code)]
    pub fn get_trues(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut board = self.0;
        while board != 0 {
            let index = board.trailing_zeros() as u8;
            result.push(LENGTH_OF_BOARD - 1 - index);
            board &= board - 1; // remove the least significant bit
        }
        result
    }

    #[allow(dead_code)]
    pub fn set(&mut self, index: usize, value: bool) {
        if value {
            self.0 |= 1 << (LENGTH_OF_BOARD as usize - 1 - index);
        } else {
            self.0 &= !(1 << (LENGTH_OF_BOARD as usize - 1 - index));
        }
    }

    #[allow(dead_code)]
    pub fn any(&self) -> bool {
        self.0 != 0
    }

    #[allow(dead_code)]
    pub fn get(&self, index: usize) -> bool {
        (self.0 >> (LENGTH_OF_BOARD as usize - 1 - index)) & 1 == 1
    }

    #[allow(dead_code)]
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }
}

impl BitAnd for &BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitAndAssign<&BitBoard> for BitBoard {
    fn bitand_assign(&mut self, rhs: &Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for &BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl Shr<usize> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

impl ShrAssign<usize> for BitBoard {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}

impl Shl<usize> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

impl ShlAssign<usize> for BitBoard {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}

#[allow(dead_code)]
pub fn generate_columns(column_nos: Vec<usize>) -> BitBoard {
    let mut bitboard = BitBoard::new();
    for _i in 0..LENGTH_OF_EDGE {
        for column_no in column_nos.iter() {
            bitboard.set(*column_no, true);
        }
        bitboard >>= LENGTH_OF_FRAME as usize;
    }
    bitboard
}

#[allow(dead_code)]
pub fn generate_column(column_no: usize) -> BitBoard {
    let mut bitboard = BitBoard::new();
    for r in 1..=LENGTH_OF_EDGE {
        let index = column_no + (r as usize) * (LENGTH_OF_FRAME as usize);
        bitboard.set(index, true);
    }
    bitboard
}

// impl std::fmt::Display for BitBoard {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

//         write!(f, "{}", convert_string(*self))
//     }
// }
