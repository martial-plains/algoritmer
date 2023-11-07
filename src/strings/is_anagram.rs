use alloc::vec::Vec;

/// Two strings are anagrams if they are made of the same letters
/// arranged differently (ignoring the case).
///
/// # Arguments
///
/// * `s1` - The first string
/// * `s2` - The second string
///
/// # Returns
///
/// Returns true if the strings are anagrams, false otherwise
#[must_use]
pub fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut a: Vec<char> = s1
        .chars()
        .filter(|ch| *ch != ' ')
        .map(|ch| ch.to_ascii_lowercase())
        .collect();
    let mut b: Vec<char> = s2
        .chars()
        .filter(|ch| *ch != ' ')
        .map(|ch| ch.to_ascii_lowercase())
        .collect();
    a.sort_unstable();
    b.sort_unstable();
    a == b
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Silent", "Listen", true)]
    #[test_case("This is    a      string", "Is     this a string", true)]
    #[test_case("There", "Their", false)]
    fn check_anagram(string1: &str, string2: &str, expected: bool) {
        let actual = is_anagram(string1, string2);
        assert_eq!(expected, actual);
    }
}
