use alloc::vec::Vec;

/// A trait that provides a method to check if two strings are anagrams.
///
/// Two strings are anagrams if they are made of the same letters
/// arranged differently (ignoring the case and spaces).
pub trait Anagram {
    /// Checks if the current string is an anagram of another string.
    ///
    /// # Arguments
    ///
    /// * `other` - The string to compare with.
    ///
    /// # Returns
    ///
    /// Returns `true` if the strings are anagrams, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use algoritmer::strings::Anagram;
    ///
    /// let s1 = "listen";
    /// let s2 = "silent";
    /// assert!(s1.is_anagram(s2));
    /// ```
    fn is_anagram(&self, other: &str) -> bool;
}

impl Anagram for str {
    fn is_anagram(&self, other: &str) -> bool {
        let mut a: Vec<char> = self
            .chars()
            .filter(|ch| *ch != ' ')
            .map(|ch| ch.to_ascii_lowercase())
            .collect();
        let mut b: Vec<char> = other
            .chars()
            .filter(|ch| *ch != ' ')
            .map(|ch| ch.to_ascii_lowercase())
            .collect();
        a.sort_unstable();
        b.sort_unstable();
        a == b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Silent", "Listen", true)]
    #[test_case("This is    a      string", "Is     this a string", true)]
    #[test_case("There", "Their", false)]
    fn check_anagram(string1: &str, string2: &str, expected: bool) {
        let actual = string1.is_anagram(string2);
        assert_eq!(expected, actual);
    }
}
