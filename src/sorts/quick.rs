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
pub fn quick<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        quick(arr, lo, p - 1);
        quick(arr, p + 1, hi);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::vec;
    use alloc::vec::Vec;

    use test_case::test_case;

    #[test_case( vec![25, 26, 22, 24, 27, 23, 21],  vec![21, 22, 23, 24, 25, 26, 27])]
    #[test_case( vec![26, 17, 20, 11, 23, 21, 13, 18, 24, 14, 12, 22, 16, 16, 15, 19, 25],  vec![11, 12, 13, 14, 15, 16, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26])]
    fn test_quicksort(mut arr: Vec<isize>, expected: Vec<isize>) {
        let last_idx = arr.len() - 1;
        quick(&mut arr, 0, last_idx as isize);
        let actual = arr;
        assert_eq!(actual, expected);
    }
}
