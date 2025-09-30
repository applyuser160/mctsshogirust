use super::address::Address;
use super::board::Board;
use super::color::{convert_from_string, get_reverse_color, ColorType};
use super::mctsresult::MctsResult;
use super::moves::Move;
use super::piece::Piece;
use super::random::Random;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub move_number: u16,
    pub turn: ColorType,
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
            let mut random = Random::init();
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
}
