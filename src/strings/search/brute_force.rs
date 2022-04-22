/// Find a pattern in a string by comparing the pattern to every substring
///
/// # Arguments
///
/// * `text` - Any string that might contain the pattern.
/// * `pattern` - String that we are searching for.
///
/// # Returns
///
/// Index where the pattern starts in the text
/// `None` if the pattern is not found
pub fn brute_force(text: &str, pattern: &str) -> Option<usize> {
    let pat_l = pattern.len();
    let txt_l = text.len();
    let mut i = 0;
    if pat_l > txt_l {
        return None;
    }
    while i <= txt_l - pat_l {
        if &text[i..i + pat_l] == pattern {
            return Some(i);
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn brute_force_which_works() {
        let test_set = [
            ["a", "aa", "-1"],
            ["a", "a", "0"],
            ["ba", "b", "0"],
            ["bba", "bb", "0"],
            ["bbca", "c", "2"],
            ["ab", "b", "1"],
        ];

        for test in test_set.iter() {
            let actual = brute_force(test[0], test[1]);
            assert_eq!(
                test[2].parse::<isize>().unwrap(),
                match actual {
                    Some(val) => val as isize,
                    None => -1,
                }
            );
        }
    }
}
