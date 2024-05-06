use bitvec::prelude::*;

use super::address;
use super::piece;
use super::color;

#[allow(dead_code)]
pub struct Move {
    pub value: BitVec<u64, Msb0>,
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
}