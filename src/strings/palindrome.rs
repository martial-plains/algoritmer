/// Trait to determine whether a string is a palindrome.
///
/// This trait provides a method to check if a given string is a palindrome, ignoring non-alphanumeric
/// characters and treating the string in a case-insensitive manner.
///
/// # Examples
///
/// ```
/// use algoritmer::strings::Palindrome;
///
/// assert!(Palindrome::is_palindrome("A man, a plan, a canal, Panama"));
/// assert!(!Palindrome::is_palindrome("Hello, world!"));
/// ```
pub trait Palindrome {
    /// Determine whether the string is a palindrome.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to be checked.
    ///
    /// # Returns
    ///
    /// Returns `true` if the string is a palindrome; otherwise, returns `false`.
    #[must_use]
    fn is_palindrome(&self) -> bool;
}

/// Default implementation of the `Palindrome` trait.
impl Palindrome for str {
    fn is_palindrome(&self) -> bool {
        self.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .eq(self
                .chars()
                .filter(|c| c.is_alphanumeric())
                .map(|c| c.to_ascii_lowercase())
                .rev())
    }
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
        let actual = text.is_palindrome();
        assert_eq!(expected, actual);
    }
}
