use super::address::Address;
use super::board::Board;
use super::color::{convert_from_string, get_reverse_color, ColorType};
use super::mctsresult::MctsResult;
use super::moves::Move;
use super::piece::Piece;
use super::random::Random;
use num_cpus;
use rayon::prelude::*;

use pyo3::prelude::*;

#[allow(dead_code)]
#[pyclass]
#[derive(Clone)]
pub struct Game {
    #[pyo3(get, set)]
    pub board: Board,
    #[pyo3(get, set)]
    pub move_number: u16,
    #[pyo3(get, set)]
    pub turn: ColorType,
    #[pyo3(get, set)]
    pub winner: ColorType,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            move_number: 1,
            turn: ColorType::Black,
            winner: ColorType::None,
        }
    }

    #[allow(dead_code)]
    pub fn from(board: Board, move_number: u16, turn: ColorType, winner: ColorType) -> Self {
        Self {
            board,
            move_number,
            turn,
            winner,
        }
    }

    #[allow(dead_code)]
    pub fn input_board(&mut self, sfen: String) {
        let startpos = String::from("startpos");
        if startpos == sfen {
            self.board.startpos();
            return;
        }

        let parts: Vec<&str> = sfen.split('/').collect();
        for (row, part) in parts.iter().enumerate().rev() {
            let mut column = 0;
            let chars = part.chars();
            for ch in chars {
                if ch.is_ascii_digit() {
                    let empty_spaces = ch.to_digit(10).unwrap() as usize;
                    column += empty_spaces;
                } else {
                    let piece = Piece::from_char(ch);
                    let piece_type = piece.piece_type;
                    let owner = piece.owner;
                    let index =
                        Address::from_numbers((1 + column) as u8, (9 - row) as u8).to_index();
                    self.board.deploy(index, piece_type, owner);
                    column += 1;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn input_hand(&mut self, sfen: String) {
        if sfen == "-" {
            return;
        }
        let mut current_sfen = sfen.chars();
        while let Some(ch) = current_sfen.next() {
            if ch.is_ascii_digit() {
                let consecutive = ch.to_digit(10).unwrap() as u8;
                let piece = Piece::from_char(current_sfen.next().unwrap());
                self.board
                    .hand
                    .add_pieces(piece.owner, piece.piece_type, consecutive);
            } else {
                let piece = Piece::from_char(ch);
                self.board.hand.add_piece(piece.owner, piece.piece_type);
            }
        }
    }

    #[allow(dead_code)]
    pub fn input_move_number(&mut self, sfen: String) {
        self.move_number = sfen.parse::<u16>().unwrap_or(0);
    }

    #[allow(dead_code)]
    pub fn input_turn(&mut self, sfen: String) {
        self.turn = convert_from_string(sfen.chars().next().unwrap_or(' '));
    }

    #[allow(dead_code)]
    pub fn is_finished(&self) -> (bool, ColorType) {
        if self.move_number >= 500 {
            (true, ColorType::None)
        } else {
            self.board.is_finished()
        }
    }

    #[allow(dead_code)]
    pub fn execute_move(&mut self, mv: &Move) {
        self.board.execute_move(mv);
        self.move_number += 1;
        self.turn = get_reverse_color(self.turn);
    }

    #[allow(dead_code)]
    pub fn one_play(&mut self) -> Self {
        // used for benchmark only
        while !self.is_finished().0 {
            let moves = self.board.search_moves(self.turn);
            let amove = &moves[0];
            self.execute_move(amove);
            let is_finish = self.is_finished();
            if is_finish.0 {
                self.winner = is_finish.1;
                break;
            }
        }
        self.clone()
    }

    #[allow(dead_code)]
    pub fn random_play(&mut self) -> Self {
        while !self.is_finished().0 {
            let moves = self.board.search_moves(self.turn);
            let mut random = Random::new(0, (moves.len() - 1) as u16);
            let amove = &moves[random.generate_one() as usize];
            self.execute_move(amove);
            let is_finish = self.is_finished();
            if is_finish.0 {
                self.winner = is_finish.1;
                break;
            }
        }
        self.clone()
    }

    #[allow(dead_code)]
    pub fn random_move(&mut self, num: usize) -> MctsResult {
        let mut result = MctsResult::new();
        result.next_moves = self.board.search_moves(self.turn);
        result.next_move_count = result.next_moves.len() as u64;
        let copied_game = self.clone();
        for _i in 0..num {
            *self = copied_game.clone();
            let mut next_random = Random::new(0, result.next_move_count as u16);
            let random_one = next_random.generate_one() as usize;
            let next_move = result.next_moves[random_one].clone();
            self.execute_move(&next_move);

            while !self.is_finished().0 {
                let moves = self.board.search_moves(self.turn);
                let move_count = moves.len();
                let mut random = Random::new(0, (move_count - 1) as u16);
                let mv = &moves[random.generate_one() as usize];
                self.execute_move(mv);
                let is_finish = self.is_finished();
                if is_finish.0 {
                    result.plus_result(self.winner, random_one);
                    break;
                }
            }
        }
        result
    }

    pub fn random_move_parallel(&self, num: usize, num_threads: usize) -> MctsResult {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .unwrap();

        let next_moves = self.board.search_moves(self.turn);
        let next_move_count = next_moves.len() as u64;

        if next_move_count == 0 {
            return MctsResult::from(0, vec![]);
        }

        let results: Vec<(ColorType, usize)> = pool.install(|| {
            (0..num)
                .into_par_iter()
                .map(|_| {
                    let mut game_clone = self.clone();
                    let mut next_random = Random::new(0, next_move_count as u16);
                    let random_one = next_random.generate_one() as usize;
                    let next_move = next_moves[random_one].clone();
                    game_clone.execute_move(&next_move);

                    while !game_clone.is_finished().0 {
                        let moves = game_clone.board.search_moves(game_clone.turn);
                        if moves.is_empty() {
                            break;
                        }
                        let move_count = moves.len();
                        let mut random = Random::new(0, (move_count - 1) as u16);
                        let mv = &moves[random.generate_one() as usize];
                        game_clone.execute_move(mv);
                    }
                    let (_is_finished, winner) = game_clone.is_finished();
                    (winner, random_one)
                })
                .collect()
        });

        let mut final_result = MctsResult::from(next_move_count, next_moves);
        for (winner, random_one) in results {
            final_result.plus_result(winner, random_one);
        }
        final_result
    }
}

#[pymethods]
impl Game {
    #[pyo3(name = "random_move")]
    #[pyo3(signature = (num, threads = None))]
    pub fn python_random_move(&self, num: usize, threads: Option<usize>) -> MctsResult {
        let num_threads = threads.unwrap_or_else(num_cpus::get);
        self.random_move_parallel(num, num_threads)
    }

    #[new]
    #[pyo3(signature = (board = Board::new_for_python("startpos".to_string()), move_number = 1, turn = ColorType::Black, winner = ColorType::None))]
    pub fn new_for_python(
        board: Board,
        move_number: u16,
        turn: ColorType,
        winner: ColorType,
    ) -> Self {
        Self {
            board,
            move_number,
            turn,
            winner,
        }
    }

    #[allow(dead_code)]
    #[pyo3(name = "input_board")]
    pub fn python_input_board(&mut self, sfen: String) {
        self.input_board(sfen);
    }

    #[allow(dead_code)]
    #[pyo3(name = "input_hand")]
    pub fn python_input_hand(&mut self, sfen: String) {
        self.input_hand(sfen);
    }

    #[allow(dead_code)]
    #[pyo3(name = "input_move_number")]
    pub fn python_input_move_number(&mut self, sfen: String) {
        self.input_move_number(sfen);
    }

    #[allow(dead_code)]
    #[pyo3(name = "input_turn")]
    pub fn python_input_turn(&mut self, sfen: String) {
        self.input_turn(sfen);
    }

    #[allow(dead_code)]
    #[pyo3(name = "is_finished")]
    pub fn python_is_finished(&self) -> (bool, ColorType) {
        self.is_finished()
    }

    #[allow(dead_code)]
    #[pyo3(name = "execute_move")]
    pub fn python_execute_move(&mut self, moves: &Move) {
        self.execute_move(moves);
    }

    #[allow(dead_code)]
    #[pyo3(name = "random_play")]
    pub fn python_random_play(&mut self) -> Self {
        self.random_play()
    }
}
