use super::address::Address;
use super::piece::{Piece, PROMOTE};
use super::moves::Move;
use super::board::Board;
use super::color::{ColorType, convert_from_string, get_reverse_color};
use super::random::Random;
use super::mctsresult::MctsResult;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub move_number: u16,
    pub turn: ColorType,
    pub winner: ColorType,
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
            winner
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
            let mut chars = part.chars();
            while let Some(ch) = chars.next() {
                if ch.is_digit(10) {
                    let empty_spaces = ch.to_digit(10).unwrap() as usize;
                    column += empty_spaces;
                } else {
                    let piece = Piece::from_char(ch);
                    let piece_type = piece.piece_type;
                    let owner = piece.owner;
                    let index = Address::from_numbers((1 + column) as u8, (9 - row) as u8).to_index();
                    self.board.deploy(index, piece_type, owner);
                    column += 1;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn input_hand(&mut self, sfen: String) {
        if sfen == String::from("-") {
            return;
        }
        let mut current_sfen = sfen.chars();
        while let Some(ch) = current_sfen.next() {
            if ch.is_digit(10) {
                let consecutive = ch.to_digit(10).unwrap() as u8;
                let piece = Piece::from_string(current_sfen.next().unwrap_or_default().to_string());
                self.board.hand.add_pieces(piece.owner, piece.piece_type, consecutive);
            } else {
                let piece = Piece::from_string(ch.to_string());
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
            return (true, ColorType::None)
        } else {
            return self.board.is_finished()
        }
    }

    #[allow(dead_code)]
    pub fn execute_move(&mut self, mv: &Move) {
        self.board.execute_move(mv);
        self.move_number += 1;
        self.turn = get_reverse_color(self.turn);
    }

    #[allow(dead_code)]
    pub fn random_play(&mut self) -> Self {
        while !self.is_finished().0 {
            let moves = self.board.serch_moves(self.turn);
            let mut random = Random::init();
            let amove = &moves[random.generate_one() as usize];
            self.execute_move(amove);
            let is_finish = self.is_finished();
            if is_finish.0 {
                break;
            }
        }
        return self.clone();
    }

    #[allow(dead_code)]
    pub fn random_move(&mut self, num: usize) -> MctsResult {
        let next_moves = self.board.serch_moves(self.turn);
        let mut result = MctsResult::from(next_moves.len() as u64, next_moves);
        let copied_game = self.clone();
        for _i in 0..num{
            *self = copied_game.clone();
            let mut next_move_count = result.next_move_count;
            if next_move_count > 0 {
                next_move_count -= 1;
            }
            let mut next_random = Random::new(0, next_move_count as u16);
            let random_one = next_random.generate_one() as usize;
            let next_move = result.next_moves[random_one].clone();
            self.execute_move(&next_move);

            while !self.is_finished().0 {
                let moves = self.board.serch_moves(self.turn);
                let mut move_count = moves.len();
                if move_count > 0 {
                    move_count -= 1;
                }
                let mut random = Random::new(0, move_count as u16);
                let mv = &moves[random.generate_one() as usize];
                self.execute_move(mv);
                let is_finish = self.is_finished();
                self.winner = is_finish.1;
                if is_finish.0 {
                    result.plus_result(self.winner, random_one);
                    break;
                }
            }
        }
        return result
    }

}