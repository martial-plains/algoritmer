use alloc::{str::pattern::Pattern, string::String, vec::Vec};
use hashbrown::HashSet;

/// Trait for removing duplicates from a sentence with a specified separator.
///
/// This trait provides a method to remove duplicate elements from a string based on a given separator.
pub trait RemoveDuplicates<P>
where
    P: Pattern,
{
    /// Removes duplicates from a string using the specified separator.
    ///
    /// # Arguments
    ///
    /// * `separator` - A string slice that specifies the separator used to split and join the text.
    ///
    /// # Returns
    ///
    /// Returns a new `String` without duplicate elements, joined by the specified separator.
    ///
    /// # Examples
    ///
    /// ```
    /// use algoritmer::strings::RemoveDuplicates;
    ///
    /// let text = "hello,world,hello,rust";
    /// let result = text.removed_duplicates(",");
    /// assert_eq!(result, "hello,world,rust");
    ///
    /// let text_space = "hello world hello rust";
    /// let result_space = text_space.removed_duplicates(" ");
    /// assert_eq!(result_space, "hello world rust");
    /// ```
    fn removed_duplicates(&self, separator: P) -> String;
}

/// Implement the `RemoveDuplicates` trait for `&str`
impl<P> RemoveDuplicates<P> for &str
where
    P: Pattern + Clone,
    String: From<P>,
{
    fn removed_duplicates(&self, separator: P) -> String {
        let mut seen = HashSet::new();
        self.split(separator.clone())
            .filter(|s| seen.insert(s.trim()))
            .collect::<Vec<_>>()
            .join(&String::from(separator))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Rust is great and Java is also great", "Rust is great and Java also")]
    fn check_removed_duplicates(text: &str, expected: &str) {
        let actual = text.removed_duplicates(" ");
        assert_eq!(expected, actual);
    }
}
