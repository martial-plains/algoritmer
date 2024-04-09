//! This module provides functions to convert characters to Morse code and vice versa,
//! as well as functions to encrypt and decrypt text using Morse code.

use alloc::{string::String, vec::Vec};
use hashbrown::HashMap;

use crate::macros::hashmap;

/// Represents the different versions of Morse code.
#[derive(Debug, Default)]
pub enum Version {
    /// The default version, which is the International Morse code.
    #[default]
    International,
}

/// A struct to represent Morse code with a specific version.
#[derive(Debug, Default)]
pub struct MorseCode {
    /// The version of Morse code used.
    version: Version,
}

impl MorseCode {
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
    pub fn encrypt(&self, text: &str) -> String {
        let text_lowered = text.to_lowercase();
        let words = text_lowered.split(' ').collect::<Vec<_>>();
        let mut morse = String::new();

        for (index, word) in words.iter().enumerate() {
            for (index, character) in word.char_indices() {
                morse.push_str(
                    self.morse_table()
                        .get(&character)
                        .unwrap_or(&String::from(character).as_str()),
                );
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
    pub fn decrypt(&self, text: &str) -> String {
        let mut decrypted_text = String::new();

        let splits = text
            .split(char::is_whitespace)
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        for split in &splits {
            if self
                .morse_table()
                .values()
                .collect::<Vec<_>>()
                .contains(&split)
            {
                for (key, value) in &self.morse_table() {
                    if split == value {
                        decrypted_text.push(*key);
                    }
                }
            } else if *split != "/" {
                decrypted_text.push_str(split);
            }

            if *split == "/" {
                decrypted_text.push(' ');
            }
        }

        decrypted_text
    }

    /// Returns a mapping of characters to their corresponding Morse code strings.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithms::ciphers::morse_code::MorseCode;
    ///
    /// let morse = MorseCode::default();
    /// let table = morse.morse_table();
    /// println!("{:?}", table);
    /// ```
    #[must_use]
    pub fn morse_table(&self) -> HashMap<char, &'static str> {
        match self.version {
            Version::International => Self::international_morse_table(),
        }
    }

    fn international_morse_table() -> HashMap<char, &'static str> {
        hashmap! {
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
        }
    }
}

/// Encrypts a text by converting each character to International Morse code.
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
    let morse = MorseCode::default();
    morse.encrypt(text)
}

/// Decrypts a text by converting each Morse code to its character equivalent in International morse code.
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
    let morse = MorseCode::default();
    morse.decrypt(text)
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
