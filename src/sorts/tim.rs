use core::cmp::min;

use alloc::vec;

/// Timsort implementation in Rust.
///
/// Timsort is a hybrid sorting algorithm derived from merge sort and insertion sort.
/// It works by dividing the array into small chunks (runs) and sorting them using
/// insertion sort, then merging these sorted runs using a modified merge process.
///
/// Complexity:
/// - **Best case:** O(n) when the array is already sorted.
/// - **Average case:** O(n log n).
/// - **Worst case:** O(n log n).
///
/// For more details, visit:
/// [Wikipedia - Timsort](https://en.wikipedia.org/wiki/Timsort)
///
/// # Parameters
/// - `arr`: A mutable slice of elements to be sorted.
///
/// # Type Constraints
/// - `T`: Clone + PartialOrd + Default.
///
/// # Example Usage
/// ```
/// use algoritmer::sorts::tim;
///
/// let mut array = vec![5, 2, 9, 1, 5, 6];
/// tim(&mut array);
/// assert_eq!(array, vec![1, 2, 5, 5, 6, 9]);
/// ```
pub fn tim<T: Clone + PartialOrd + Default>(arr: &mut [T]) {
    let n = arr.len();
    let run = 32;

    for i in (0..n).step_by(run) {
        let right = min(i + run - 1, n - 1);
        insertion_sort(arr, i, right);
    }

    let mut size = run;
    while size < n {
        let mut left = 0;
        while left < n {
            let mid = left + size - 1;
            let right = min(left + 2 * size - 1, n - 1);

            if mid < right {
                merge(arr, left, mid, right);
            }

            left += 2 * size;
        }
        size *= 2;
    }
}

fn insertion_sort<T: Clone + PartialOrd>(arr: &mut [T], left: usize, right: usize) {
    for i in (left + 1)..=right {
        let tmp = arr[i].clone();
        let mut j = i;

        while j > left && arr[j - 1] > tmp {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }

        arr[j] = tmp;
    }
}

fn merge<T: Default + Clone + PartialOrd>(arr: &mut [T], l: usize, m: usize, r: usize) {
    let (mut x, mut y, mut i, mut j, mut k) = (0, 0, 0, 0, 0);
    let len1: usize = m - l + 1;
    let len2 = r - m;
    let mut left = vec![T::default(); len1];
    let mut right = vec![T::default(); len2];

    while x < len1 {
        left[x] = arr[l + x].clone();
        x += 1;
    }

    while y < len2 {
        right[y] = arr[(m + 1) + y].clone();
        y += 1;
    }

    while i < len1 && j < len2 {
        if left[i] <= right[j] {
            arr[l + k] = left[i].clone();
            i += 1;
        } else {
            arr[l + k] = right[j].clone();
            j += 1;
        }

        k += 1;
    }

    while i < len1 {
        arr[l + k] = left[i].clone();
        k += 1;
        i += 1;
    }

    while j < len2 {
        arr[l + k] = right[j].clone();
        k += 1;
        j += 1;
    }
}
