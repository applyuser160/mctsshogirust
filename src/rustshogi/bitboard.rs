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
pub struct BitBoard(pub u128);

impl Default for BitBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl BitBoard {
    #[allow(dead_code)]
    pub fn new() -> Self {
        BitBoard(0)
    }

    #[allow(dead_code)]
    pub fn from_bitboard(bitboard: Self) -> Self {
        BitBoard(bitboard.0)
    }

    #[allow(dead_code)]
    pub fn from_u128(integer: u128) -> Self {
        BitBoard(integer)
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
            let index = board.leading_zeros() as u8;
            result.push(index);
            board &= !(1u128 << (127 - index));
        }
        result
    }

    #[allow(dead_code)]
    pub fn get_trues_iter(&self) -> impl Iterator<Item = u8> + '_ {
        let mut board = self.0;
        std::iter::from_fn(move || {
            if board == 0 {
                None
            } else {
                let index = board.leading_zeros() as u8;
                board &= !(1u128 << (127 - index));
                Some(index)
            }
        })
    }

    #[allow(dead_code)]
    pub fn flip(&mut self) {
        let board_mask = !((1u128 << (128 - LENGTH_OF_BOARD as u32)) - 1);
        self.0 ^= board_mask;
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAnd<&BitBoard> for &BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: &BitBoard) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitAndAssign<&BitBoard> for BitBoard {
    fn bitand_assign(&mut self, rhs: &BitBoard) {
        self.0 &= rhs.0;
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOr<&BitBoard> for &BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: &BitBoard) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<&BitBoard> for BitBoard {
    fn bitor_assign(&mut self, rhs: &BitBoard) {
        self.0 |= rhs.0;
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
    let mut first_row_mask = BitBoard::new();
    for column_no in column_nos {
        // Assumes column_no is 0-indexed (0-8) for the 9 playable columns
        // The first playable row starts at index 12 (1*11 + 1)
        let index = 11 + (column_no + 1);
        first_row_mask.0 |= 1u128 << (127 - index);
    }

    for _r in 0..LENGTH_OF_EDGE {
        bitboard.0 |= first_row_mask.0;
        first_row_mask.0 >>= LENGTH_OF_FRAME;
    }
    bitboard
}

#[allow(dead_code)]
pub fn generate_column(column_no: usize) -> BitBoard {
    // Assumes column_no is 0-indexed (0-8) for the 9 playable columns
    let mut bitboard = BitBoard::new();
    let index = 11 + (column_no + 1);
    let mut mask = 1u128 << (127 - index);
    for _r in 0..LENGTH_OF_EDGE {
        bitboard.0 |= mask;
        mask >>= LENGTH_OF_FRAME;
    }
    bitboard
}

// impl std::fmt::Display for BitBoard {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

//         write!(f, "{}", convert_string(*self))
//     }
// }
