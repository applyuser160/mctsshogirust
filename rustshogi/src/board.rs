use bitvec::prelude::*;

use super::bitboard::{BitBoard, BIT_OF_FRAME, BIT_OF_PRO_ZONE_BLACK, BIT_OF_PRO_ZONE_WHITE,
    BIT_OF_LAST1_ZONE_BLACK, BIT_OF_LAST1_ZONE_WHITE, BIT_OF_LAST2_ZONE_BLACK, BIT_OF_LAST2_ZONE_WHITE,
    generate_column, LENGTH_OF_EDGE, LENGTH_OF_FRAME};
use super::color::{ColorType, get_reverse_color};
use super::hand::Hand;
use super::piece::{Piece, PieceType, PIECE_TYPE_NUMBER, MoveType, PROMOTE};
use super::address::Address;
use super::direction::{DirectionName, Direction};
use super::moves::Move;

#[allow(dead_code)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Board {
    pub has_piece: BitBoard,
    pub player_prossesion: [BitBoard; ColorType::ColorNumber as usize],
    pub is_frame: BitBoard,
    pub able_pro: [BitBoard; ColorType::ColorNumber as usize],
    pub last_one: [BitBoard; ColorType::ColorNumber as usize],
    pub last_two: [BitBoard; ColorType::ColorNumber as usize],
    pub has_specific_piece: [BitBoard; PIECE_TYPE_NUMBER as usize],
    pub hand: Hand,
}

impl Board {
    #[allow(dead_code)]
    fn is_a_has_specific_piece(&self, index: u8, piece_type: PieceType) -> bool {
        return self.has_specific_piece[piece_type as usize].board[index as usize]
    }

    #[allow(dead_code)]
    fn drop(&mut self, index: u8) {
        self.has_piece.board.set(index as usize, false);
        for i in 0..ColorType::ColorNumber as usize {
            self.player_prossesion[i].board.set(index as usize, false);
        }
        self.has_specific_piece[PieceType::None as usize].board.set(index as usize, true);
        for i in 1..PIECE_TYPE_NUMBER as usize {
            self.has_specific_piece[i].board.set(index as usize, false);
        }
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
        let is_frame = BitBoard::from_u128(BIT_OF_FRAME);
        let mut has_specific_piece = BitBoard::from_u128(BIT_OF_FRAME);
        has_specific_piece.flip();
        Self {
            has_piece: BitBoard::new(),
            player_prossesion: [BitBoard::new(), BitBoard::new()],
            is_frame,
            able_pro: [BitBoard::from_u128(BIT_OF_PRO_ZONE_BLACK),
                BitBoard::from_u128(BIT_OF_PRO_ZONE_WHITE),],
            last_one: [BitBoard::from_u128(BIT_OF_LAST1_ZONE_BLACK),
                BitBoard::from_u128(BIT_OF_LAST1_ZONE_WHITE),],
            last_two: [BitBoard::from_u128(BIT_OF_LAST2_ZONE_BLACK),
                BitBoard::from_u128(BIT_OF_LAST2_ZONE_WHITE),],
            has_specific_piece: [has_specific_piece, BitBoard::new(), BitBoard::new(),
                BitBoard::new(), BitBoard::new(), BitBoard::new(), BitBoard::new(),
                BitBoard::new(), BitBoard::new(), BitBoard::new(), BitBoard::new(),
                BitBoard::new(), BitBoard::new(), BitBoard::new(), BitBoard::new(),],
            hand: Hand::new(),
        }
    }

    #[allow(dead_code)]
    pub fn deploy(&mut self, index: u8, piece_type: PieceType, color: ColorType) {
        self.has_piece.board.set(index as usize, true);
        for i in 0..ColorType::ColorNumber as usize {
            self.player_prossesion[i].board.set(index as usize, color == ColorType::from_u8(i as u8));
        }
        for i in 0..PIECE_TYPE_NUMBER as usize {
            self.has_specific_piece[i].board.set(index as usize, piece_type == PieceType::from_usize(i));
        }
    }

