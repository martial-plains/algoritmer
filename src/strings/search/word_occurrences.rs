use std::collections::HashMap;

/// Create a map containing count of each word
pub fn word_occurrences(text: &str) -> HashMap<&str, u32> {
    let mut occurrence: HashMap<&str, u32> = HashMap::new();
    for word in text.split_ascii_whitespace() {
        if occurrence.contains_key(word) {
            occurrence.entry(word).and_modify(|w| *w += 1);
        } else {
            occurrence.insert(word, 1);
        }
    }
    occurrence
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn hello_1_world_1() {
        let mut expected: HashMap<&str, u32> = HashMap::new();
        expected.insert("Hello", 1);
        expected.insert("World", 1);
        let actual = word_occurrences("Hello World");
        assert_eq!(expected, actual)
    }
}
