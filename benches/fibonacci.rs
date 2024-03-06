#![feature(test)]

extern crate test;

use algorithms::math::fibonacci::*;

use test::Bencher;

// bench: find the `BENCH_SIZE` first terms of the fibonacci sequence
static BENCH_SIZE: usize = 10;

#[bench]
fn recursive_fibonacci(b: &mut Bencher) {
    b.iter(|| (0..BENCH_SIZE).map(recursive).collect::<Vec<_>>());
}

#[bench]
fn iterative_fibonacci(b: &mut Bencher) {
    b.iter(|| (0..BENCH_SIZE).map(iterative).collect::<Vec<_>>())
}

#[bench]
fn memoized_fibonacci(b: &mut Bencher) {
    b.iter(|| (0..BENCH_SIZE).map(memoized).collect::<Vec<_>>())
}

#[bench]
fn analytic_fibonacci(b: &mut Bencher) {
    b.iter(|| (0..BENCH_SIZE).map(analytic).collect::<Vec<_>>())
}
