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

/// Searches for occurrences of a "word" w within a main "text string"
/// s
///
/// Returns an array of integers (positions in `s` at which `w` is
/// found) and an integer (number of positions)
///
/// # Wikipedia
/// https://en.wikipedia.org/wiki/Knuth–Morris–Pratt_algorithm
///
/// # Arguments
/// * `s` - The text to be searched
/// * `w` - The word sought
pub fn kmp_search(s: &str, w: &str) -> (Vec<usize>, i32) {
    let mut j = 0; // The position of the current character in `s`
    let mut k = 0; // The position of the current character in `w`
    let table = kmp_table(w);

    let mut n_p = 0;
    let mut p: Vec<usize> = Vec::new();

    while j < s.len() {
        if w.chars().nth(k).unwrap() == s.chars().nth(j).unwrap() {
            j += 1;
            k += 1;
            if k == w.len() {
                // occurrence found if only first occurrence is needed, m <- j - k may be returned here
                p.push(j - k);
                n_p += 1;
                k = table[k] as usize; // table[k] can't be -1
            } else {
                k = table[k] as usize;
            }
        }
    }
    (p, n_p as i32)
}

/// Returns an array of integers (the table to be filled)
///
/// # Arguments
/// * `w` - The word to be analyzed
pub fn kmp_table(w: &str) -> Vec<i32> {
    let mut t: Vec<i32> = Vec::default();
    let mut pos = 1; // The current position we are computing in `t`
    let mut cnd = 0; // The zero-based index in `w` of the next character of the current candidate substring

    t.push(-1);

    while pos < w.len() {
        if w.chars().nth(pos).unwrap() == w.chars().nth(cnd).unwrap() {
            t.push(t[cnd]);
        } else {
            t.push(cnd as i32);
            while w.chars().nth(pos).unwrap() != w.chars().nth(cnd).unwrap() {
                cnd = t[cnd] as usize
            }
        }
        pos += 1;
        cnd += 1;
    }
    t.push(cnd as i32); // only needed when all word occurrences are searched)
    t
}

pub fn naive_pattern_search(s: String, pattern: &str) -> Vec<i32> {
    s.char_indices()
        .map(|(i, x)| {
            let myv = pattern.chars().nth(0).unwrap();

            if x == myv {
                (i as i32, &s[i..(i + (pattern.len() - 1))])
            } else {
                (i as i32, "")
            }
        })
        .filter(|(_i, x)| pattern == *x)
        .map(|(i, _x)| i)
        .collect()
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
