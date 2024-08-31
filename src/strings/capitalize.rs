use alloc::string::String;

/// A trait for capitalizing the first letter of a word or sentence.
///
/// This trait provides a method to capitalize the first character of a string slice or a string,
/// converting it to uppercase if it is a lowercase ASCII character.
pub trait Capitalizable {
    /// This function will capitalize the first letter of a sentence or a word
    ///
    /// # Returns
    ///
    /// A new `String` with the first letter capitalized. If the string is empty,
    /// an empty string is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use algoritmer::strings::Capitalizable;
    ///
    /// let text = "hello world";
    /// let capitalized = text.capitalize();
    /// assert_eq!(capitalized, "Hello world");
    /// ```
    fn capitalize(&self) -> String;
}

impl Capitalizable for str {
    fn capitalize(&self) -> String {
        if *self == String::new() {
            String::new()
        } else {
            let mut new_string = String::new();
            for (i, ch) in self.char_indices() {
                if i == 0 && self.starts_with(|c: char| c.is_ascii_lowercase()) {
                    new_string.insert(i, ch.to_ascii_uppercase());
                } else {
                    new_string.insert(i, ch);
                }
            }
            new_string
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case("123 hello world", "123 hello world")]
    #[test_case("hello world", "Hello world")]
    #[test_case("a", "A")]
    #[test_case("", "")]
    fn does_it_capitalize(sentence: &str, expected: &str) {
        let actual = sentence.capitalize();
        assert_eq!(expected, actual);
    }
}
