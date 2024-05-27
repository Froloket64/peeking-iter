use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use peeking_bench::PeekingIter;

fn peeking(c: &mut Criterion) {
    let mut group = c.benchmark_group("Peeking");
    let mut multipeek = (0..1000).multipeek();
    let mut peeking_iter = PeekingIter::new(0..1000);

    group.bench_function(BenchmarkId::new("itertools::MultiPeek", "(0..1000)"), |b| {
        b.iter(|| {
            multipeek.next();
        })
    });

    group.bench_function(BenchmarkId::new("PeekingIter", "(0..1000)"), |b| {
        b.iter(|| {
            peeking_iter.next();
        })
    });
}

fn next_peek(c: &mut Criterion) {
    let mut group = c.benchmark_group("next() + peek()");
    let mut multipeek = (0..1000).multipeek();
    let mut peeking_iter = PeekingIter::new(0..1000);

    group.bench_function(BenchmarkId::new("itertools:MultiPeek", "(0..1000)"), |b| {
        b.iter(|| {
            multipeek.next();
            multipeek.peek();
            multipeek.peek();
            multipeek.next();
        });
    });

    group.bench_function(BenchmarkId::new("PeekingIter", "(0..1000)"), |b| {
        b.iter(|| {
            peeking_iter.next();
            peeking_iter.peek();
            peeking_iter.peek();
            peeking_iter.next();
        });
    });
}

fn plain_next(c: &mut Criterion) {
    let mut group = c.benchmark_group("Plain next()");
    let mut multipeek = (0..1000).multipeek();
    let mut peeking_iter = PeekingIter::new(0..1000);
    let mut it = 0..1000;

    group.bench_function(BenchmarkId::new("std::ops::Range", "(0..1000)"), |b| {
        b.iter(|| {
            it.next();
        })
    });

    group.bench_function(BenchmarkId::new("itertools::MultiPeek", "(0..1000)"), |b| {
        b.iter(|| {
            multipeek.next();
        })
    });

    group.bench_function(BenchmarkId::new("PeekingIter", "(0..1000)"), |b| {
        b.iter(|| {
            peeking_iter.next();
        })
    });
}

criterion_group! { benches, peeking, next_peek, plain_next }
criterion_main! { benches }
