//! The Fibonacci sequence is a sequence of numbers that starts with 1 and 1, and each subsequent number is the sum of the previous two.
//!
//! [Wikipedia](https://en.wikipedia.org/wiki/Fibonacci_number)

use std::collections::HashMap;

use crate::dynamic_programming::memoize;

/// Calculate the nth Fibonacci number recursively.
pub fn recursive(nth: usize) -> u128 {
    if nth <= 1 {
        nth as u128
    } else {
        recursive(nth - 1) + recursive(nth - 2)
    }
}

/// Calculate the nth Fibonacci number imperatively.
pub fn imperative(nth: usize) -> u128 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 1..nth {
        c = a + b;
        a = b;
        b = c;
    }
    c
}

/// Calculate the nth Fibonacci number using dynamic_programming and memoization.
pub fn memoized(nth: usize) -> u128 {
    fn fib_memo(cache: &mut HashMap<usize, u128>, arg: usize) -> u128 {
        match arg {
            0 => 0,
            1 => 1,
            n => (memoize(cache, fib_memo, n - 1) + memoize(cache, fib_memo, arg - 2)).into(),
        }
    }
    memoize(&mut HashMap::new(), fib_memo, nth)
}
