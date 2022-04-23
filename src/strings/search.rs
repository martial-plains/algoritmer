//! Useful algorithms for searching strings

mod brute_force;
mod kmp;
mod naive_pattern_search;
mod rabin_karp;
mod word_occurrences;

pub use brute_force::*;
pub use kmp::*;
pub use naive_pattern_search::*;
pub use rabin_karp::*;
pub use word_occurrences::*;
