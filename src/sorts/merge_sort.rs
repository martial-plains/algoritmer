fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    // Create temporary vectors to support the merge.
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();

    // Indexes to track the positions while merging.
    let mut l = 0;
    let mut r = 0;

    for v in arr {
        // Choose either the smaller element, or from whichever vec is not exhausted.
        if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]) {
            *v = left_half[l];
            l += 1;
        } else {
            *v = right_half[r];
            r += 1;
        }
    }
}

/// Merge sort implementation.
///
/// # Arguments
///
/// * `arr` - The array to sort.
///
/// # Performance
///
/// ## Time Complexity
///
/// - Best: O(n*log n)
/// - Worst: O(n*log n)
/// - Average: O(n*log n)
///
/// ## Space Complexity
///
/// O(n)
///
/// # Stability
///
/// Stable
pub fn merge_sort<T>(arr: &mut [T])
where
    T: Ord + Copy,
{
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        // Sort the left half recursively.
        merge_sort(&mut arr[..mid]);
        // Sort the right half recursively.
        merge_sort(&mut arr[mid..]);
        // Combine the two halves.
        merge(arr, mid);
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use test_case::test_case;

    use super::*;

    #[test_case( vec![25, 26, 22, 24, 27, 23, 21],  vec![21, 22, 23, 24, 25, 26, 27])]
    #[test_case( vec![26, 17, 20, 11, 23, 21, 13, 18, 24, 14, 12, 22, 16, 16, 15, 19, 25],  vec![11, 12, 13, 14, 15, 16, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26])]
    fn test_shell_sort(mut arr: Vec<isize>, expected: Vec<isize>) {
        merge_sort(&mut arr);
        let actual = arr;
        assert_eq!(actual, expected);
    }
}
