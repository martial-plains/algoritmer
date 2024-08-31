use alloc::{
    string::{String, ToString},
    vec::Vec,
};

/// Trait that provides methods for reversing strings and their components.
pub trait Reversible {
    /// Reverses the order of characters in a word.
    ///
    /// # Arguments
    ///
    /// * `word` - The word to reverse.
    ///
    /// # Returns
    ///
    /// A new `String` with the characters in reverse order.
    ///
    /// # Example
    ///
    /// ```
    /// use algoritmer::strings::Reversible;
    ///
    /// let reversed = "hello".reversed_word();
    /// assert_eq!(reversed, "olleh");
    /// ```
    fn reversed_word(&self) -> String;

    /// Reverses the order of words in a sentence.
    ///
    /// # Arguments
    ///
    /// * `sentence` - The sentence to reverse.
    ///
    /// # Returns
    ///
    /// A new `String` with the words in reverse order.
    ///
    /// # Example
    ///
    /// ```
    /// use algoritmer::strings::Reversible;
    ///
    /// let reversed = "Rust is awesome".reversed_word_order(' ');
    /// assert_eq!(reversed, "awesome is Rust");
    /// ```
    fn reversed_word_order<P>(&self, pattern: P) -> String
    where
        String: From<P>;

    /// Reverses all words longer than `n` characters in a sentence.
    ///
    /// # Arguments
    ///
    /// * `sentence` - The sentence to process.
    /// * `n` - The minimum length of a word before it can be reversed.
    ///
    /// # Returns
    ///
    /// A new `String` with all words longer than `n` characters reversed.
    ///
    /// # Example
    ///
    /// ```
    /// use algoritmer::strings::Reversible;
    ///
    /// let result = "Rust is great and Java is also great".reversed_words_longer_than(4, ' ');
    /// assert_eq!(result, "Rust is taerg and Java is also taerg");
    /// ```
    fn reversed_words_longer_than<P>(&self, n: usize, pattern: P) -> String
    where
        String: From<P>;
}

impl Reversible for str {
    fn reversed_word(&self) -> String {
        self.chars().rev().collect()
    }

    fn reversed_word_order<P>(&self, pattern: P) -> String
    where
        String: From<P>,
    {
        self.split_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(&String::from(pattern))
    }

    fn reversed_words_longer_than<P>(&self, n: usize, pattern: P) -> String
    where
        String: From<P>,
    {
        self.split_whitespace()
            .map(|word| {
                if word.len() > n {
                    word.chars().rev().collect::<String>()
                } else {
                    word.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(&String::from(pattern))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case("Hey wollef sroirraw", 4)]
    fn test_remove_long_words(sentence: &str, index: usize) {
        let actual = sentence.reversed_words_longer_than(index, ' ');
        assert_eq!(actual, "Hey fellow warriors");
    }
}
