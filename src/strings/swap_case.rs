use alloc::string::String;

/// A trait for swapping the case of characters in a string.
pub trait CaseSwapper {
    /// This function will convert all lowercase letters to uppercase letters and vice versa.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to be processed
    ///
    /// # Returns
    ///
    /// Returns the text with all lowercase letters converted to uppercase letters and vice versa.
    #[must_use]
    fn swaped_case(&self) -> String;
}

impl CaseSwapper for str {
    fn swaped_case(&self) -> String {
        self.chars()
            .map(|c| {
                if c.is_ascii_lowercase() {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .fold(String::new(), |mut s, ch| {
                s.push(ch);
                s
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Algorithm", "aLGORITHM")]
    fn check_case_swap(text: &str, expected: &str) {
        let actual = text.swaped_case();
        assert_eq!(expected, actual);
    }
}
