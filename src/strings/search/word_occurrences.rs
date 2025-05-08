use alloc::vec::Vec;

/// Create a map containing count of each word
///
/// # Arguments
///
/// * `text` - The text to be analyzed
///
/// # Returns
///
/// Returns a map containing count of each word
#[must_use]
pub fn word_occurrences(text: &str) -> Vec<(&str, u32)> {
    let mut occurrences: Vec<(&str, u32)> = Vec::new();

    for word in text.split_ascii_whitespace() {
        if let Some(entry) = occurrences.iter_mut().find(|(w, _)| *w == word) {
            entry.1 += 1;
        } else {
            occurrences.push((word, 1));
        }
    }

    occurrences
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hello_1_world_1() {
        let mut expected: Vec<(&str, u32)> = Vec::new();
        let _ = expected.push(("Hello", 1));
        let _ = expected.push(("World", 1));
        let actual = word_occurrences("Hello World");
        assert_eq!(expected, actual);
    }
}
