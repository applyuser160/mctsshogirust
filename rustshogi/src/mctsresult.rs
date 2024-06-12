use pyo3::prelude::*;
use super::moves::Move;
use super::color::ColorType;

#[pyclass]
#[allow(dead_code)]
pub struct MctsResult {
    #[pyo3(get)]
    pub result: Vec<[u64; ColorType::ColorNumber as usize + 1]>,
    pub next_moves: Vec<Move>,
    #[pyo3(get)]
    pub next_move_count: u64,
    #[pyo3(get)]
    pub count: u64,
}

#[pymethods]
impl MctsResult {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }

    #[getter]
    pub fn get_next_moves(&self) -> Vec<String> {
        self.next_moves.clone().iter().map(|m| m.to_string()).collect()
    }
}

impl MctsResult {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            result: Vec::new(),
            next_moves: Vec::new(),
            next_move_count: 0,
            count: 0,
        }
    }

    #[allow(dead_code)]
    pub fn from(next_move_count: u64, next_moves: Vec<Move>) -> Self {
        let mut result: Vec<[u64; ColorType::ColorNumber as usize + 1]> = Vec::new();
        for _i in 0..next_move_count {
            result.push([0; ColorType::ColorNumber as usize + 1]);
        }
        Self {
            result,
            next_moves,
            next_move_count,
            count: 0,
        }
    }

    #[allow(dead_code)]
    pub fn merge(&mut self, result: MctsResult) {
        for i in 0..self.next_moves.len() {
            for j in 0..=ColorType::ColorNumber as usize {
                self.result[i][j] += result.result[i][j];
            }
        }
    }

    #[allow(dead_code)]
    pub fn plus_result(&mut self, winner: ColorType, next_move_index: usize) {
        if winner as i8 > -1 {
            self.result[next_move_index][winner as usize] += 1;
        }
        self.result[next_move_index][ColorType::ColorNumber as usize] += 1;
        self.count += 1;
    }

    #[allow(dead_code)]
    pub fn calc_result(&mut self, turn: ColorType) {
        let mut best_index: usize = 0;
        let mut best_win_per: f64 = 0.0;

        for i in 0..self.next_move_count as usize {
            let total = self.result[i][ColorType::ColorNumber as usize] as f64;
            let current_turn_win = if turn == ColorType::White {
                self.result[i][ColorType::White as usize] as f64
            } else {
                self.result[i][ColorType::Black as usize] as f64
            };
            let win_per = current_turn_win / total;

            if win_per > best_win_per {
                best_win_per = win_per;
                best_index = i;
            }
        }

        println!("{} times played!", self.count);
        println!("bestmove {}", self.next_moves[best_index].to_string());
    }
}