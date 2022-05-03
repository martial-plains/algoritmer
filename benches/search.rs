#![feature(test)]

extern crate test;

use algorithms::search::*;
use test::Bencher;

#[bench]
fn binary_search_bench(b: &mut Bencher) {
    let arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| binary_search(&arr, 9).unwrap());
}

#[bench]
fn jump_search_bench(b: &mut Bencher) {
    let arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| jump_search(&arr, &9).unwrap());
}

#[bench]
fn linear_search_bench(b: &mut Bencher) {
    let arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| linear_search(&arr, 9).unwrap());
}

#[bench]
fn struzik_search_bench(b: &mut Bencher) {
    let arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| struzik_search(&arr, 9).unwrap());
}

#[bench]
fn ternary_search_bench(b: &mut Bencher) {
    let arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| ternary_search(&9, &arr, 0, arr.len() - 1).unwrap());
}
