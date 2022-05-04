use core::cmp::Ordering;

/// Binary search is a search algorithm that finds the position of a
/// target within a sorted array. Binary search compares the target
/// value to the element of the array. If they are not equal, the half
/// in which the target cannot lie is eliminated and the search
/// continues on the remaining half, again taking the middle element to
/// compare to the target value, and repeating this until the target
/// value is found. If the search ends with the remaining half being
/// empty, the target is not in the array.
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
/// - Best: O(1)
/// - Worst: O(log n)
/// - Average: O(log n)
///
/// ## Space Complexity
///
/// O(1)
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Insertion_sort#Variants)
pub fn binary<T>(arr: &[T], key: T) -> Option<usize>
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
        assert_eq!(binary(&arr, 1), Some(0));
        assert_eq!(binary(&arr, 2), Some(1));
        assert_eq!(binary(&arr, 3), Some(2));
        assert_eq!(binary(&arr, 4), Some(3));
        assert_eq!(binary(&arr, 5), Some(4));
        assert_eq!(binary(&arr, 6), Some(5));
        assert_eq!(binary(&arr, 7), Some(6));
        assert_eq!(binary(&arr, 8), Some(7));
        assert_eq!(binary(&arr, 9), Some(8));
        assert_eq!(binary(&arr, 10), None);
    }
}
