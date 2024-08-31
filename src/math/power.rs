//! This module provides a function to compute the power of a given base raised to a specified exponent.

/// Returns the power of a number.
///
/// # Arguments
///
/// * `base` - The base of the power
/// * `exponent` - The exponent of the power
///
/// # Example
///
/// ```
/// use algoritmer::math::power;
///
/// let result = power(2, 3);
/// // The base is 2 and the exponent is 3.
/// // Therefore, the result is 2 * 2 * 2 = 8.
/// assert_eq!(result, 8);
/// ```
///
/// # Explanation
///
/// The `power` function calculates the result of raising a `base` to a given `exponent`.
/// It uses a fold operation to multiply the `base` repeatedly, starting from 1, for the
/// number of times specified by the `exponent`. The initial value of 1 ensures that when
/// the exponent is 0, the result will be 1 (any number raised to the power of 0 is 1).
///
/// The fold operation iterates over the range from 1 to `exponent`, inclusive. For each
/// iteration, the current value of `acc` (which starts with 1) is multiplied by the `base`.
/// Finally, the resulting value is returned as the power of the number.
#[must_use]
pub fn power(base: i32, exponent: i32) -> i32 {
    (1..=exponent).fold(1, |acc, _| -> i32 { acc * base })
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
        let actual = power(num, pow);
        assert_eq!(expected, actual);
    }
}
