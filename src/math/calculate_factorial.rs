/// Calculates the factorial of the input
///
/// # Arguments
///
/// * `num` - The number to calculate the factorial of
///
/// # Returns
///
/// The factorial of the input number `num`.
///
/// # Panics
///
/// This function will panic if the input num is a negative number, as there is no factorial defined for negative integers.
#[must_use]
pub fn calculate_factorial(num: i32) -> i32 {
    if num < 0 {
        panic!("No Factorial for negative numbers");
    } else {
        (1..=num).product()
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
        let actual = calculate_factorial(num);
        assert_eq!(expected, actual);
    }
}
