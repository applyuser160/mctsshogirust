use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_bitboard(c: &mut Criterion) {
    use rustshogi::bitboard::BitBoard;
    c.bench_function("bitbord", |b| {
        b.iter(|| {
            let bb1 = BitBoard::from_u128(1124249833570304);
            let bb2 = BitBoard::from_u128(548949983232);
            let mut bb3 = bb1 & bb2;
            bb3.flip();
            let _ = bb3.clone() >> 1;
            let _ = bb3.clone() << 1;
        });
    });
}

fn benchmark_game_random_game(c: &mut Criterion) {
    use rustshogi::game::Game;
    c.bench_function("random_game", |b| {
        b.iter(|| {
            let mut game = Game::new();
            game.input_board("startpos".to_string());
            let _result_game = game.one_play();
        });
    });
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
    benchmark_bitboard,
    benchmark_game_random_game,
    benchmark_direction,
    benchmark_piece,
    benchmark_random
);
criterion_main!(benches);
