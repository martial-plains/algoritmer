/// Determine whether the string is palindrome
///
/// # Arguments
///
/// * `text` - The text to be checked
///
/// # Returns
///
/// Returns true if the string is palindrome else false
#[must_use]
pub fn is_palindrome(text: &str) -> bool {
    text.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .eq(text
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .rev())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("amanaplanacanalpanama", true)]
    #[test_case("Hello", false)]
    #[test_case("Able was I ere I saw Elba", true)]
    #[test_case("racecar", true)]
    #[test_case("Mr. Owl ate my metal worm?", true)]
    fn check_palindrome(text: &str, expected: bool) {
        let actual = is_palindrome(text);
        assert_eq!(expected, actual);
    }
}
