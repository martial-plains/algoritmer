// Numbers of alphabet which we call base
const ALPHABET_SIZE: isize = 256;

// Modulus to hash a string
const MODULUS: isize = 1000003;

/// The Rabin-Karp Algorithm for finding a pattern within a piece of text
/// with complexity O(nm), most efficient when it is used with multiple patterns
/// as it is able to check if any of a set of patterns match a section of text in o(1)
/// given the precomputed hashes.
///
/// This will be the simple version which only assumes one pattern is being searched
/// for but it's not hard to modify
///
/// 1) Calculate pattern hash
///
/// 2) Step through the text one character at a time passing a window with the same
///     length as the pattern
///     calculating the hash of the text within the window compare it with the hash
///     of the pattern. Only testing equality if the hashes match
///
/// 3) If the hashes match then check if the text is equal to the pattern
///
/// # Arguments
///
/// * `text` - The text to be searched
/// * `pattern` - The word sought
///
/// # Returns
///
/// Returns true if the pattern is found in the text else false
pub fn rabin_karp_check(text: &str, pattern: &str) -> bool {
    let p_len = pattern.len();
    let t_len = text.len();

    if p_len > t_len {
        return false;
    }

    let mut p_hash = 0isize;
    let mut text_hash = 0isize;
    let mut modulus_pow = 1isize;

    // Calculating the hash of pattern and substring of text
    for i in 0..p_len {
        p_hash = (pattern.chars().nth(i).unwrap() as isize + p_hash * ALPHABET_SIZE) % MODULUS;
        text_hash = (text.chars().nth(i).unwrap() as isize + text_hash * ALPHABET_SIZE) % MODULUS;

        if i == p_len - 1 {
            continue;
        }

        modulus_pow = (modulus_pow * ALPHABET_SIZE) % MODULUS;
    }

    for i in 0..t_len - p_len + 1 {
        if text_hash == p_hash && &text[i..i + p_len] == pattern {
            return true;
        }

        if i == t_len - p_len {
            continue;
        }

        // Calculating the hash of the next character
        // Ref: <https://en.wikipedia.org/wiki/Rolling_hash>
        let curr_text = text.chars().nth(i).unwrap();
        let curr_text_char_with_p_len = text.chars().nth(i + p_len).unwrap();
        text_hash = ((((text_hash - curr_text as isize * modulus_pow) * ALPHABET_SIZE)
            + curr_text_char_with_p_len as isize)
            % MODULUS)
            + MODULUS;
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rabin_karp_search() {
        let pattern = "abc1abc12";
        let text = "alskfjaldsabc1abc1abc12k23adsfabcabc";
        let result = super::rabin_karp_check(text, pattern);
        assert!(result);
    }
}
