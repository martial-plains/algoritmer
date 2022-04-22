/// Returns true if a number is perfect.
///
/// # Arguments
///
/// * `number` - The number to check
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
    #[test_case(33550336, true)]
    #[test_case(33550337, false)]
    fn is_number_perfect(n: i32, expected: bool) {
        let actual = is_perfect(n);
        assert_eq!(expected, actual);
    }
}
