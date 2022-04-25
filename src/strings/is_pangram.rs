/// A pangram or holoalphabetic sentence is a sentence using every letter of a given alphabet at least once
///
/// wiki: <https://en.wikipedia.org/wiki/Pangram>
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
        assert_eq!(expected, actual)
    }
}
