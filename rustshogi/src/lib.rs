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

    let result = game.random_move(num);
    Ok(result)
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn rustshogi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(goresult, m)?)?;
    Ok(())
}
