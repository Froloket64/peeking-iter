use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use peeking_iter::PeekingIter;
use rand::random;
use std::iter;

fn peek(c: &mut Criterion) {
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

fn peek_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("Peeking");
    let it = iter::from_fn(|| Some(random::<u32>())).cycle();
    let mut multipeek = it.clone().multipeek();
    let mut peeking_iter = PeekingIter::new(it);

    group.bench_function(BenchmarkId::new("itertools::MultiPeek", "random"), |b| {
        b.iter(|| {
            multipeek.next();
        })
    });

    group.bench_function(BenchmarkId::new("PeekingIter", "random"), |b| {
        b.iter(|| {
            peeking_iter.next();
        })
    });
}

fn next(c: &mut Criterion) {
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

fn next_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("Plain next()");
    let mut it = iter::from_fn(|| Some(random::<u32>())).cycle();
    let mut multipeek = it.clone().multipeek();
    let mut peeking_iter = PeekingIter::new(it.clone());

    group.bench_function(BenchmarkId::new("std::ops::Range", "random"), |b| {
        b.iter(|| {
            it.next();
        })
    });

    group.bench_function(BenchmarkId::new("itertools::MultiPeek", "random"), |b| {
        b.iter(|| {
            multipeek.next();
        })
    });

    group.bench_function(BenchmarkId::new("PeekingIter", "random"), |b| {
        b.iter(|| {
            peeking_iter.next();
        })
    });
}

fn next_peek(c: &mut Criterion) {
    let mut group = c.benchmark_group("next() + peek()");
    let mut multipeek = (0..1000).multipeek();
    let mut peeking_iter = PeekingIter::new(0..1000);

    group.bench_function(BenchmarkId::new("itertools::MultiPeek", "(0..1000)"), |b| {
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

fn next_peek_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("next() + peek()");
    let it = iter::from_fn(|| Some(random::<u32>())).cycle();
    let mut multipeek = it.clone().multipeek();
    let mut peeking_iter = PeekingIter::new(it);

    group.bench_function(BenchmarkId::new("itertools::MultiPeek", "random"), |b| {
        b.iter(|| {
            multipeek.next();
            multipeek.peek();
            multipeek.peek();
            multipeek.next();
        });
    });

    group.bench_function(BenchmarkId::new("PeekingIter", "random"), |b| {
        b.iter(|| {
            peeking_iter.next();
            peeking_iter.peek();
            peeking_iter.peek();
            peeking_iter.next();
        });
    });
}

fn next_while(c: &mut Criterion) {
    let it = iter::from_fn(|| Some(random::<u32>())).cycle();
    let mut peeking_iter = PeekingIter::new(it);

    c.bench_function("next_while", |b| {
        b.iter(|| peeking_iter.next());
    });
}

criterion_group! { compare_preset, peek, next, next_peek }
criterion_group! { compare_random, peek_random, next_random, next_peek_random }
criterion_group! { targeted, next_while }
criterion_main! { compare_preset, compare_random }
