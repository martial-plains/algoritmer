#[cfg(test)]
mod tests {
    extern crate test_case;
    use algorithms::math_algorithms::*;
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
    #[test_case(33550336, true)]
    #[test_case(33550337, false)]
    fn is_number_perfect(n: i32, expected: bool) {
        let actual = is_perfect(n);
        assert_eq!(expected, actual);
    }

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
