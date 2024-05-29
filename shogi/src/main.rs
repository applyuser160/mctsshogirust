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
mod test_address;
mod test_bitboard;
mod test_board;
mod test_common;
mod test_direction;
mod test_random;
mod test_color;
mod test_piece;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    // let bites = bitboard::BIT_OF_FRAME.to_be_bytes();
    // println!("{:?}", bites);
    
    // let mut s = String::with_capacity(3);
    // println!("{}", s);
    // s.push('a');
    // s.push('b');
    // s.push('c');
    // s.push('d');
    // println!("{}", s);
    
    // let n: usize = 1;
    // let w = piece::PieceType::from_usize(n);
    // println!("{}", w as u8);

    // let string = String::from("abc");
    // println!("{}", string);
    // let vec = string.chars().collect::<Vec<char>>();
    // println!("{:?}", vec);

    // let b = board::Board::new();
    // println!("{:?}", b.has_specific_piece[0].board);

    // let sfen = String::from("w");
    // let sfen_n = color::convert_from_string(sfen.chars().next().unwrap_or(' '));
    // println!("{}", sfen_n as u8);

    /* */
    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];

    // // counterが何回使われてるかを表示します
    // println!("count first: {}", Arc::strong_count(&counter));

    // for i in 0..3 {
    //     let counter_arc = Arc::clone(&counter);
    //     println!("count: {}", Arc::strong_count(&counter));
    //     let handle = thread::spawn(move || {
    //         println!("Thread {} attempting to acquire lock...", i);
    //         let mut num = counter_arc.lock().unwrap();
    //         println!("Thread {} acquired lock", i);

    //         *num += 1;
    //         println!("Thread {} incremented counter to {}", i, *num);

    //         // スレッドがロックを持っている間に少しスリープして、
    //         // 他のスレッドがロックを待っていることを示します
    //         thread::sleep(Duration::from_millis(50));

    //         println!("Thread {} releasing lock", i);
    //     });
    //     handles.push(handle);

    // }

    // for handle in handles {
	// // join()でスレッドが終わるのを待つ
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());
    /* */

/* main */
    // let mut game = game::Game::new();
    // let mut result = mctsresult::MctsResult::new();

    // game.input_board(String::from("startpos"));
    // game.input_turn(String::from("b"));
    // game.input_hand(String::from("-"));
    // game.input_move_number(String::from("1"));
    // result.next_moves = game.board.serch_moves(game.turn);
    // result.next_move_count = result.next_moves.len() as u64;

    // result = game.random_move(10);
    
    // let from = Address::from_number(92);
    // println!("{}", from.to_string());
    // let to = Address::from_number(73);
    // println!("{}", to.to_string());
    // let amove = Move::from_standart(from, to, false);
    // println!("{}", amove);

    let mut board = board::Board::new();
    board.deploy(78, piece::PieceType::Gold, color::ColorType::White);

    let next_moves = board.serch_moves(color::ColorType::White);
    println!("next moves:");
    for i in next_moves {
        println!("{}", i);
    }

/* main */

    let end = start.elapsed();
    println!("{}.{:06}秒", end.as_secs(), end.as_micros());

}
