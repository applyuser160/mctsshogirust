#[path = "address.rs"]
pub mod address;
#[path = "bitboard.rs"]
pub mod bitboard;
#[path = "board.rs"]
pub mod board;
#[path = "color.rs"]
pub mod color;
#[path = "common.rs"]
pub mod common;
#[path = "direction.rs"]
pub mod direction;
#[path = "game.rs"]
pub mod game;
#[path = "hand.rs"]
pub mod hand;
#[path = "mctsresult.rs"]
pub mod mctsresult;
#[path = "moves.rs"]
pub mod moves;
#[path = "piece.rs"]
pub mod piece;
#[path = "random.rs"]
pub mod random;

#[cfg(test)]
#[path = "test_address.rs"]
pub mod test_address;
#[cfg(test)]
#[path = "test_bitboard.rs"]
pub mod test_bitboard;
#[cfg(test)]
#[path = "test_board.rs"]
pub mod test_board;
#[cfg(test)]
#[path = "test_color.rs"]
pub mod test_color;
#[cfg(test)]
#[path = "test_common.rs"]
pub mod test_common;
#[cfg(test)]
#[path = "test_direction.rs"]
pub mod test_direction;
#[cfg(test)]
#[path = "test_game.rs"]
pub mod test_game;
#[cfg(test)]
#[path = "test_hand.rs"]
pub mod test_hand;
#[cfg(test)]
#[path = "test_moves.rs"]
pub mod test_moves;
#[cfg(test)]
#[path = "test_piece.rs"]
pub mod test_piece;
#[cfg(test)]
#[path = "test_random.rs"]
pub mod test_random;

use pyo3::prelude::*;

#[pymodule]
fn _core(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<address::Address>()?;
    m.add_class::<color::ColorType>()?;
    m.add_class::<piece::PieceType>()?;
    m.add_class::<piece::Piece>()?;
    m.add_class::<moves::Move>()?;
    m.add_class::<hand::Hand>()?;
    m.add_class::<board::Board>()?;
    m.add_class::<game::Game>()?;
    m.add_class::<mctsresult::MctsResult>()?;
    Ok(())
}
