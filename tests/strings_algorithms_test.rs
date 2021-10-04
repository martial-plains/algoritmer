#[cfg(test)]
mod tests {
    extern crate test_case;

    use std::collections::HashMap;
    use algorithms::strings::*;
    use test_case::test_case;

    #[test_case("123 hello world", "123 hello world".to_string())]
    #[test_case("hello world", "Hello world".to_string())]
    #[test_case("a", "A".to_string())]
    #[test_case("", "".to_string())]
    fn does_it_capitalize(sentence: &str, expected: String) {
        let actual = capitalize(&mut String::from(sentence));
        assert_eq!(expected, actual);
    }

    #[test_case("Silent", "Listen", true)]
    #[test_case("This is    a      string", "Is     this a string", true)]
    #[test_case("There", "Their", false)]
    fn check_anagram(string1: &str, string2: &str, expected: bool) {
        let actual = is_anagram(string1, string2);
        assert_eq!(expected, actual)
    }

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

    #[test_case("amanaplanacanalpanama", true)]
    #[test_case("Hello", false)]
    #[test_case("Able was I ere I saw Elba", true)]
    #[test_case("racecar", true)]
    #[test_case("Mr. Owl ate my metal worm?", true)]
    fn check_palindrome(text: &str, expected: bool) {
        let actual = is_palindrome(text);
        assert_eq!(expected, actual)
    }

    #[test_case(String::from("Algorithm"), String::from("aLGORITHM"))]
    fn check_case_swap(text: String, expected: String) {
        let actual = swap_case(text);
        assert_eq!(expected, actual);
    }

    #[test_case("Python is great and Java is also great".to_string(), "Python is great and Java also".to_string())]
    fn check_removed_duplicates(text: String, expected: String) {
        let actual = remove_duplicates(text);
        assert_eq!(expected, actual)
    }

    #[test]
    fn hello_1_world_1() {
        let mut expected: HashMap<&str, u32> = HashMap::new();
        expected.insert("Hello", 1);
        expected.insert("World", 1);
        let actual = word_occurrences("Hello World");
        assert_eq!(expected, actual)
    }
}