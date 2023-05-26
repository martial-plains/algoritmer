/// Quicksort is a sorting algorithm that uses a divide and conquer strategy.
///
/// # Arguments
///
/// * `arr` - The array to sort.
/// * `lo` - The left index of the array.
/// * `hi` - The right index of the array.
///
/// # Performance
///
/// ## Time Complexity
///
/// - Best: O(n*log n)
///     - Best case: It occurs when the pivot element is always in or near the middle of the array.
/// - Worst: O(n²)
///    - Worst case: It occurs when the pivot is either the last or first element of the array.
///    - This condition leads to the case in which the pivot element lies in an extreme end of the sorted array.
/// - Average: O(n*log n)
///     - Average case: It occurs when the above conditions are not met.
///
/// ## Space Complexity
///
/// - 0(log n)
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Quicksort)
pub fn quick<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let pivot_index = partition(arr, lo, hi);
        if pivot_index > 0 {
            quick(arr, lo, pivot_index - 1);
        }
        quick(arr, pivot_index + 1, hi);
    }
}

fn partition<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot_index = hi;
    let mut i = lo;
    for j in lo..hi {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

#[cfg(test)]
mod tests {

    use alloc::vec;
    use alloc::vec::Vec;

    use test_case::test_case;

    use crate::sorts::quick;

    #[test_case( vec![25, 26, 22, 24, 27, 23, 21],  vec![21, 22, 23, 24, 25, 26, 27])]
    #[test_case( vec![26, 17, 20, 11, 23, 21, 13, 18, 24, 14, 12, 22, 16, 16, 15, 19, 25],  vec![11, 12, 13, 14, 15, 16, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26])]
    fn test_quicksort(mut arr: Vec<usize>, expected: Vec<usize>) {
        let last_idx = arr.len() - 1;
        quick(&mut arr, 0, last_idx);
        let actual = arr;
        assert_eq!(actual, expected);
    }
}
