use criterion::{Criterion, criterion_group, criterion_main};
use std::{hint::black_box, time::Duration};

fn perft(c: &mut Criterion) {
    let mut board = chess::Chessboard::from_fen(chess::START_POS).unwrap();

    let mut light = c.benchmark_group("light");
    light.bench_function("perft(1)", |b| {
        b.iter(|| black_box(chess::perft(&mut board, 1)))
    });
    light.bench_function("perft(2)", |b| {
        b.iter(|| black_box(chess::perft(&mut board, 2)))
    });
    light.bench_function("perft(3)", |b| {
        b.iter(|| black_box(chess::perft(&mut board, 3)))
    });
    light.bench_function("perft(4)", |b| {
        b.iter(|| black_box(chess::perft(&mut board, 4)))
    });
    light.finish();
    let mut heavy = c.benchmark_group("heavy");
    heavy.warm_up_time(Duration::from_secs(10));
    heavy.measurement_time(Duration::from_secs(10));
    heavy.sample_size(10);
    heavy.bench_function("perft(5)", |b| {
        b.iter(|| black_box(chess::perft(&mut board, 5)))
    });
    heavy.bench_function("perft(6)", |b| {
        b.iter(|| black_box(chess::perft(&mut board, 6)))
    });
    heavy.bench_function("perft(7)", |b| {
        b.iter(|| black_box(chess::perft(&mut board, 7)))
    });
    heavy.finish();
    assert_eq!(chess::perft(&mut board, 6), 119060324);
    assert_eq!(chess::perft(&mut board, 7), 3195901860);
}

criterion_group!(benches, perft);
criterion_main!(benches);
