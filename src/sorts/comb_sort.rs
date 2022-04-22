/// Comb sort algorithm is a simple sorting algorithm. It improves
/// on the bubble sort in the same way that Shellsort improves on
/// insertion sort
///
/// # Arguments
///
/// * `data` - The array being sorted
///
/// # Examples
///
/// ```
///
/// use algorithms::sorts::comb_sort;
///
/// let mut data = [0, 5, 3, 2, 2];
/// comb_sort(&mut data);
/// println!("{:?}", data); // [0, 2, 2, 3, 5]
///
/// ```
pub fn comb_sort<T>(data: &mut [T])
where
    T: Ord,
{
    // The shrink factor has a great effect on the efficiency of
    // the comb sort. k = 1.3 has been suggested as an ideal shrink
    // factors by the authors of the original article. A value
    // too small slows the algorithm down by making unnecessarily
    // many comparisons, whereas a value too large fails to
    // effectively.

    // Set the gap shrink factor
    let shrink_factor = 1.3; // k: [ n/k, n/k2, n/k3, ..., 1 ].

    // The gap starts out as the length of the list being sorted

    // Initialize gap size
    let mut gap = data.len(); // distance from each other

    let mut is_sorted = false;

    while !is_sorted {
        // The gap size is then divided by the shrink factor k

        // Update the gap value for a next comb
        gap = (gap as f64 / shrink_factor) as usize;
        if gap <= 1 {
            is_sorted = true; // If there are no swaps this pass, we are done
        }

        // A single "comb" over the input list
        let mut index = 0;

        while index + gap < data.len() {
            if data[index] > data[index + gap] {
                // Swap values
                data.swap(index, index + gap);
                is_sorted = false;
                // If this assignment never happens within the loop,
                // then there have been no swaps and the list is sorted.
            }
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comb_sort_test() {
        let mut vector = vec![99, 45, -7, 8, 2, 0, -15, 3];
        let expected = vec![-15, -7, 0, 2, 3, 8, 45, 99];
        comb_sort(&mut vector);
        let actual = vector;
        assert_eq!(expected, actual);
    }
}
