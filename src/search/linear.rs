/// A Rust implementation of a binary search algorithm.
///
/// # Arguments
///
/// * `arr` - The array being searched
/// * `key` - The target value being searched for
///
/// # Returns
///
/// The index of the target value if found, or None if not found
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Linear_search)
pub fn linear<T>(arr: &[T], key: T) -> Option<usize>
where
    T: PartialOrd + Copy,
{
    for (i, &v) in arr.iter().enumerate() {
        if v == key {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(linear(&arr, 1), Some(0));
        assert_eq!(linear(&arr, 10), Some(9));
        assert_eq!(linear(&arr, 11), None);
    }
}
