mod common;
mod address;
mod direction;
mod random;
mod bitboard;
mod color;
mod piece;
mod hand;
mod moves;
mod board;
mod game;
mod mctsresult;
use pyo3::prelude::*;
use mctsresult::MctsResult;

#[pyfunction]
fn goresult(board: String, turn: String, hand: String, move_number: String, num: usize) -> PyResult<MctsResult> {
    let mut game = game::Game::new();

    game.input_board(board);
    game.input_turn(turn);
    game.input_hand(hand);
    game.input_move_number(move_number);

    let result = game.random_move_parallel(num, 12);
    Ok(result)
}


#[pymodule]
fn rustshogi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(goresult, m)?)?;
    Ok(())
}
