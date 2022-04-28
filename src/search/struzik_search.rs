use core::cmp::Ordering;

/// Struzik Search Algorithm (Exponential)
///
/// The algorithm is described in [Struzik, J. (1983).
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
/// - Axuiliary Space: O(1)
pub fn struzik_search<T>(array: &[T], key: T) -> Option<usize>
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
    fn struzik_search_test() {
        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(struzik_search(&array, 1), Some(0));
        assert_eq!(struzik_search(&array, 2), Some(1));
        assert_eq!(struzik_search(&array, 3), Some(2));
        assert_eq!(struzik_search(&array, 4), Some(3));
        assert_eq!(struzik_search(&array, 5), Some(4));
        assert_eq!(struzik_search(&array, 6), Some(5));
        assert_eq!(struzik_search(&array, 7), Some(6));
        assert_eq!(struzik_search(&array, 8), Some(7));
        assert_eq!(struzik_search(&array, 9), Some(8));
        assert_eq!(struzik_search(&array, 10), Some(9));
        assert_eq!(struzik_search(&array, 11), None);
    }
}
