use alloc::vec::Vec;

/// Searches for occurrences of a "word" w within a main "text string"
///
/// The Knuth-Morris-Pratt Algorithm for finding a pattern within a piece of text
/// with complexity O(n + m)
///
/// # Wikipedia
///
/// <https://en.wikipedia.org/wiki/Knuth–Morris–Pratt_algorithm>
///
/// # Arguments
///
/// * `s` - The text to be searched
/// * `w` - The word sought
pub fn kmp_check(s: &str, pat: &str) -> bool {
    let mut i = 0; // The position of the current character in `s`
    let mut j = 0; // The position of the current character in `pat`
    let failure = get_failure_array(pat).unwrap();

    while i < s.len() {
        if pat.chars().nth(j).unwrap() == s.chars().nth(i).unwrap() {
            if j == pat.len() - 1 {
                return true;
            }
            j += 1;
        } else if j > 0 {
            j = failure[j - 1];
            continue;
        }
        i += 1;
    }

    false
}

/// Calculates the new index if the comparison fails else returns
/// original index
///
/// # Arguments
///
/// * `pattern` - The pattern to be analyzed
///
/// # Returns
///
/// Returns an array of integers (the table to be filled)
fn get_failure_array(p: &str) -> Option<Vec<usize>> {
    let mut t: Vec<usize> = Vec::default();
    t.push(0);

    let mut pos = 1; // The current position we are computing in `t`
    let mut cnd = 0; // The zero-based index in `w` of the next character of the current candidate substring

    while pos < p.len() {
        if p.chars().nth(pos)? == p.chars().nth(cnd)? {
            cnd += 1
        } else if cnd > 0 {
            cnd = t[cnd - 1];
            continue;
        }
        pos += 1;
        t.push(cnd)
    }
    Some(t)
}

#[cfg(test)]
mod tests {
    use super::kmp_check;

    #[test]
    fn test_1() {
        let pattern = "abc1abc12";
        let text1 = "alskfjaldsabc1abc1abc12k23adsfabcabc";
        //let text2 = "alskfjaldsk23adsfabcabc";
        assert!(kmp_check(text1, pattern));
        //assert!(kmp_search(text2, pattern) == false);
    }
}
