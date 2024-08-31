use core::cmp::Ordering;

/// Binary search is an algorithm that locates a target in a sorted array.
///
/// It compares the target to the middle element and eliminates half of
/// the array where the target can't be. This process repeats until the
/// target is found or the search space is empty, indicating the target
/// isn't in the array.
///
/// # Arguments
///
/// * `arr` - The array being searched. It must be sorted in ascending order.
/// * `key` - The target value being searched for.
///
/// # Returns
///
/// The index of the target value if found, or `None` if not found.
///
/// # Examples
///
/// ```
/// use algoritmer::search::binary;
///
/// let arr = [1, 3, 5, 7, 9];
/// assert_eq!(binary(&arr, &5), Some(2));
///
/// let arr = [1, 3, 5, 7, 9];
/// assert_eq!(binary(&arr, &2), None);
///
/// let arr: [char; 6] = ['a', 'b', 'c', 'd', 'e', 'f'];
/// assert_eq!(binary(&arr, &'d'), Some(3));
/// ```
///
/// # Performance
///
/// ## Time Complexity
///
/// - Best: O(1) - when the target value is at the middle element of the array.
/// - Worst: O(log n) - when the target value is at the extreme ends of the array.
/// - Average: O(log n) - logarithmic time complexity based on the size of the array.
///
/// ## Space Complexity
///
/// O(1) - the algorithm uses a constant amount of additional space.
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Binary_search_algorithm)
pub fn binary<T>(arr: &[T], key: &T) -> Option<usize>
where
    T: Ord,
{
    let mut lo = 0; // set the lowest point of the array.
    let mut hi = arr.len() - 1; // set the highest point of the array.

    while lo < hi + 1 {
        let m = lo + (hi - lo) / 2; // set the middle point of the array.

        match key.cmp(&arr[m]) {
            Ordering::Less => hi = m - 1,
            Ordering::Greater => lo = m + 1,
            Ordering::Equal => return Some(m),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(binary(&arr, &1), Some(0));
        assert_eq!(binary(&arr, &2), Some(1));
        assert_eq!(binary(&arr, &3), Some(2));
        assert_eq!(binary(&arr, &4), Some(3));
        assert_eq!(binary(&arr, &5), Some(4));
        assert_eq!(binary(&arr, &6), Some(5));
        assert_eq!(binary(&arr, &7), Some(6));
        assert_eq!(binary(&arr, &8), Some(7));
        assert_eq!(binary(&arr, &9), Some(8));
        assert_eq!(binary(&arr, &10), None);
    }
}
