//! This module provides a function to compute the power of a given base raised to a specified exponent.

use core::iter::Step;

use num::Num;

/// A trait to calculate the power of a number.
///
/// This trait provides a method to compute the power of a number by repeatedly multiplying the base.
///
/// # Example
///
/// ```
/// use algoritmer::math::Power;
///
/// let result = 2.power(3);
/// // The base is 2 and the exponent is 3.
/// // Therefore, the result is 2 * 2 * 2 = 8.
/// assert_eq!(result, 8);
/// ```
///
/// # Explanation
///
/// The `Power` trait defines a method `power` that calculates the result of raising a `base` to a given `exponent`.
/// It uses a fold operation to multiply the `base` repeatedly, starting from 1, for the number of times specified
/// by the `exponent`. The initial value of 1 ensures that when the exponent is 0, the result will be 1
/// (any number raised to the power of 0 is 1).
///
/// The fold operation iterates over the range from 1 to `exponent`, inclusive. For each iteration, the current
/// value of `acc` (which starts with 1) is multiplied by the `base`. Finally, the resulting value is returned as
/// the power of the number.
pub trait Power {
    /// Returns the power of a number.
    ///
    /// # Arguments
    ///
    /// * `exponent` - The exponent of the power
    ///
    /// # Returns
    ///
    /// The result of raising the number to the specified `exponent`.
    ///
    /// # Panics
    ///
    /// The function does not panic.
    #[must_use]
    fn power(self, exponent: Self) -> Self;
}

impl<T> Power for T
where
    T: Num + Step + Copy,
{
    fn power(self, exponent: T) -> T {
        (T::one()..=exponent).fold(T::one(), |acc, _| -> T { acc * self })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2, 2, 4)]
    #[test_case(2, 3, 8)]
    #[test_case(2, 4, 16)]
    #[test_case(2, 8, 256)]
    #[test_case(2, 16, 65536)]
    #[test_case(3, 5, 243)]
    #[test_case(5, 3, 125)]
    #[test_case(10, 4, 10000)]
    #[test_case(1, 2, 1)]
    #[test_case(1, 50, 1)]
    fn power_of(num: i32, pow: i32, expected: i32) {
        let actual = num.power(pow);
        assert_eq!(expected, actual);
    }
}
