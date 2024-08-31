use core::iter::zip;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

/// A trait for computing the Jaro–Winkler distance between two strings.
///
/// Jaro–Winkler distance is a string metric measuring an edit distance between two
/// sequences. The output value is between 0.0 and 1.0.
///
/// # Example
///
/// ```
/// use algoritmer::strings::JaroWinkler;
///
/// let str1 = "martha";
/// let str2 = "marhta";
/// println!("{}", str1.jaro_winklered(&str2));
/// ```
///
/// # References
///
/// [Jaro–Winkler Algorithm](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance)
pub trait JaroWinkler {
    /// Calculates the Jaro–Winkler distance between the calling string and another string.
    ///
    /// # Arguments
    ///
    /// * `other` - the other string to compare against.
    ///
    /// # Returns
    ///
    /// The Jaro–Winkler distance as a `f64` value between 0.0 and 1.0.
    ///
    /// # Example
    ///
    /// ```
    /// use algoritmer::strings::JaroWinkler;
    ///
    /// let str1 = "martha";
    /// let str2 = "marhta";
    /// assert_eq!(str1.jaro_winklered(&str2), 0.9611111111111111);
    /// ```
    fn jaro_winklered(&self, other: &str) -> f64;
}

impl JaroWinkler for str {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn jaro_winklered(&self, other: &str) -> f64 {
        let get_matched_chars = |str1: &str, str2: &str| -> String {
            let mut istr2 = str2.to_string();
            let mut matched_chars = Vec::new();
            let limit = (str1.len().min(str2.len()) as f32 / 2.0) as usize;

            for (i, l) in str1.chars().enumerate() {
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

        let matched_chars_1 = get_matched_chars(self, other);
        let matched_chars_2 = get_matched_chars(other, self);

        let match_count = matched_chars_1.len();
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
                * (match_count_f / self.len() as f64
                    + match_count_f / other.len() as f64
                    + (match_count_f - transpositions as f64) / match_count_f)
        };

        let mut prefix_length = 0;
        let mut bool_list = Vec::new();

        if self.len() == other.len() {
            for (c1, c2) in self.chars().zip(other.chars()) {
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
        assert_eq!(str1.jaro_winklered(str2), expected);
    }
}
