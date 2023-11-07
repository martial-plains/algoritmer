use core::cmp::Ordering;

/// An implementation of ternary search algorithm. This is a divide and conquer algorithm. It does this by dividing the array into three parts, and then finding the desired index.
///
/// # Arguments
///
/// * `arr` - array to search in
/// * `key` - value to search for
///
/// # Returns
///
/// The index of the target value if found, or None if not found
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Ternary_search)
pub fn ternary<T>(arr: &[T], key: &T) -> Option<usize>
where
    T: Ord,
{
    let mut left = 0;
    let mut right = arr.len() - 1;
    if arr.is_empty() {
        return None;
    }

    while left <= right {
        let mid1: usize = left + (right - left) / 3;
        let mid2: usize = right - (right - left) / 3;

        match key.cmp(&arr[mid1]) {
            Ordering::Less => right = mid1 - 1,
            Ordering::Equal => return Some(mid1),
            Ordering::Greater => match key.cmp(&arr[mid2]) {
                Ordering::Greater => left = mid2 + 1,
                Ordering::Equal => return Some(mid2),
                Ordering::Less => {
                    left = mid1 + 1;
                    right = mid2 - 1;
                }
            },
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_search() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(ternary(&arr, &1), Some(0));
        assert_eq!(ternary(&arr, &10), Some(9));
        assert_eq!(ternary(&arr, &11), None);
    }
}
