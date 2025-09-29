#[path = "rustshogi/address.rs"]
pub mod address;
#[path = "rustshogi/bitboard.rs"]
pub mod bitboard;
#[path = "rustshogi/board.rs"]
pub mod board;
#[path = "rustshogi/color.rs"]
pub mod color;
#[path = "rustshogi/common.rs"]
pub mod common;
#[path = "rustshogi/direction.rs"]
pub mod direction;
#[path = "rustshogi/game.rs"]
pub mod game;
#[path = "rustshogi/hand.rs"]
pub mod hand;
#[path = "rustshogi/mctsresult.rs"]
pub mod mctsresult;
#[path = "rustshogi/moves.rs"]
pub mod moves;
#[path = "rustshogi/piece.rs"]
pub mod piece;
#[path = "rustshogi/random.rs"]
pub mod random;

#[cfg(test)]
#[path = "rustshogi/test_address.rs"]
pub mod test_address;
#[cfg(test)]
#[path = "rustshogi/test_bitboard.rs"]
pub mod test_bitboard;
#[cfg(test)]
#[path = "rustshogi/test_board.rs"]
pub mod test_board;
#[cfg(test)]
#[path = "rustshogi/test_color.rs"]
pub mod test_color;
#[cfg(test)]
#[path = "rustshogi/test_common.rs"]
pub mod test_common;
#[cfg(test)]
#[path = "rustshogi/test_direction.rs"]
pub mod test_direction;
#[cfg(test)]
#[path = "rustshogi/test_game.rs"]
pub mod test_game;
#[cfg(test)]
#[path = "rustshogi/test_hand.rs"]
pub mod test_hand;
#[cfg(test)]
#[path = "rustshogi/test_moves.rs"]
pub mod test_moves;
#[cfg(test)]
#[path = "rustshogi/test_piece.rs"]
pub mod test_piece;
#[cfg(test)]
#[path = "rustshogi/test_random.rs"]
pub mod test_random;

use pyo3::prelude::*;

#[pymodule]
fn rustshogi(_py: Python, _m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
