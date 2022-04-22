pub fn reverse_words(text: String) -> String {
    text.split(' ').rev().collect()
}
