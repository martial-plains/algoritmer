/// Returns true if a number is perfect.
///
/// A perfect number is a positive integer that is equal to the sum of its proper divisors.
/// For example, the number 6 is perfect because its proper divisors are 1, 2, and 3, and
/// their sum is 6, which is equal to the number itself.
///
/// # Arguments
///
/// * `number` - The number to check
///
/// # Examples
///
/// ```
/// use algorithms::math::is_perfect;
///
/// assert_eq!(is_perfect(6), true);
/// assert_eq!(is_perfect(28), true);
/// assert_eq!(is_perfect(12), false);
/// assert_eq!(is_perfect(496), true);
/// assert_eq!(is_perfect(10), false);
/// ```
///
/// The `is_perfect` function can be used to determine if a given number is perfect. In the examples
/// above, the function is called with different numbers, and the expected results are asserted using
/// the `assert_eq!` macro. The first example checks if 6 is a perfect number, which is true. The second
/// example checks if 28 is a perfect number, which is also true. The third example checks if 12 is a
/// perfect number, which is false. The fourth example checks if 496 is a perfect number, which is true.
/// Lastly, the fifth example checks if 10 is a perfect number, which is false.
///
/// ```
/// use algorithms::math::is_perfect;
///
/// assert_eq!(is_perfect(-6), false);
/// assert_eq!(is_perfect(0), false);
/// ```
///
/// The function returns false for non-positive numbers (less than or equal to 0) since they are not
/// considered perfect numbers.
#[must_use]
pub fn is_perfect(number: i32) -> bool {
    match number {
        x if x <= 0 => false,
        _ => (1..number).filter(|n| number % n == 0).sum::<i32>() == number,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(-1, false)]
    #[test_case(0, false)]
    #[test_case(2, false)]
    #[test_case(3, false)]
    #[test_case(4, false)]
    #[test_case(5, false)]
    #[test_case(6, true)]
    #[test_case(7, false)]
    #[test_case(27, false)]
    #[test_case(28, true)]
    #[test_case(496, true)]
    #[test_case(8128, true)]
    #[test_case(33_550_336, true)]
    #[test_case(33_550_337, false)]
    fn is_number_perfect(n: i32, expected: bool) {
        let actual = is_perfect(n);
        assert_eq!(expected, actual);
    }
}
