/// Returns the power of a number.
///
/// # Arguments
///
/// * `base` - The base of the power
/// * `exponent` - The exponent of the power
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
