/// Calculates the factorial of the input
///
/// # Arguments
///
/// * `num` - The number to calculate the factorial of
pub fn calculate_factorial(num: i32) -> i32 {
    if num < 0 {
        panic!("No Factorial for negative numbers");
    } else {
        (1..=num).fold(1, |acc, n| acc * n)
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
    #[test_case(10, 3628800)]
    fn factorial_of(num: i32, expected: i32) {
        let actual = calculate_factorial(num);
        assert_eq!(expected, actual);
    }
}
