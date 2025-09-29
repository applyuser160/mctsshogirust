use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bitboard_benchmark(c: &mut Criterion) {
    use shogi::bitboard::BitBoard;
    c.bench_function("bitbord", |b| {
        b.iter( || {
            let bb1 = BitBoard::from_u128(1124249833570304);
            let bb2 = BitBoard::from_u128(548949983232);
            let mut bb3 = bb1 & bb2;
            bb3.flip();
            bb3.clone() >> 1;
            bb3.clone() << 1;
        });
    });
}

criterion_group!(benches, bitboard_benchmark);
criterion_main!(benches);
