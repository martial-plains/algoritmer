//! This module contains the implementations of dynamic programming styles written in Rust.
//!
//! ## Dynamic Programming
//!
//! Dynamic programming is a method of solving problems by breaking the problem down into subproblems,
//! and then solving each subproblem only once.
//!
//! [Wikipedia](https://en.wikipedia.org/wiki/Dynamic_programming)
mod memoize;

pub use memoize::memoize;
