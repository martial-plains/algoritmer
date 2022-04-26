/// Quicksort is a sorting algorithm that uses a divide and conquer strategy.
///
/// # Arguments
///
/// * `arr` - The array to sort.
/// * `left` - The left index of the array.
/// * `right` - The right index of the array.
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
/// 0(log n)
///
pub fn quicksort<T>(arr: &mut [T], left: usize, right: usize)
where
    T: Copy + Ord,
{
    if left < right {
        let pivot = partition(arr, left, right);
        quicksort(arr, left, pivot - 1);
        quicksort(arr, pivot + 1, right);
    }
}

fn partition<T>(arr: &mut [T], left: usize, right: usize) -> usize
where
    T: Copy + Ord,
{
    let pivot = arr[right];
    let mut i = left;
    for j in left..right {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, right);
    i
}