    #[allow(dead_code)]
    pub fn startpos(&mut self) {
        self.deploy(Address::from_numbers(1, 1).to_index(), PieceType::Lance, ColorType::Black);
        self.deploy(Address::from_numbers(2, 1).to_index(), PieceType::Knight, ColorType::Black);
        self.deploy(Address::from_numbers(3, 1).to_index(), PieceType::Silver, ColorType::Black);
        self.deploy(Address::from_numbers(4, 1).to_index(), PieceType::Gold, ColorType::Black);
        self.deploy(Address::from_numbers(5, 1).to_index(), PieceType::King, ColorType::Black);
        self.deploy(Address::from_numbers(6, 1).to_index(), PieceType::Gold, ColorType::Black);
        self.deploy(Address::from_numbers(7, 1).to_index(), PieceType::Silver, ColorType::Black);
        self.deploy(Address::from_numbers(8, 1).to_index(), PieceType::Knight, ColorType::Black);
        self.deploy(Address::from_numbers(9, 1).to_index(), PieceType::Lance, ColorType::Black);
        self.deploy(Address::from_numbers(2, 2).to_index(), PieceType::Bichop, ColorType::Black);
        self.deploy(Address::from_numbers(8, 2).to_index(), PieceType::Rook, ColorType::Black);
        self.deploy(Address::from_numbers(1, 3).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(2, 3).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(3, 3).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(4, 3).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(5, 3).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(6, 3).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(7, 3).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(8, 3).to_index(), PieceType::Pawn, ColorType::Black);
        self.deploy(Address::from_numbers(9, 3).to_index(), PieceType::Pawn, ColorType::Black);

        self.deploy(Address::from_numbers(1, 9).to_index(), PieceType::Lance, ColorType::White);
        self.deploy(Address::from_numbers(2, 9).to_index(), PieceType::Knight, ColorType::White);
        self.deploy(Address::from_numbers(3, 9).to_index(), PieceType::Silver, ColorType::White);
        self.deploy(Address::from_numbers(4, 9).to_index(), PieceType::Gold, ColorType::White);
        self.deploy(Address::from_numbers(5, 9).to_index(), PieceType::King, ColorType::White);
        self.deploy(Address::from_numbers(6, 9).to_index(), PieceType::Gold, ColorType::White);
        self.deploy(Address::from_numbers(7, 9).to_index(), PieceType::Silver, ColorType::White);
        self.deploy(Address::from_numbers(8, 9).to_index(), PieceType::Knight, ColorType::White);
        self.deploy(Address::from_numbers(9, 9).to_index(), PieceType::Lance, ColorType::White);
        self.deploy(Address::from_numbers(8, 8).to_index(), PieceType::Bichop, ColorType::White);
        self.deploy(Address::from_numbers(2, 8).to_index(), PieceType::Rook, ColorType::White);
        self.deploy(Address::from_numbers(1, 7).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(2, 7).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(3, 7).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(4, 7).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(5, 7).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(6, 7).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(7, 7).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(8, 7).to_index(), PieceType::Pawn, ColorType::White);
        self.deploy(Address::from_numbers(9, 7).to_index(), PieceType::Pawn, ColorType::White);

        self.hand = Hand::new();
    }

    #[allow(dead_code)]
    pub fn get_piece_type_from_index(&self, index: u8) -> PieceType {
        for i in 0..PIECE_TYPE_NUMBER as usize {
            if self.is_a_has_specific_piece(index, PieceType::from_usize(i)) {
                return PieceType::from_usize(i)
            }
        }
        return PieceType::None
    }

    #[allow(dead_code)]
    pub fn get_color_type_from_index(&self, index: u8) -> ColorType {
        let has_a_piece = self.has_piece.board[index as usize];
        let is_black = self.player_prossesion[ColorType::Black as usize].board[index as usize];
        if !has_a_piece {
            return ColorType::None
        } else if is_black {
            return ColorType::Black
        } else {
            return ColorType::White
        }
    }

