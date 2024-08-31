//! The Fibonacci sequence is a sequence of numbers that starts with 1 and 1, and each subsequent number is the sum of the previous two.
//!
//! [Wikipedia](https://en.wikipedia.org/wiki/Fibonacci_number)

use hashbrown::HashMap;

use crate::dynamic_programming::memoize;

/// A trait for calculating Fibonacci numbers using different methods.
pub trait Fibonacci {
    /// Calculate the nth Fibonacci number recursively.
    #[must_use]
    fn recursive(self) -> Self;

    /// Calculate the nth Fibonacci number imperatively.
    #[must_use]
    fn iterative(self) -> Self;

    /// Calculate the nth Fibonacci number using dynamic programming and memoization.
    #[must_use]
    fn memoized(self) -> Self;

    /// Calculate the nth Fibonacci number using Binet's formula.
    ///
    /// # Note
    /// This is sufficient to calculate the first 70 Fibonacci numbers.
    #[must_use]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap
    )]
    fn analytic(self) -> Self;
}

impl Fibonacci for usize {
    fn recursive(self) -> usize {
        if self <= 1 {
            self
        } else {
            Self::recursive(self - 1) + Self::recursive(self - 2)
        }
    }

    fn iterative(self) -> usize {
        match self {
            0 => 0,
            1 | 2 => 1,
            _ => {
                let mut prev = 1;
                let mut curr = 1;
                for _ in 2..self {
                    let temp = curr;
                    curr += prev;
                    prev = temp;
                }
                curr
            }
        }
    }

    fn memoized(self) -> usize {
        fn fib_memo(cache: &mut HashMap<usize, usize>, arg: usize) -> usize {
            match arg {
                0 => 0,
                1 => 1,
                n => memoize(cache, fib_memo, n - 1) + memoize(cache, fib_memo, arg - 2),
            }
        }
        memoize(&mut HashMap::new(), fib_memo, self)
    }

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap
    )]
    fn analytic(self) -> usize {
        let sqrt_5 = f64::sqrt(5.0);
        let phi = (1. + sqrt_5) / 2.;
        let q = 1. / phi;

        ((phi.powi(self as i32) + q.powi(self as i32)) / sqrt_5 + 0.5) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::Fibonacci;

    #[test]
    fn test_recursive() {
        assert_eq!(1.recursive(), 1);
        assert_eq!(2.recursive(), 1);
        assert_eq!(3.recursive(), 2);
        assert_eq!(4.recursive(), 3);
        assert_eq!(5.recursive(), 5);
    }

    #[test]
    fn test_iterative() {
        assert_eq!(1.iterative(), 1);
        assert_eq!(2.iterative(), 1);
        assert_eq!(3.iterative(), 2);
        assert_eq!(4.iterative(), 3);
        assert_eq!(5.iterative(), 5);
    }

    #[test]
    fn test_memoized() {
        assert_eq!(1.memoized(), 1);
        assert_eq!(2.memoized(), 1);
        assert_eq!(3.memoized(), 2);
        assert_eq!(4.memoized(), 3);
        assert_eq!(5.memoized(), 5);
    }

    #[test]
    fn test_analytic() {
        assert_eq!(1.analytic(), 1);
        assert_eq!(2.analytic(), 1);
        assert_eq!(3.analytic(), 2);
        assert_eq!(4.analytic(), 3);
        assert_eq!(5.analytic(), 5);
    }
}
