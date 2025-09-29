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

criterion_group!(benches, benchmark_bitboard, benchmark_game_random_game);
criterion_main!(benches);
