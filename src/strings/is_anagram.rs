use alloc::vec::Vec;

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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Silent", "Listen", true)]
    #[test_case("This is    a      string", "Is     this a string", true)]
    #[test_case("There", "Their", false)]
    fn check_anagram(string1: &str, string2: &str, expected: bool) {
        let actual = is_anagram(string1, string2);
        assert_eq!(expected, actual)
    }
}
