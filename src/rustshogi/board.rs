use super::address::Address;
use super::bitboard::{
    generate_column, BitBoard, BIT_OF_FRAME, BIT_OF_LAST1_ZONE_BLACK, BIT_OF_LAST1_ZONE_WHITE,
    BIT_OF_LAST2_ZONE_BLACK, BIT_OF_LAST2_ZONE_WHITE, BIT_OF_PRO_ZONE_BLACK, BIT_OF_PRO_ZONE_WHITE,
    LENGTH_OF_EDGE, LENGTH_OF_FRAME,
};
use super::color::{get_reverse_color, ColorType};
use super::direction::{Direction, DirectionName};
use super::hand::Hand;
use super::moves::Move;
use super::piece::{MoveType, Piece, PieceType, PROMOTE};

use pyo3::prelude::*;

#[allow(dead_code)]
#[pyclass]
#[derive(Clone, PartialEq, Debug)]
pub struct Board {
    pieces: [Piece; 121],
    pub has_piece: BitBoard,
    pub player_prossesion: [BitBoard; ColorType::ColorNumber as usize],
    pub is_frame: BitBoard,
    pub able_pro: [BitBoard; ColorType::ColorNumber as usize],
    pub last_one: [BitBoard; ColorType::ColorNumber as usize],
    pub last_two: [BitBoard; ColorType::ColorNumber as usize],
    pub hand: Hand,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    #[allow(dead_code)]
    fn drop(&mut self, index: u8) {
        self.has_piece.set(index as usize, false);
        for player_prosession in self.player_prossesion.iter_mut() {
            player_prosession.set(index as usize, false);
        }
        self.pieces[index as usize] = Piece::new();
    }

    #[allow(dead_code)]
    fn move_standard(&mut self, from_index: u8, to_index: u8) {
        let piece_type = self.get_piece_type_from_index(from_index);
        let color_type = self.get_color_type_from_index(from_index);
        self.drop(from_index);
        self.deploy(to_index, piece_type, color_type);
    }

    #[allow(dead_code)]
    fn move_to_hand(&mut self, index: u8, is_color_reverse: bool) {
        let mut piece_type = self.get_piece_type_from_index(index);
        if piece_type as u8 > PROMOTE {
            piece_type = PieceType::from_usize(piece_type as usize - PROMOTE as usize);
        }

        let mut color_type = self.get_color_type_from_index(index);
        if is_color_reverse {
            color_type = get_reverse_color(color_type);
        }

        self.drop(index);
        self.hand.add_piece(color_type, piece_type);
    }

    #[allow(dead_code)]
    fn move_from_hand(&mut self, index: u8, piece_type: PieceType, color: ColorType) {
        self.hand.decrease_piece(color, piece_type);
        self.deploy(index, piece_type, color);
    }

    pub fn new() -> Self {
        Self {
            pieces: [Piece::new(); 121],
            has_piece: BitBoard::new(),
            player_prossesion: [BitBoard::new(), BitBoard::new()],
            is_frame: BitBoard::from_u128(BIT_OF_FRAME),
            able_pro: [
                BitBoard::from_u128(BIT_OF_PRO_ZONE_BLACK),
                BitBoard::from_u128(BIT_OF_PRO_ZONE_WHITE),
            ],
            last_one: [
                BitBoard::from_u128(BIT_OF_LAST1_ZONE_BLACK),
                BitBoard::from_u128(BIT_OF_LAST1_ZONE_WHITE),
            ],
            last_two: [
                BitBoard::from_u128(BIT_OF_LAST2_ZONE_BLACK),
                BitBoard::from_u128(BIT_OF_LAST2_ZONE_WHITE),
            ],
            hand: Hand::new(),
        }
    }

    #[allow(dead_code)]
    pub fn deploy(&mut self, index: u8, piece_type: PieceType, color: ColorType) {
        self.pieces[index as usize] = Piece::from(color, piece_type);
        self.has_piece.set(index as usize, true);
        for (i, player_prossesion) in self.player_prossesion.iter_mut().enumerate() {
            player_prossesion.set(index as usize, color == ColorType::from_u8(i as u8));
        }
    }

