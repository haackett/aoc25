use aoc2025::*;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn benchmark_day1(c: &mut Criterion) {
    let input1 = include_str!("../input/1");

    c.bench_function("day1p1", |b| b.iter(|| day1p1(black_box(input1))));

    c.bench_function("day1p2", |b| b.iter(|| day1p2(black_box(input1))));
}

fn benchmark_day2(c: &mut Criterion) {
    let input2 = include_str!("../input/2");

    c.bench_function("day2p1", |b| b.iter(|| day2p1(black_box(input2))));

    c.bench_function("day2p2", |b| b.iter(|| day2p2(black_box(input2))));
}

fn benchmark_day3(c: &mut Criterion) {
    let input3 = include_str!("../input/3");

    c.bench_function("day3p1", |b| b.iter(|| day3p1(black_box(input3))));

    c.bench_function("day3p2", |b| b.iter(|| day3p2(black_box(input3))));
}

criterion_group!(benches, benchmark_day1, benchmark_day2, benchmark_day3);
criterion_main!(benches);
