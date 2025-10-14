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

fn benchmark_simd_bitboard_operations(c: &mut Criterion) {
    use rustshogi::bitboard::BitBoard;
    use rustshogi::board::Board;
    use rustshogi::color::ColorType;
    use rustshogi::piece::PieceType;

    c.bench_function("simd_bitand", |b| {
        let bb1 = BitBoard::from_u128(1124249833570304);
        let bb2 = BitBoard::from_u128(548949983232);
        b.iter(|| {
            let _ = bb1.simd_bitand(&bb2);
        });
    });

    c.bench_function("simd_get_trues", |b| {
        let bb = BitBoard::from_u128(548949983232);
        b.iter(|| {
            let _ = bb.simd_get_trues();
        });
    });

    c.bench_function("optimized_deploy", |b| {
        let mut board = Board::new();
        b.iter(|| {
            board.deploy(55, PieceType::Pawn, ColorType::Black);
        });
    });
}

criterion_group!(
    benches,
    benchmark_bitboard,
    benchmark_game_random_game,
    benchmark_simd_bitboard_operations
);
criterion_main!(benches);
