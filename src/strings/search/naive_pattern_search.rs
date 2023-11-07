use alloc::vec::Vec;

/// A simple and inefficient way to see where one string occurs inside another
/// is to check each place it could be, one by one, to see if it's there.
///
/// This algorithm tries to find the pattern from every position of
/// the main string if pattern is found from position i it adds it to
/// the answer and does the same for position i+1
///
/// Complexity : O(n*m)
///     n=length of main string
///     m=length of pattern string
///
/// # Examples
///
/// ```rust
/// use algorithms::strings::search::naive_pattern_search;
///
/// println!("{:?}", naive_pattern_search("ABAAABCDBBABCDDEBCABC", "ABC"));
/// println!("{:?}", naive_pattern_search("ABC", "ABAAABCDBBABCDDEBCABC"));
/// println!("{:?}", naive_pattern_search("", "ABC"));
/// println!("{:?}", naive_pattern_search("TEST", "TEST"));
/// println!("{:?}", naive_pattern_search("ABCDEGFTEST", "TEST"));
/// ```
///
/// # References
///
/// [Naive Pattern Search](https://en.wikipedia.org/wiki/Naive_pattern_search)
///
/// # Panics
///
/// Panics if `s` or `pattern` is empty
#[must_use]
pub fn naive_pattern_search<'a>(s: &'a str, pattern: &'a str) -> Vec<usize> {
    let pat_len = pattern.len();
    let mut positions = Vec::new();

    if pat_len > s.len() {
        return alloc::vec![];
    }

    for i in 0..=(s.len() - pat_len) {
        let mut match_found = true;
        for j in 0..pat_len {
            if s.chars().nth(i + j).unwrap() != pattern.chars().nth(j).unwrap() {
                match_found = false;
                break;
            }
        }
        if match_found {
            positions.push(i);
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case(("ABCDEFG", "DE"))]
    fn test_naive_pattern_search(data: (&str, &str)) {
        let actual = naive_pattern_search(data.0, data.1);
        assert_eq!(actual, [3]);
    }
}
