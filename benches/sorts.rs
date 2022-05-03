#![feature(test)]

extern crate test;

use algorithms::sorts::*;
use test::Bencher;

#[bench]
fn bead_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| bead_sort(&mut arr));
}

#[bench]
fn bitonic_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    let length = arr.len() - 1;
    b.iter(|| bitonic_sort(&mut arr, 0, length, 0));
}

#[bench]
fn bogo_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| bogo_sort(&mut arr));
}

#[bench]
fn bubble_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| bubble_sort(&mut arr));
}

#[bench]
fn bucket_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| bucket_sort(&mut arr));
}

#[bench]
fn cocktail_shaker_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| cocktail_shaker_sort(&mut arr));
}

#[bench]
fn comb_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| comb_sort(&mut arr));
}

#[bench]
fn insertion_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| insertion_sort(&mut arr));
}

#[bench]
fn merge_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| merge_sort(&mut arr));
}

#[bench]
fn quick_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    let hi = arr.len() as isize - 1;
    b.iter(|| quicksort(&mut arr, 0, hi));
}

#[bench]
fn selection_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| selection_sort(&mut arr));
}

#[bench]
fn shell_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| shell_sort(&mut arr));
}

#[bench]
fn wiggle_sort_bench(b: &mut Bencher) {
    let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
    b.iter(|| wiggle_sort(&mut arr));
}
