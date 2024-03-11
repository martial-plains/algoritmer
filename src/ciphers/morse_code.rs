//! This module provides functions to convert characters to Morse code and vice versa,
//! as well as functions to encrypt and decrypt text using Morse code.

use alloc::{string::String, vec::Vec};
use hashbrown::HashMap;

use crate::macros::hashmap;

lazy_static::lazy_static! {
    static ref MORSE_TABLE: HashMap<char, &'static str> = hashmap! {
        'a' => ".-",
        'b' => "-...",
        'c' => "-.-.",
        'd' => "-..",
        'e' => ".",
        'f' => "..-.",
        'g' => "--.",
        'h' => "....",
        'i' => "..",
        'j' => ".---",
        'k' => "-.-",
        'l' => ".-..",
        'm' => "--",
        'n' => "-.",
        'o' => "---",
        'p' => ".--.",
        'q' => "--.-",
        'r' => ".-.",
        's' => "...",
        't' => "-",
        'u' => "..-",
        'v' => "...-",
        'w' => ".--",
        'x' => "-..-",
        'y' => "-.--",
        'z' => "--..",
        '1' => ".----",
        '2' => "..---",
        '3' => "...--",
        '4' => "....-",
        '5' => ".....",
        '6' => "-....",
        '7' => "--...",
        '8' => "---..",
        '9' => "----.",
        '0' => "-----",
        '.' => ".-.-.-",
        ',' => "--..--",
        '?' => "..--..",
        '\'' => ".----.",
        '!' => "-.-.--",
        '/' => "-..-.",
        '(' => "-.--.",
        ')' => "-.--.-",
        '&' => ".-...",
        ':' => "---...",
        ';' => "-.-.-.",
        '=' => "-...-",
        '+' => ".-.-.",
        '-' => "-....-",
        '_' => "..--.-",
        '"' => ".-..-.",
        '$' => "...-..-",
        '@' => ".--.-.",
        '¿' => "..-.-",
        '¡' => "--...-",
    };
}

/// Encrypts a text by converting each character to Morse code.
///
/// # Arguments
///
/// * `text` - A string slice that represents the text to be encrypted.
///
/// # Returns
///
/// * A string that represents the encrypted text.
#[must_use]
pub fn encrypt(text: &str) -> String {
    let text_lowered = text.to_lowercase();
    let words = text_lowered.split(' ').collect::<Vec<_>>();
    let mut morse = String::new();

    for (index, word) in words.iter().enumerate() {
        for (index, character) in word.char_indices() {
            morse.push_str(MORSE_TABLE.get(&character).unwrap_or(&String::from(character).as_str()));
            if index == word.len() {
                morse.push_str("");
            } else {
                morse.push_str("   ");
            }
        }

        if index == words.len() - 1 {
            morse.push_str("");
        } else {
            morse.push_str(" /    ");
        }
    }

    morse
}

/// Decrypts a text by converting each Morse code to its character equivalent.
///
/// # Arguments
///
/// * `text` - A string slice that represents the Morse code to be decrypted.
///
/// # Returns
///
/// * A string that represents the decrypted text.
#[must_use]
pub fn decrypt(text: &str) -> String {
    let mut decrypted_text = String::new();

    let splits = text
        .split(char::is_whitespace)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    for split in &splits {
        if MORSE_TABLE.values().collect::<Vec<_>>().contains(&split) {
            for (key, value) in MORSE_TABLE.iter() {
                if split == value {
                    decrypted_text.push(*key);
                }
            }
        } else {
            if !(*split == "/") {
                decrypted_text.push_str(split)
            }
        }

        if *split == "/" {
            decrypted_text.push(' ');
        }
    }

    decrypted_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_morse() {
        let text = "01234567890";
        let encrypted = encrypt(text);
        let decrypted = decrypt(&encrypted);
        assert_eq!(text, decrypted);

        let text = "abcdefghijklmnopqrstuvwxyz";
        let encrypted = encrypt(text);
        let decrypted = decrypt(&encrypted);
        assert_eq!(text, decrypted);
    }

    #[test]
    pub fn test_hello_world() {
        let text = "hello world!";
        let encrypted = encrypt(text);
        let decrypted = decrypt(&encrypted);
        assert_eq!(text, decrypted);
    }


    #[test]
    pub fn test_hello_world_with_emojis() {
        let text = "hello world 👋!";
        let encrypted = encrypt(text);
        let decrypted = decrypt(&encrypted);
        assert_eq!(text, decrypted);
    }
}
