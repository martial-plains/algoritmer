use std::{collections::HashMap, mem, str};

/// Find a pattern in a string by comparing the pattern to every substring
///
/// # Arguments
///
/// * `text` - Any string that might contain the pattern.
/// * `pattern` - String that we are searching for.
///
/// # Returns
///
/// Index where the pattern starts in the text
/// `None` if the pattern is not found
pub fn brute_force(text: &str, pattern: &str) -> Option<usize> {
    let pat_l = pattern.len();
    let txt_l = text.len();
    let mut i = 0;
    if pat_l > txt_l {
        return None;
    }
    while i <= txt_l - pat_l {
        if &text[i..i + pat_l] == pattern {
            return Some(i);
        }
        i += 1;
    }
    None
}

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

/// Two strings are anagrams if they are made of the same letters
/// arranged differently (ignoring the case).
pub fn is_anagram(str1: &str, str2: &str) -> bool {
    let mut a: Vec<char> = str1
        .chars()
        .filter(|ch| *ch != ' ')
        .map(|ch| ch.to_ascii_lowercase())
        .collect();
    let mut b: Vec<char> = str2
        .chars()
        .filter(|ch| *ch != ' ')
        .map(|ch| ch.to_ascii_lowercase())
        .collect();
    a.sort();
    b.sort();
    a == b
}

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

/// Determine whether the string is palindrome
pub fn is_palindrome(text: &str) -> bool {
    text.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .eq(text
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .rev())
}

/// Remove duplicates from sentence
pub fn remove_duplicates(text: String) -> String {
    let modified_text_vec = text.split_whitespace().collect::<Vec<&str>>();
    let modified_text_vec_deduped = {
        let mut values: Vec<&str> = vec![];
        for t in modified_text_vec {
            if !values.contains(&t) {
                values.push(t);
            }
        }
        values
    };

    modified_text_vec_deduped.join(" ")
}

pub fn reverse_words(text: String) -> String {
    text.split(' ').rev().collect()
}

pub mod search;

/// This function will convert all lowercase letters to uppercase letters and vice versa
pub fn swap_case(text: String) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            }
        })
        .fold(String::new(), |mut s, ch| -> String {
            s.push(ch);
            s
        })
}

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
    use super::*;
    use std::collections::HashMap;
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

    #[test]
    fn brute_force_which_works() {
        let test_set = [
            ["a", "aa", "-1"],
            ["a", "a", "0"],
            ["ba", "b", "0"],
            ["bba", "bb", "0"],
            ["bbca", "c", "2"],
            ["ab", "b", "1"],
        ];

        for test in test_set.iter() {
            let actual = brute_force(test[0], test[1]);
            assert_eq!(
                test[2].parse::<isize>().unwrap(),
                match actual {
                    Some(val) => val as isize,
                    None => -1,
                }
            );
        }
    }
}
