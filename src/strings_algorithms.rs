use std::{collections::HashMap, str};

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

/// wiki: https://en.wikipedia.org/wiki/Pangram
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
