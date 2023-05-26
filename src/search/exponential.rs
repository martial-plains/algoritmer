use core::cmp::Ordering;

/// Exponential Search Algorithm (Struzik)
///
/// The algorithm is described by Struzik, J. (1983).
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
/// # Performance
///
/// ## Time Complexity
///
/// - Best Case: O(1)
/// - Worst Case: O(log i)
/// - Average Case: O(log i)
///
/// ## Space Complexity
///
/// - O(1)
///
/// - Auxiliary Space: O(1)
///
/// # Examples
///
/// ```
/// use algorithms::search::exponential;
///
/// let array = [1, 3, 5, 7, 9, 11, 13];
/// let key = 7;
/// let index = exponential(&array, key);
/// assert_eq!(index, Some(3));
/// ```
///
/// ```
/// use algorithms::search::exponential;
///
/// let array = [1, 3, 5, 7, 9, 11, 13];
/// let key = 10;
/// let index = exponential(&array, key);
/// assert_eq!(index, None);
/// ```
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Exponential_search)
#[doc(alias("struzik"))]
pub fn exponential<T>(array: &[T], key: T) -> Option<usize>
where
    T: Ord,
{
    let mut left = 0;
    let mut right = array.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;

        match array[mid].cmp(&key) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid - 1,
            Ordering::Equal => return Some(mid),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponential_search_test() {
        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(exponential(&array, 1), Some(0));
        assert_eq!(exponential(&array, 2), Some(1));
        assert_eq!(exponential(&array, 3), Some(2));
        assert_eq!(exponential(&array, 4), Some(3));
        assert_eq!(exponential(&array, 5), Some(4));
        assert_eq!(exponential(&array, 6), Some(5));
        assert_eq!(exponential(&array, 7), Some(6));
        assert_eq!(exponential(&array, 8), Some(7));
        assert_eq!(exponential(&array, 9), Some(8));
        assert_eq!(exponential(&array, 10), Some(9));
        assert_eq!(exponential(&array, 11), None);
    }
}