    #[allow(dead_code)]
    pub fn get_piece(&self, index: u8) -> Piece {
        let piece_type = self.get_piece_type_from_index(index);
        let color_type = self.get_color_type_from_index(index);
        let piece = Piece::from(color_type, piece_type);
        return piece
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for row in (1..=9).rev() {
            if row < 9 {
                result.push('/');
            }
            let mut empty_count = 0;
            for col in 1..=9 {
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

        let move_types: [MoveType; DirectionName::DirectionNameNumber as usize] = Piece::get_movetype(piece_type);
        let mut is_in_board = bitvec![1; DirectionName::DirectionNameNumber as usize];

        let mut bit_board = BitBoard::new();
        bit_board.board.set(index as usize, true);
        let mut bit_movable = BitBoard::new();

        for i in 1..LENGTH_OF_EDGE {
            for j in 0..DirectionName::DirectionNameNumber as usize {
                if move_types[j] == MoveType::None || !is_in_board[j] {
                    continue;
                }

                let mut direction = Direction::new(DirectionName::from_usize(j));
                let mut up = Direction::new(DirectionName::Up);
                if color_type == ColorType::White {
                    direction.reverse();
                    up.reverse();
                }

                let mut v = direction.vertical_vector * i as i8;
                let h = direction.horizon_vector * i as i8;

                if move_types[j] == MoveType::Hop {
                    v += up.vertical_vector;
                }

                let shift_number = LENGTH_OF_FRAME as i8 * v + h;

                let bn1 = if shift_number > 0 { 
                    bit_board.clone() << shift_number as usize
                } else {
                    bit_board.clone() >> shift_number.abs() as usize
                };

                if (self.is_frame.clone() & bn1.clone()).board.any() {
                    is_in_board.set(j, false);
                } else if (self.player_prossesion[get_reverse_color(color_type) as usize].clone() & bn1.clone()).board.any() {
                    is_in_board.set(j, false);
                    bit_movable = bit_movable.clone() | bn1.clone();
                } else if (self.player_prossesion[color_type as usize].clone() & bn1.clone()).board.any() {
                    is_in_board.set(j, false);
                } else {
                    bit_movable = bit_movable | bit_board.clone() | bn1.clone();
                }

                if move_types[j] != MoveType::Long {
                    is_in_board.set(j, false);
                }
            }
        }
        bit_movable.board.set(index as usize, false);
        return bit_movable
    }

    #[allow(dead_code)]
    pub fn get_able_pro_move_squares(&self, index: u8, bit_movable: BitBoard) -> BitBoard {
        let result = BitBoard::new();
        let piece_type = self.get_piece_type_from_index(index);
        let color_type = self.get_color_type_from_index(index);
        let mut bit_board = BitBoard::new();
        bit_board.board.set(index as usize, true);
        let is_able_pro = Piece::able_pro(piece_type);
        if !is_able_pro {
            return result
        }

        let pro_area = self.able_pro[color_type as usize].clone();

        if (bit_board & pro_area.clone()).board.any() {
            return bit_movable
        }

        return bit_movable & pro_area
    }

    #[allow(dead_code)]
    pub fn get_able_drop_squares(&self, color: ColorType, piece_type: PieceType) -> BitBoard {
        let none = self.has_specific_piece[PieceType::None as usize].clone();
        let last_two = self.last_two[color as usize].clone();
        let last_one = self.last_one[color as usize].clone();

        let mut last_not_two = last_two;
        let mut last_not_one = last_one;
        last_not_two.flip();
        last_not_one.flip();

        match piece_type {
            PieceType::Gold => none,
            PieceType::Rook => none,
            PieceType::Bichop => none,
            PieceType::Silver => none,
            PieceType::Knight => none & last_not_two,
            PieceType::Lance => none & last_not_one,
            PieceType::Pawn => {
                let pawn = self.has_specific_piece[PieceType::Pawn as usize].clone() & self.player_prossesion[color as usize].clone();
                let pawn_indexs = pawn.get_trues();
                let mut double_pawn = BitBoard::new();

                for i in 0..pawn_indexs.len() {
                    double_pawn = double_pawn | generate_column(Address::from_number(pawn_indexs[i]).column as usize);
                }

                let mut not_double_pawn = double_pawn;
                not_double_pawn.flip();

                return none & last_not_one & not_double_pawn
            },
            _ => BitBoard::new()
        }
    }

    #[allow(dead_code)]
    pub fn serch_moves(&self, color: ColorType) -> Vec<Move> {
        let mut vector_move: Vec<Move> = Vec::new();

        let player_board = if color.to_bool() 
            { self.player_prossesion[ColorType::White as usize].clone() } 
            else { self.player_prossesion[ColorType::Black as usize].clone() };
        
        let player_board_indexs = player_board.get_trues();

        for i in 0..player_board_indexs.len() {

            let move_board = self.get_able_move_squares(player_board_indexs[i]);
            let move_indexs = move_board.get_trues();
            for j in 0..move_indexs.len() {
                let from = Address::from_number(player_board_indexs[i]);
                let to = Address::from_number(move_indexs[j]);
                let moves = Move::from_standart(from, to, false);
                vector_move.push(moves);
            }
            drop(move_indexs);

            let pro_board = self.get_able_pro_move_squares(player_board_indexs[i], move_board);
            let move_indexs = pro_board.get_trues();
            for j in 0..move_indexs.len() {
                let from = Address::from_number(player_board_indexs[i]);
                let to = Address::from_number(move_indexs[j]);
                let moves = Move::from_standart(from, to, true);
                vector_move.push(moves);
            }
        }

        let player_hand_pieces = self.hand.get_player_pieces(color);
        for i in 0..player_hand_pieces.len() {
            let move_board = self.get_able_drop_squares(player_hand_pieces[i].owner, player_hand_pieces[i].piece_type);
            let move_indexs = move_board.get_trues();
            for j in 0..move_indexs.len() {
                let to = Address::from_number(move_indexs[j]);
                let moves = Move::from_drop(player_hand_pieces[i], to);
                vector_move.push(moves);
            }
        }

        return vector_move
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

        if self.has_piece.board[to_index as usize] {
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
        let winner;
        let is_finish = self.has_specific_piece[PieceType::King as usize].board.count_ones() != ColorType::ColorNumber as usize;
        // println!("is_finish:{}", is_finish);
        if is_finish {
            let is_black_win = (self.has_specific_piece[PieceType::King as usize].clone() & self.player_prossesion[ColorType::Black as usize].clone()).board.any();
            if is_black_win {
                winner = ColorType::Black;
            } else {
                winner = ColorType::White;
            }
        } else {
            winner = ColorType::None;
        }
        return (is_finish, winner)
    }
}


impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}