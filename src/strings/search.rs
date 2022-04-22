//! Useful algorithms for searching strings

mod brute_force;
mod kmp;
mod naive_pattern_search;
mod word_occurrences;

pub use brute_force::brute_force;
pub use kmp::kmp_search;
pub use naive_pattern_search::naive_pattern_search;
pub use word_occurrences::word_occurrences;
