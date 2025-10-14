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

criterion_group!(benches, benchmark_bitboard_operations, benchmark_game_logic);
criterion_main!(benches);
