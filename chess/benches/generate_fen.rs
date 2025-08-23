use chess::{Chessboard, ChessboardRaw, EMPTY_POS, KIWIPETE_POS, START_POS};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn fen(c: &mut Criterion) {
    const FENS: [(&str, &str); 3] = [
        ("start", START_POS),
        ("empty", EMPTY_POS),
        ("kiwipete", KIWIPETE_POS),
    ];
    let mut group = c.benchmark_group("FEN");

    for fen in FENS {
        group.bench_with_input(BenchmarkId::new("raw", fen.0), &fen.1, |b, i| {
            b.iter(|| black_box(ChessboardRaw::from_fen(i).unwrap()))
        });
        // Empty boards are not allowed in regular Chessboard's
        if fen.0 != "empty" {
            group.bench_with_input(BenchmarkId::new("board", fen.0), &fen.1, |b, i| {
                b.iter(|| black_box(Chessboard::from_fen(i).unwrap()))
            });
        }
    }
    group.finish();
}

criterion_group!(benches, fen);
criterion_main!(benches);