    #[allow(dead_code)]
    pub fn startpos(&mut self) {
        self.deploy(
            Address::from_numbers(1, 1).to_index(),
            PieceType::Lance,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(2, 1).to_index(),
            PieceType::Knight,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(3, 1).to_index(),
            PieceType::Silver,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(4, 1).to_index(),
            PieceType::Gold,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(5, 1).to_index(),
            PieceType::King,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(6, 1).to_index(),
            PieceType::Gold,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(7, 1).to_index(),
            PieceType::Silver,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(8, 1).to_index(),
            PieceType::Knight,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(9, 1).to_index(),
            PieceType::Lance,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(2, 2).to_index(),
            PieceType::Bishop,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(8, 2).to_index(),
            PieceType::Rook,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(1, 3).to_index(),
            PieceType::Pawn,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(2, 3).to_index(),
            PieceType::Pawn,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(3, 3).to_index(),
            PieceType::Pawn,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(4, 3).to_index(),
            PieceType::Pawn,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(5, 3).to_index(),
            PieceType::Pawn,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(6, 3).to_index(),
            PieceType::Pawn,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(7, 3).to_index(),
            PieceType::Pawn,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(8, 3).to_index(),
            PieceType::Pawn,
            ColorType::Black,
        );
        self.deploy(
            Address::from_numbers(9, 3).to_index(),
            PieceType::Pawn,
            ColorType::Black,
        );

        self.deploy(
            Address::from_numbers(1, 9).to_index(),
            PieceType::Lance,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(2, 9).to_index(),
            PieceType::Knight,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(3, 9).to_index(),
            PieceType::Silver,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(4, 9).to_index(),
            PieceType::Gold,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(5, 9).to_index(),
            PieceType::King,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(6, 9).to_index(),
            PieceType::Gold,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(7, 9).to_index(),
            PieceType::Silver,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(8, 9).to_index(),
            PieceType::Knight,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(9, 9).to_index(),
            PieceType::Lance,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(8, 8).to_index(),
            PieceType::Bishop,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(2, 8).to_index(),
            PieceType::Rook,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(1, 7).to_index(),
            PieceType::Pawn,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(2, 7).to_index(),
            PieceType::Pawn,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(3, 7).to_index(),
            PieceType::Pawn,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(4, 7).to_index(),
            PieceType::Pawn,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(5, 7).to_index(),
            PieceType::Pawn,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(6, 7).to_index(),
            PieceType::Pawn,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(7, 7).to_index(),
            PieceType::Pawn,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(8, 7).to_index(),
            PieceType::Pawn,
            ColorType::White,
        );
        self.deploy(
            Address::from_numbers(9, 7).to_index(),
            PieceType::Pawn,
            ColorType::White,
        );

        self.hand = Hand::new();
    }

