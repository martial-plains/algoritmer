//! This module provides a function to calculate the factorial of a given number.
//!
//! The `calculate_factorial` function computes the factorial of a non-negative integer. If a negative integer is provided, the function will panic.

use core::iter::Step;

use num::Num;

/// A trait for calculating the factorial of a number.
///
/// # Example
///
/// ```
/// use algoritmer::math::Factorial;
///
/// let result = 5.factorial();
/// assert_eq!(result, 120);
/// ```
pub trait Factorial {
    /// Calculates the factorial of the input.
    ///
    /// # Returns
    ///
    /// The factorial of the input number.
    ///
    /// # Panics
    ///
    /// This function will panic if the input number is negative, as there is no factorial defined for negative integers.
    #[must_use]
    fn factorial(self) -> Self;
}

impl<T> Factorial for T
where
    T: Num + Step + core::iter::Product,
{
    fn factorial(self) -> T {
        if self < T::zero() {
            panic!("No Factorial for negative numbers");
        } else {
            (T::one()..=self).product()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 1)]
    #[test_case(2, 2)]
    #[test_case(3, 6)]
    #[test_case(5, 120)]
    #[test_case(8, 40320)]
    #[test_case(10, 3_628_800)]
    fn factorial_of(num: i32, expected: i32) {
        let actual = num.factorial();
        assert_eq!(expected, actual);
    }
}
