use alloc::{vec, vec::Vec};
use core::cmp::min;

fn go_next(i: usize, result: &[usize], s: &str) -> bool {
    i + result[i] < s.len() && s.chars().nth(result[i]) == s.chars().nth(i + result[i])
}

/// This function implements an efficient algorithm for finding pattern occurrences in a string.
///
/// For a given pattern, it computes a value for each index, representing the maximal length substring starting from that index, which matches a prefix of the pattern.
///
/// # Performance
///
/// ## Time Complexity
///
/// - Average: O(n) - where `n` is the length of `pattern`
///
/// # See more
/// [CP Algorithms](https://cp-algorithms.com/string/z-function.html)
#[must_use]
pub fn z_function(pattern: &str) -> Vec<usize> {
    let mut result = vec![0; pattern.len()];
    let (mut left_pointer, mut right_pointer) = (0, 0);

    for i in 1..pattern.len() {
        if i <= right_pointer {
            let min_edge = min(right_pointer - i + 1, result[i - left_pointer]);
            result[i] = min_edge;
        }

        while go_next(i, &result, pattern) {
            result[i] += 1;
        }

        if i + result[i] - 1 > right_pointer {
            (left_pointer, right_pointer) = (i, i + result[i] - 1);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::z_function;
    use alloc::format;
    use test_case::test_case;

    #[test_case("abr", "abracadabra" => 2)]
    #[test_case("a", "aaaa" => 4)]
    #[test_case("xz", "zxxzxxz" => 2)]
    fn test_z_function(pattern: &str, input: &str) -> usize {
        let mut answer = 0;
        let z_result = z_function(&format!("{pattern}{input}"));

        for item in z_result {
            if item >= pattern.len() {
                answer += 1;
            }
        }

        answer
    }
}
