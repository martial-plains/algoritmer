/// Bubble sort is the simplest and slowest algorithm used for
/// sorting. It is based on various iterations, called **passes**.
///
/// As its worst-case performance is O(n²), it should be used for smaller datasets.
///
/// This function sorts its input in-place
///
/// # Arguments
///
/// * `arr` - some mutable ordered collection with heterogeneous comparable items inside
///
/// # Explanation
///
/// When the first pass is done, the largest element is in the
/// last position. When the second pass is done, the
/// second-largest element will be located at the second
/// highest position of the `collection` and so on. This is
/// done by comparing the adjacent elements, and swapping them
/// if they are in the wrong order.
///
/// # Performance
///
/// - Worst-case performance: O(n²)
/// - Best-case performance: O(n)
/// - Average performance: O(n²)
/// - Worst-case space complexity: O(1)
///     
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Bubble_sort)
pub fn bubble<T>(arr: &mut [T])
where
    T: PartialEq + PartialOrd,
{
    let mut swapped = true;
    let mut i = 0;

    // Perform passes pushing the largest element to the end of the collection
    // until no swaps are made
    //
    // Pass 1 is the first iteration of the outer loop
    while swapped {
        swapped = false;

        // This is where the list will be sorted until the highest element is pushed to the end
        // of the list. The first pass will have an N - 1 length, the second pass will have an N - 2, etc.
        // Each successive pass will have a length of N - 1 - i reducing the number of comparisons by 1
        for j in 0..arr.len() - i - 1 {
            // Compare adjacent neighbor elements.
            // If they are in wrong order, swap them.
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {

    use alloc::vec::Vec;
    use alloc::vec;

    use test_case::test_case;

    use super::*;

    #[test_case(vec![0, 5, 3, 2, 2],&[0, 2, 2, 3, 5])]
    #[test_case(vec![-2, -45, -5],&[-45, -5, -2])]
    #[test_case(vec![-23, 0, 6, -4, 34],&[-23, -4, 0, 6, 34])]
    #[test_case(vec![25, 21, 22, 24, 23, 27, 26],&[21, 22, 23, 24, 25, 26, 27])]
    fn bubble_sort_test(mut input: Vec<isize>, expected: &[isize]) {
        bubble(&mut input);
        let actual = input;
        assert_eq!(expected, actual);
    }
}
