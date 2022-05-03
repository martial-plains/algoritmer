/// The selection sort algorithm selects the smallest element from an
/// unsorted array in each iteration and places it at the beginning of
/// the array.
///
/// # Arguments
///
/// * `array` - The array to sort.
///
/// # Performance
///
/// ## Time Complexity
///
/// - Worst: O(n²)
///     - The worst case occurs when the array is in reverse order.
/// - Best: O(n²)
///     - The best case occurs when the array is already sorted.
/// - Average: O(n²)
///     - The average case occurs when the array is randomly ordered.
///
/// ## Space Complexity
///
/// - O(1)
///
/// ## Stable
///
/// No
///
/// # See Also
///
/// wiki: <https://en.wikipedia.org/wiki/Selection_sort>
pub fn selection<T>(array: &mut [T])
where
    T: Ord,
{
    for i in 0..array.len() {
        let mut min_idx = i;
        // For `j` from `i` to the end of the array, find the smallest
        // element in the array. Where `j` is the index of the first
        // unsorted element.
        for j in i..array.len() {
            // Compare the current element with the min element.
            if array[j] < array[min_idx] {
                min_idx = j;
            }
        }
        // Swap the current element with the min element.
        array.swap(i, min_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut array = [3, 2, 1];
        selection(&mut array);
        assert_eq!(array, [1, 2, 3]);
    }
}
