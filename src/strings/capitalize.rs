/// This function will capitalize the first letter of a sentence or a word
pub fn capitalize(sentence: &String) -> String {
    if *sentence != "".to_string() {
        let mut new_string = String::new();
        for (i, ch) in sentence.char_indices() {
            if i == 0 && sentence.starts_with(|c| c >= 'a' && c <= 'z') {
                new_string.insert(i, ch.to_ascii_uppercase());
            } else {
                new_string.insert(i, ch);
            }
        }
        new_string
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("123 hello world", "123 hello world".to_string())]
    #[test_case("hello world", "Hello world".to_string())]
    #[test_case("a", "A".to_string())]
    #[test_case("", "".to_string())]
    fn does_it_capitalize(sentence: &str, expected: String) {
        let actual = capitalize(&mut String::from(sentence));
        assert_eq!(expected, actual);
    }
}
