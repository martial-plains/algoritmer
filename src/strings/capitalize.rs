use alloc::string::String;

/// This function will capitalize the first letter of a sentence or a word
///
/// # Arguments
///
/// * `text` - The text to be capitalized
#[must_use]
pub fn capitalize(text: &str) -> String {
    if *text == String::new() {
        String::new()
    } else {
        let mut new_string = String::new();
        for (i, ch) in text.char_indices() {
            if i == 0 && text.starts_with(|c: char| c.is_ascii_lowercase()) {
                new_string.insert(i, ch.to_ascii_uppercase());
            } else {
                new_string.insert(i, ch);
            }
        }
        new_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::String;

    use test_case::test_case;

    #[test_case("123 hello world", "123 hello world")]
    #[test_case("hello world", "Hello world")]
    #[test_case("a", "A")]
    #[test_case("", "")]
    fn does_it_capitalize(sentence: &str, expected: &str) {
        let actual = capitalize(&String::from(sentence));
        assert_eq!(expected, actual);
    }
}
