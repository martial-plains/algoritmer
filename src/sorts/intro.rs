use super::{heap, insertion};

/// Introsort implementation.
///
/// This function performs introspective sort, which is a hybrid sorting algorithm
/// that combines quicksort, heapsort, and insertionsort to optimize sorting performance.
/// It switches between these algorithms based on the depth limit and the size of the array.
///
/// # Arguments
///
/// * `arr` - The array to be sorted.
/// * `depth_limit` - The recursion depth limit, used to determine when to switch to heapsort.
///
/// # Examples
///
/// ```
/// use algoritmer::sorts::intro;
///
/// let mut arr = [5, 2, 8, 1, 9, 3, 4, 6];
/// let depth_limit = (arr.len() as f64).log2() as usize * 2;
/// intro(&mut arr, depth_limit);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6, 8, 9]);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Complexity
///
/// The average-case time complexity of introsort is **O(n log n)**,
/// where **n** is the size of the input array.
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Introsort)
pub fn intro<T: PartialOrd + Clone>(arr: &mut [T], depth_limit: usize) {
    let len = arr.len();

    if len < 16 {
        insertion(arr);
    } else if depth_limit == 0 {
        heap(arr, |a, b| a > b);
    } else {
        let pivot_index = partition(arr);
        intro(&mut arr[..pivot_index], depth_limit - 1);
        intro(&mut arr[pivot_index + 1..], depth_limit - 1);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    let mut store_index = 0;

    for i in 0..arr.len() - 1 {
        if arr[i] < arr[arr.len() - 1] {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }
    arr.swap(store_index, arr.len() - 1);
    store_index
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn test_empty_array() {
        let mut data: Vec<i32> = vec![];
        let depth_limit = 2;
        intro(&mut data, depth_limit);
        assert_eq!(data, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut data = vec![42];
        let depth_limit = 2;
        intro(&mut data, depth_limit);
        assert_eq!(data, vec![42]);
    }

    #[test]
    fn test_sorted_array() {
        let mut data = vec![1, 2, 3, 4, 5];
        let depth_limit = 2;
        intro(&mut data, depth_limit);
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted_array() {
        let mut data = vec![5, 4, 3, 2, 1];
        let depth_limit = 2;
        intro(&mut data, depth_limit);
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unsorted_array() {
        let mut data = vec![9, 7, 5, 3, 1, 4, 8, 6, 2];
        let depth_limit = (data.len() as f64).log2() as usize * 2;
        intro(&mut data, depth_limit);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_with_duplicates() {
        let mut data = vec![3, 3, 2, 1, 2, 1, 3];
        let depth_limit = (data.len() as f64).log2() as usize * 2;
        intro(&mut data, depth_limit);
        assert_eq!(data, vec![1, 1, 2, 2, 3, 3, 3]);
    }
}
