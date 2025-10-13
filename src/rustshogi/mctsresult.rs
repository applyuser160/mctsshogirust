use super::color::ColorType;
use super::moves::Move;
use pyo3::prelude::*;

#[allow(dead_code)]
#[pyclass]
#[derive(Clone)]
pub struct MctsResult {
    #[pyo3(get, set)]
    pub result: Vec<[u64; ColorType::ColorNumber as usize + 1]>,
    #[pyo3(get, set)]
    pub next_moves: Vec<Move>,
    #[pyo3(get, set)]
    pub next_move_count: u64,
    #[pyo3(get, set)]
    pub count: u64,
}

#[pymethods]
impl MctsResult {
    pub fn merge(&mut self, other: &MctsResult) {
        for i in 0..self.result.len() {
            for j in 0..self.result[i].len() {
                self.result[i][j] += other.result[i][j];
            }
        }
        self.count += other.count;
    }
}

impl Default for MctsResult {
    fn default() -> Self {
        Self::new()
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
