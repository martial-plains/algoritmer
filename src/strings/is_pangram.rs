/// wiki: <https://en.wikipedia.org/wiki/Pangram>
pub fn is_pangram(text: &str) -> bool {
    let mut flags = [false; 26];
    for ch in text.to_ascii_lowercase().chars() {
        if ch.is_alphabetic() {
            flags[ch as usize - 'a' as usize] = true;
        }
    }
    flags.iter().all(|f| *f == true)
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
