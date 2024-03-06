//! The Fibonacci sequence is a sequence of numbers that starts with 1 and 1, and each subsequent number is the sum of the previous two.
//!
//! [Wikipedia](https://en.wikipedia.org/wiki/Fibonacci_number)

use hashbrown::HashMap;

use crate::dynamic_programming::memoize;

/// Calculate the nth Fibonacci number recursively.
///
/// # Arguments
///
/// * `nth` - The nth Fibonacci number to calculate.
#[must_use]
pub fn recursive(nth: usize) -> usize {
    if nth <= 1 {
        nth
    } else {
        recursive(nth - 1) + recursive(nth - 2)
    }
}

/// Calculate the nth Fibonacci number imperatively.
///
/// # Arguments
///
/// * `nth` - The nth Fibonacci number to calculate.
#[must_use]
pub fn iterative(nth: usize) -> usize {
    match nth {
        0 => 0,
        1 | 2 => 1,
        _ => {
            let mut prev = 1;
            let mut curr = 1;
            for _ in 2..nth {
                let temp = curr;
                curr += prev;
                prev = temp;
            }
            curr
        }
    }
}

/// Calculate the nth Fibonacci number using dynamic programming and memoization.
///
/// # Arguments
///
/// * `nth` - The nth Fibonacci number to calculate.
pub fn memoized(nth: usize) -> usize {
    fn fib_memo(cache: &mut HashMap<usize, usize>, arg: usize) -> usize {
        match arg {
            0 => 0,
            1 => 1,
            n => memoize(cache, fib_memo, n - 1) + memoize(cache, fib_memo, arg - 2),
        }
    }
    memoize(&mut HashMap::new(), fib_memo, nth)
}

/// Calculate the nth Fibonacci number using Binet's formula.
///
/// # Arguments
///
/// * `nth` - The nth Fibonacci number to calculate.
///
/// # Note
/// This is sufficient to calculate the first 70 Fibonacci numbers.
#[must_use]
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
pub fn analytic(nth: usize) -> usize {
    let sqrt_5 = f64::sqrt(5.0);
    let phi = (1. + sqrt_5) / 2.;
    let q = 1. / phi;

    ((phi.powi(nth as i32) + q.powi(nth as i32)) / sqrt_5 + 0.5) as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_recursive() {
        assert_eq!(super::recursive(1), 1);
        assert_eq!(super::recursive(2), 1);
        assert_eq!(super::recursive(3), 2);
        assert_eq!(super::recursive(4), 3);
        assert_eq!(super::recursive(5), 5);
    }

    #[test]
    fn test_iterative() {
        assert_eq!(super::iterative(1), 1);
        assert_eq!(super::iterative(2), 1);
        assert_eq!(super::iterative(3), 2);
        assert_eq!(super::iterative(4), 3);
        assert_eq!(super::iterative(5), 5);
    }

    #[test]
    fn test_memoized() {
        assert_eq!(super::memoized(1), 1);
        assert_eq!(super::memoized(2), 1);
        assert_eq!(super::memoized(3), 2);
        assert_eq!(super::memoized(4), 3);
        assert_eq!(super::memoized(5), 5);
    }

    #[test]
    fn test_analytic() {
        assert_eq!(super::analytic(1), 1);
        assert_eq!(super::analytic(2), 1);
        assert_eq!(super::analytic(3), 2);
        assert_eq!(super::analytic(4), 3);
        assert_eq!(super::analytic(5), 5);
    }
}
