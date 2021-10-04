use std::collections::HashMap;

use crate::dynamic_programming::memoize;

pub fn recursive(nth: usize) -> u128 {
    if nth <= 1 {
        nth as u128
    } else {
        recursive(nth - 1) + recursive(nth - 2)
    }
}

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
