use core::cmp::Ordering;

/// An implementation of ternary search algorithm. This is a divide and conquer algorithm. It does this by dividing the array into three parts, and then finding the desired index.
///
/// # Arguments
///
/// * `target` - value to search for
/// * `arr` - array to search in
/// * `start` - index to start searching from
/// * `end` - index to end searching at
pub fn ternary_search<T>(target: &T, arr: &[T], mut start: usize, mut end: usize) -> Option<usize>
where
    T: Ord,
{
    if arr.is_empty() {
        return None;
    }

    while start <= end {
        let mid1: usize = start + (end - start) / 3;
        let mid2: usize = end - (end - start) / 3;

        match target.cmp(&arr[mid1]) {
            Ordering::Less => end = mid1 - 1,
            Ordering::Equal => return Some(mid1),
            Ordering::Greater => match target.cmp(&arr[mid2]) {
                Ordering::Greater => start = mid2 + 1,
                Ordering::Equal => return Some(mid2),
                Ordering::Less => {
                    start = mid1 + 1;
                    end = mid2 - 1;
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
        assert_eq!(ternary_search(&1, &arr, 0, arr.len() - 1), Some(0));
        assert_eq!(ternary_search(&10, &arr, 0, arr.len() - 1), Some(9));
        assert_eq!(ternary_search(&11, &arr, 0, arr.len() - 1), None);
    }
}