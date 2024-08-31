use core::iter::zip;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

/// Jaro–Winkler distance is a string metric measuring an edit distance between two
/// sequences.
/// Output value is between 0.0 and 1.0.
///
/// # Arguments
///
/// * `str1` - first string
/// * `str2` - second string
///
/// # Example
///
/// ```
/// use algoritmer::strings::jaro_winkler;
///
/// println!("{}", jaro_winkler("martha", "marhta"));
/// println!("{}", jaro_winkler("CRATE", "TRACE"));
/// println!("{}", jaro_winkler("test", "dbdbdbdb"));
/// println!("{}", jaro_winkler("test", "test"));
/// println!("{}", jaro_winkler("hello world", "HeLLo W0rlD"));
/// println!("{}", jaro_winkler("test", ""));
/// println!("{}", jaro_winkler("hello", "world"));
/// println!("{}", jaro_winkler("hell**o", "*world"));
///
/// ```
///
/// # References
///
/// [Jaro–Winkler Algorithm](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance)
#[must_use]
#[allow(clippy::pedantic)]
pub fn jaro_winkler(str1: &str, str2: &str) -> f64 {
    let get_matched_chars = |_str1: &str, _str2: &str| -> String {
        let istr1 = _str1;
        let mut istr2 = _str2.to_string();
        let mut matched_chars = Vec::new();

        let limit = ((_str1.len().min(str2.len())) as f64 / 2.0) as usize;

        for (i, l) in istr1.chars().enumerate() {
            let left = (i as isize - limit as isize).max(0) as usize;
            let right = (i + limit + 1).min(istr2.len());

            if istr2
                .chars()
                .skip(left)
                .take(right)
                .collect::<String>()
                .contains(l)
            {
                matched_chars.push(l);
                let l_index = istr2.find(l).unwrap();
                istr2 = format!("{} {}", &istr2[..l_index], &istr2[l_index + 1..]);
            }
        }

        matched_chars.into_iter().collect()
    };

    // Matching characters
    let matched_chars_1 = get_matched_chars(str1, str2);
    let matched_chars_2 = get_matched_chars(str2, str1);

    let match_count = matched_chars_1.len();

    // Transposition
    let transpositions = f64::floor(
        zip(matched_chars_1.chars(), matched_chars_2.chars())
            .filter(|(a, b)| a != b)
            .count() as f64
            / 2.,
    ) as usize;

    if match_count == 0 {
        return 0.;
    }
    let jaro = {
        let match_count_f = match_count as f64;
        1.0 / 3.0
            * (match_count_f / str1.len() as f64
                + match_count_f / str2.len() as f64
                + (match_count_f - transpositions as f64) / match_count_f)
    };

    // Prefix
    let mut prefix_length = 0;
    let mut bool_list = Vec::new();

    if str1.len() == str2.len() {
        for (c1, c2) in str1.chars().zip(str2.chars()) {
            if c1 == c2 {
                bool_list.push(true);
            } else {
                bool_list.push(false);
            }
        }
        if bool_list.contains(&false) {
            prefix_length += bool_list.iter().position(|&x| !x).unwrap_or(0);
        }
    }

    jaro + 0.1 * prefix_length as f64 * (1. - jaro)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("martha", "marhta", 0.961_111_111_111_111_1)]
    #[test_case("CRATE", "TRACE", 0.733_333_333_333_333_4)]
    #[test_case("test", "dbdbdbdb", 0.0)]
    #[test_case("test", "test", 1.0)]
    #[test_case("hello world", "HeLLo W0rlD", 0.636_363_636_363_636_4)]
    #[test_case("test", "", 0.0)]
    #[test_case("hello", "world", 0.466_666_666_666_666_6)]
    #[test_case("hell**o", "*world", 0.436_507_936_507_936_5)]
    #[allow(clippy::float_cmp)]
    fn test_jaro_winkler(str1: &str, str2: &str, expected: f64) {
        assert_eq!(jaro_winkler(str1, str2), expected);
    }
}
