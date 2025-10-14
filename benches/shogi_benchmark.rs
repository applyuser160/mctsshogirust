use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustshogi::bitboard::BitBoard;

fn benchmark_bitboard_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("BitBoard Operations");

    let bb1 = black_box(BitBoard::from_u128(1124249833570304));
    let bb2 = black_box(BitBoard::from_u128(548949983232));

    group.bench_function("bitand", |b| {
        b.iter(|| {
            let _ = bb1 & bb2;
        });
    });

    group.bench_function("bitor", |b| {
        b.iter(|| {
            let _ = bb1 | bb2;
        });
    });

    group.bench_function("get_trues", |b| {
        b.iter(|| {
            let _ = bb1.get_trues();
        });
    });

    // Batch operations benchmarks
    let boards: Vec<BitBoard> = (0..10).map(|i| BitBoard::from_u128(1u128 << i)).collect();
    let boards_slice = black_box(&boards);

    group.bench_function("bitand_batch", |b| {
        b.iter(|| {
            let _ = BitBoard::bitand_batch(boards_slice);
        });
    });

    group.bench_function("bitor_batch", |b| {
        b.iter(|| {
            let _ = BitBoard::bitor_batch(boards_slice);
        });
    });

    group.bench_function("bitxor_batch", |b| {
        b.iter(|| {
            let _ = BitBoard::bitxor_batch(boards_slice);
        });
    });

    group.finish();
}

fn benchmark_game_logic(c: &mut Criterion) {
    use rustshogi::color::ColorType;
    use rustshogi::game::Game;

    let mut group = c.benchmark_group("Game Logic");

    let mut game = Game::new();
    game.input_board("startpos".to_string());
    let board = black_box(game.board);

    group.bench_function("search_moves", |b| {
        b.iter(|| {
            let _ = board.search_moves(ColorType::Black);
        });
    });

    group.bench_function("random_game", |b| {
        b.iter(|| {
            let mut game = Game::new();
            game.input_board("startpos".to_string());
            let _result_game = game.one_play();
        });
    });

    group.finish();
}

fn benchmark_direction(c: &mut Criterion) {
    use rustshogi::direction::Direction;
    let mut group = c.benchmark_group("direction");
    group.bench_function("get_all_direction_vectors", |b| {
        b.iter(|| {
            let _ = Direction::get_all_direction_vectors();
        })
    });
    if cfg!(all(target_arch = "x86_64", target_feature = "sse2")) {
        group.bench_function("get_all_direction_vectors_simd", |b| {
            b.iter(|| unsafe {
                let _ = Direction::get_all_direction_vectors_simd();
            })
        });
    }
    group.finish();
}

fn benchmark_piece(c: &mut Criterion) {
    use rustshogi::piece::{Piece, PieceType};
    let mut group = c.benchmark_group("piece");
    let piece_types = [
        PieceType::Rook,
        PieceType::Bichop,
        PieceType::Silver,
        PieceType::Knight,
        PieceType::Lance,
        PieceType::Pawn,
        PieceType::King,
        PieceType::Gold,
        PieceType::Rook,
        PieceType::Bichop,
        PieceType::Silver,
        PieceType::Knight,
        PieceType::Lance,
        PieceType::Pawn,
        PieceType::King,
        PieceType::Gold,
    ];

    group.bench_function("able_pro_batch", |b| {
        b.iter(|| {
            let _ = Piece::able_pro_batch(&piece_types);
        })
    });

    if cfg!(all(target_arch = "x86_64", target_feature = "sse2")) {
        group.bench_function("able_pro_batch_simd", |b| {
            b.iter(|| unsafe {
                let _ = Piece::able_pro_batch_simd(&piece_types);
            })
        });
    }
    group.finish();
}

fn benchmark_random(c: &mut Criterion) {
    use rustshogi::random::Random;
    let mut group = c.benchmark_group("random");
    let mut rng = Random::new(1, 100);
    let len: u16 = 1024;

    group.bench_function("generate_multi", |b| {
        b.iter(|| {
            let _ = rng.generate_multi(len);
        })
    });

    group.bench_function("generate_multi_fast", |b| {
        b.iter(|| {
            let _ = rng.generate_multi_fast(len);
        })
    });

    if cfg!(all(target_arch = "x86_64", target_feature = "sse2")) {
        group.bench_function("generate_multi_sse2", |b| {
            b.iter(|| unsafe {
                let _ = rng.generate_multi_sse2(len);
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_bitboard_operations,
    benchmark_game_logic,
    benchmark_direction,
    benchmark_piece,
    benchmark_random
);
criterion_main!(benches);
