use alloc::string::String;

/// Reverses the words in a sentence.
///
/// # Arguments
///
/// * `sentence` - The sentence to be reversed.
pub fn reverse_words(sentence: &str) -> String {
    sentence.split(' ').rev().collect()
}
