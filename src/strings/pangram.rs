/// A pangram or holoalphabetic sentence is a sentence using every letter of a given alphabet at least once
///
/// # Arguments
///
/// * `text` - The sentence to be checked
///
/// # Returns
///
/// Returns true if the sentence is a pangram else false
///
/// - [Pangram](https://en.wikipedia.org/wiki/Pangram)
#[must_use]
pub fn is_pangram(text: &str) -> bool {
    text.to_ascii_lowercase()
        .chars()
        .fold([false; 26], |mut init, ch| {
            if ch.is_alphabetic() {
                init[ch as usize - 'a' as usize] = true;
                init
            } else {
                init
            }
        })
        .iter()
        .all(|f| *f)
}

/// A trait to check if a string is a pangram.
///
/// A pangram or holoalphabetic sentence is a sentence that uses every letter of the alphabet at least once.
///
/// # Examples
///
/// ```
/// use algoritmer::strings::PangramCheck;
///
/// let sentence = "The quick brown fox jumps over the lazy dog";
/// assert!(sentence.is_pangram());
/// ```
pub trait PangramCheck {
    /// Checks if the implementing string is a pangram.
    ///
    /// # Returns
    ///
    /// Returns `true` if the string is a pangram, otherwise returns `false`.
    #[must_use]
    fn is_pangram(&self) -> bool;
}

impl PangramCheck for str {
    fn is_pangram(&self) -> bool {
        self.to_ascii_lowercase()
            .chars()
            .fold([false; 26], |mut init, ch| {
                if ch.is_alphabetic() {
                    init[ch as usize - 'a' as usize] = true;
                }
                init
            })
            .iter()
            .all(|&f| f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("The quick brown fox jumps over the lazy dog", true)]
    #[test_case("Waltz, bad nymph, for quick jigs vex.", true)]
    #[test_case("Jived fox nymph grabs quick waltz.", true)]
    #[test_case("My name is Unknown", false)]
    #[test_case("The quick brown fox jumps over the la_y do", false)]
    #[test_case("", false)]
    fn check_pangram(sentence: &str, expected: bool) {
        let actual = is_pangram(sentence);
        assert_eq!(expected, actual);
    }
}
