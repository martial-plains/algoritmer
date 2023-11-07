use alloc::vec::Vec;

/// Searches for occurrences of a "word" w within a main "text string"
///
/// The Knuth-Morris-Pratt Algorithm for finding a pattern within a piece of text
/// with complexity O(n + m)
///
/// # Arguments
///
/// * `text` - The text to be searched
/// * `pattern` - The word sought
///
/// # References
///
/// [Knuth-Morris-Pratt Algorithm](https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm)
///
/// # Panics
///
/// Panics if `pattern` is empty
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn kmp_check(text: &str, pattern: &str) -> bool {
    let mut i = 0; // The position of the current character in `s`
    let mut j = 0; // The position of the current character in `pat`
    let failure = get_failure_array(pattern).unwrap();

    while i < text.len() {
        if let (Some(p_i), Some(t_i)) = (pattern.chars().nth(j), text.chars().nth(i)) {
            if p_i == t_i {
                if j == pattern.len() - 1 {
                    return true;
                }
                j += 1;
            } else if j > 0 {
                j = failure[j - 1];
                continue;
            }
            i += 1;
        } else {
            return false;
        }
    }

    false
}

/// Calculates the new index if the comparison fails else returns
/// None
///
/// # Arguments
///
/// * `pattern` - The pattern to be analyzed
///
/// # Returns
///
/// Returns an array of integers (the table to be filled) or `None`
fn get_failure_array(p: &str) -> Option<Vec<usize>> {
    let mut t: Vec<usize> = alloc::vec![0];

    let mut pos = 1; // The current position we are computing in `t`
    let mut cnd = 0; // The zero-based index in `w` of the next character of the current candidate substring

    while pos < p.len() {
        if p.chars().nth(pos)? == p.chars().nth(cnd)? {
            cnd += 1;
        } else if cnd > 0 {
            cnd = t[cnd - 1];
            continue;
        }
        pos += 1;
        t.push(cnd);
    }
    Some(t)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("abc1abc12", "alskfjaldsabc1abc1abc12k23adsfabcabc")]
    #[test_case("ABABX", "ABABZABABYABABX")]
    #[test_case("AAAB", "ABAAAAAB")]
    #[test_case("abcdabcy", "abcxabcdabxabcdabcdabcy")]
    fn test_kmp_check(pattern: &str, text: &str) {
        assert!(kmp_check(text, pattern));
    }

    #[test_case("aabaabaaa", &[0, 1, 0, 1, 2, 3, 4, 5, 2])]
    fn test_get_failure_array(pattern: &str, array: &[usize]) {
        unsafe {
            assert_eq!(get_failure_array(pattern).unwrap_unchecked(), array);
        }
    }
}
