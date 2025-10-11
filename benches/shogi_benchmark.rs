use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_bitboard(c: &mut Criterion) {
    use rustshogi::bitboard::BitBoard;
    c.bench_function("bitboard_ops", |b| {
        b.iter(|| {
            let bb1 = BitBoard::from_u128(1124249833570304);
            let bb2 = BitBoard::from_u128(548949983232);
            let bb3 = &bb1 & &bb2;
            let _ = &bb1 | &bb3;
        });
    });

    c.bench_function("bitboard_get_trues", |b| {
        let bb = BitBoard::from_u128(1124249833570304);
        b.iter(|| {
            let _ = bb.get_trues();
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

criterion_group!(benches, benchmark_bitboard, benchmark_game_random_game);
criterion_main!(benches);
