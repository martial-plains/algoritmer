/// Shell sort implementation.
///
/// # Arguments
///
/// * `arr` - Some mutable ordered collection with heterogeneous comparable items inside
///
/// # Explanation
///
/// Shell sort is a variation of insertion sort that allows the use of a gap sequence.
///
/// The gap sequence is defined as follows:
///
/// # Performance
///
/// ## Time complexity
///
/// - Best-case performance: O(nlog n)
/// - Worst case performance: O(n²)
/// - Average performance: O(nlog n)
/// - Worst-case space complexity: O(1)
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Shellsort)
pub fn shell<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    let mut gap = arr.len() / 2;

    while gap > 0 {
        for i in gap..arr.len() {
            let cur = arr[i];
            let mut j = i;

            // Sort the sublist of for the current gap
            while j >= gap && arr[j - gap] > cur {
                arr.swap(j, j - gap);
                j -= gap;
            }
        }
        // Decrease the gap
        gap /= 2;
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
        shell(&mut arr);
        let actual = arr;
        assert_eq!(actual, expected);
    }
}
