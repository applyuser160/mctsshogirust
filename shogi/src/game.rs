
use super::address::Address;
use super::piece::{Piece, PROMOTE};
use super::moves::Move;
use super::board::Board;
use super::color::{ColorType, convert_from_string, get_reverse_color};
use super::random::Random;

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

        let mut current_sfen = sfen.chars();
        let mut consecutive = 0;
        for row in 1..9 {
            for column in (1..9).rev() {
                if let Some(ch) = current_sfen.next() {
                    if ch.is_digit(10) {
                        consecutive = ch.to_digit(10).unwrap() as i32;
                    }
                    if consecutive > 0 {
                        consecutive -= 1;
                        continue;
                    }
                    if ch == '/' {
                        continue;
                    }
                    let index = Address::from_numbers(column as u8, row as u8).to_index();
                    let piece = Piece::from_string(ch.to_string());
                    self.board.deploy(index, piece.piece_type, piece.owner);
                    if piece.piece_type as i32 > PROMOTE as i32 {
                        current_sfen.next();
                    }
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

    // #[allow(dead_code)]
    // pub fn random_move(&mut self, )
}