    #[allow(dead_code)]
    pub fn input_board(&mut self, sfen: &str) {
        let startpos = String::from("startpos");
        if startpos == sfen {
            self.startpos();
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
                    self.deploy(index, piece_type, owner);
                    column += 1;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn input_hand(&mut self, sfen: &str) {
        if sfen == "-" {
            return;
        }
        let mut current_sfen = sfen.chars();
        while let Some(ch) = current_sfen.next() {
            if ch.is_ascii_digit() {
                let consecutive = ch.to_digit(10).unwrap() as u8;
                let piece = Piece::from_char(current_sfen.next().unwrap());
                self.hand
                    .add_pieces(piece.owner, piece.piece_type, consecutive);
            } else {
                let piece = Piece::from_char(ch);
                self.hand.add_piece(piece.owner, piece.piece_type);
            }
        }
    }

    #[allow(dead_code)]
    pub fn from_sfen(sfen: String) -> Self {
        let mut board = Self::new();
        let parts: Vec<&str> = sfen.split(" ").collect();
        board.input_board(parts[0]);
        board.input_hand(parts[1]);
        board
    }

    #[allow(dead_code)]
    pub fn get_piece_type_from_index(&self, index: u8) -> PieceType {
        self.pieces[index as usize].piece_type
    }

    #[allow(dead_code)]
    pub fn get_color_type_from_index(&self, index: u8) -> ColorType {
        self.pieces[index as usize].owner
    }

    #[allow(dead_code)]
    pub fn get_piece(&self, index: u8) -> Piece {
        self.pieces[index as usize]
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for row in (1..=LENGTH_OF_EDGE).rev() {
            if row < 9 {
                result.push('/');
            }
            let mut empty_count = 0;
            for col in 1..=LENGTH_OF_EDGE {
                let index = Address::from_numbers(col, row).to_index();
                let piece = self.get_piece(index);
                if piece.piece_type != PieceType::None {
                    if empty_count > 0 {
                        result.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    let piece_char = piece.to_string();
                    result.push_str(&piece_char);
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                result.push_str(&empty_count.to_string());
            }
        }
        result
    }

    #[allow(dead_code)]
    pub fn get_able_move_squares(&self, index: u8) -> BitBoard {
        let piece_type = self.get_piece_type_from_index(index);
        let color_type = self.get_color_type_from_index(index);

        let move_types = Piece::get_movetype(piece_type);
        let mut bit_movable = BitBoard::new();
        let start_pos_bb = BitBoard::from_u128(1 << (120 - index));

        for (j, &move_type) in move_types.iter().enumerate() {
            if move_type == MoveType::None {
                continue;
            }

            let mut direction = Direction::new(DirectionName::from_usize(j));
            if color_type == ColorType::White {
                direction.reverse();
            }

            let v_step = direction.vertical_vector;
            let h_step = direction.horizon_vector;

            if move_type == MoveType::Hop {
                let mut up = Direction::new(DirectionName::Up);
                if color_type == ColorType::White {
                    up.reverse();
                }
                let shift_hop = LENGTH_OF_FRAME as i8 * (v_step * 2 + up.vertical_vector) + h_step * 2;
                let shift_val = shift_hop.abs() as usize;
                let mut current_pos = start_pos_bb;

                if shift_hop > 0 {
                    current_pos >>= shift_val;
                } else {
                    current_pos <<= shift_val;
                }

                if !(&self.is_frame & &current_pos).any() && !(&self.player_prossesion[color_type as usize] & &current_pos).any() {
                    bit_movable |= current_pos;
                }
                continue;
            }

            let shift_step = LENGTH_OF_FRAME as i8 * v_step + h_step;
            let mut current_pos = start_pos_bb;

            for _ in 0..LENGTH_OF_EDGE {
                let shift_val = shift_step.abs() as usize;
                if shift_step > 0 {
                    current_pos >>= shift_val;
                } else {
                    current_pos <<= shift_val;
                }

                if (&self.is_frame & &current_pos).any() {
                    break;
                }
                if (&self.player_prossesion[color_type as usize] & &current_pos).any() {
                    break;
                }

                bit_movable |= current_pos;

                if (&self.player_prossesion[get_reverse_color(color_type) as usize] & &current_pos).any() {
                    break;
                }
                if move_type != MoveType::Long {
                    break;
                }
            }
        }
        bit_movable
    }

    #[allow(dead_code)]
    pub fn get_able_pro_move_squares(&self, index: u8, bit_movable: BitBoard) -> BitBoard {
        let result = BitBoard::new();
        let piece_type = self.get_piece_type_from_index(index);
        let color_type = self.get_color_type_from_index(index);
        let mut bit_board = BitBoard::new();
        bit_board.set(index as usize, true);
        let is_able_pro = Piece::able_pro(piece_type);
        if !is_able_pro {
            return result;
        }

        let pro_area = &self.able_pro[color_type as usize];

        if (&bit_board & pro_area).any() {
            return bit_movable;
        }

        &bit_movable & pro_area
    }

    #[allow(dead_code)]
    pub fn get_able_drop_squares(&self, color: ColorType, piece_type: PieceType) -> BitBoard {
        let mut none = BitBoard::new();
        for i in 0..121 {
            if self.pieces[i].piece_type == PieceType::None && !self.is_frame.get(i) {
                none.set(i, true);
            }
        }
        let last_not_two = !self.last_two[color as usize];
        let last_not_one = !self.last_one[color as usize];

        match piece_type {
            PieceType::Gold => none,
            PieceType::Rook => none,
            PieceType::Bishop => none,
            PieceType::Silver => none,
            PieceType::Knight => &none & &last_not_two,
            PieceType::Lance => &none & &last_not_one,
            PieceType::Pawn => {
                let mut pawn = BitBoard::new();
                for (i, piece) in self.pieces.iter().enumerate() {
                    if piece.piece_type == PieceType::Pawn && piece.owner == color {
                        pawn.set(i, true);
                    }
                }
                let pawn_indexs = pawn.get_trues();
                let mut double_pawn = BitBoard::new();

                for index in pawn_indexs {
                    double_pawn |= generate_column((index % LENGTH_OF_FRAME) as usize);
                }

                let not_double_pawn = !double_pawn;

                &none & &(&last_not_one & &not_double_pawn)
            }
            _ => BitBoard::new(),
        }
    }

    #[allow(dead_code)]
    pub fn search_moves(&self, color: ColorType) -> Vec<Move> {
        let mut vector_move: Vec<Move> = Vec::new();

        let player_board = if color == ColorType::White {
            &self.player_prossesion[ColorType::White as usize]
        } else {
            &self.player_prossesion[ColorType::Black as usize]
        };

        let player_board_indexs = player_board.get_trues();

        for player_board_index in player_board_indexs.iter() {
            let move_board = self.get_able_move_squares(*player_board_index);
            let move_indexs = move_board.get_trues();
            vector_move.extend(move_indexs.iter().map(|move_index| {
                Move::from_standart(
                    Address::from_number(*player_board_index),
                    Address::from_number(*move_index),
                    false,
                )
            }));
            drop(move_indexs);

            let pro_board = self.get_able_pro_move_squares(*player_board_index, move_board);
            let move_indexs = pro_board.get_trues();
            vector_move.extend(move_indexs.iter().map(|move_index| {
                Move::from_standart(
                    Address::from_number(*player_board_index),
                    Address::from_number(*move_index),
                    true,
                )
            }));
        }

        let player_hand_pieces = self.hand.get_player_pieces(color);
        for player_hand_piece in player_hand_pieces.iter() {
            let move_board =
                self.get_able_drop_squares(player_hand_piece.owner, player_hand_piece.piece_type);
            let move_indexs = move_board.get_trues();
            vector_move.extend(move_indexs.iter().map(|move_index| {
                Move::from_drop(*player_hand_piece, Address::from_number(*move_index))
            }));
        }

        vector_move
    }

    #[allow(dead_code)]
    pub fn execute_move(&mut self, moves: &Move) {
        let is_drop = moves.get_is_drop();
        let to_index = moves.get_to().to_index();
        let mut piece = Piece::new();
        let mut from_index: u8 = 0;

        if is_drop {
            piece = moves.get_piece();
        } else {
            from_index = moves.get_from().to_index();
        }

        if self.has_piece.get(to_index as usize) {
            self.move_to_hand(to_index, true);
        }

        if is_drop {
            self.move_from_hand(to_index, piece.piece_type, piece.owner);
        } else {
            self.move_standard(from_index, to_index);
        }
    }

    #[allow(dead_code)]
    pub fn is_finished(&self) -> (bool, ColorType) {
        let mut king_count = 0;
        let mut black_king_exists = false;
        for piece in self.pieces.iter() {
            if piece.piece_type == PieceType::King {
                king_count += 1;
                if piece.owner == ColorType::Black {
                    black_king_exists = true;
                }
            }
        }

        if king_count < 2 {
            if black_king_exists {
                (true, ColorType::Black)
            } else {
                (true, ColorType::White)
            }
        } else {
            (false, ColorType::None)
        }
    }
}

#[pymethods]
impl Board {
    #[new]
    #[pyo3(signature = (sfen = "startpos".to_string()))]
    pub fn new_for_python(sfen: String) -> Self {
        Self::from_sfen(sfen)
    }

    pub fn __repr__(&self) -> String {
        format!("Board(sfen={})", self.to_string())
    }

    pub fn __str__(&self) -> String {
        format!("Board(sfen={})", self.to_string())
    }

    pub fn __eq__(&self, other: &Self) -> bool {
        self.pieces.iter().zip(other.pieces.iter()).all(|(a, b)| a == b)
            && self.has_piece == other.has_piece
            && self.player_prossesion == other.player_prossesion
            && self.is_frame == other.is_frame
            && self.able_pro == other.able_pro
            && self.last_one == other.last_one
            && self.last_two == other.last_two
            && self.hand == other.hand
    }

    pub fn __ne__(&self, other: &Self) -> bool {
        !self.__eq__(other)
    }

    #[allow(dead_code)]
    #[pyo3(name = "deploy")]
    pub fn python_deploy(&mut self, address: &Address, piece_type: PieceType, color: ColorType) {
        self.deploy(address.to_index(), piece_type, color);
    }

    #[allow(dead_code)]
    #[pyo3(name = "startpos")]
    pub fn python_startpos(&mut self) {
        self.startpos();
    }

    #[allow(dead_code)]
    #[pyo3(name = "get_piece")]
    pub fn python_get_piece(&self, address: &Address) -> Piece {
        self.get_piece(address.to_index())
    }

    #[allow(dead_code)]
    #[pyo3(name = "search_moves")]
    pub fn python_search_moves(&self, color: ColorType) -> Vec<Move> {
        self.search_moves(color)
    }

    #[allow(dead_code)]
    #[pyo3(name = "execute_move")]
    pub fn python_execute_move(&mut self, moves: &Move) {
        self.execute_move(moves);
    }

    #[allow(dead_code)]
    #[pyo3(name = "is_finished")]
    pub fn python_is_finished(&self) -> (bool, ColorType) {
        self.is_finished()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
