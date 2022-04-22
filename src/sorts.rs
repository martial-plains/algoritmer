/// Bead sort only works for sequences of non-negative integers.
/// <https://en.wikipedia.org/wiki/Bead_sort>
/// # Arguments
///
/// * `sequence` - The array being sorted
///
/// # Examples
///
/// ```
///
/// use algorithms::sorts::bead_sort;
///
/// let mut sequence = [9, 8, 7, 6, 5, 4, 3, 2, 1];
/// bead_sort(&mut sequence);
/// println!("{:?}", sequence); // [1, 2, 3, 4, 5, 6, 7, 8, 9]
///
/// ```
pub fn bead_sort<T>(sequence: &mut [T])
where
    T: Ord + Copy,
{
    for _ in 0..sequence.len() {
        for i in 0..sequence[1..].len() {
            let rod_upper = sequence[i];
            let rod_lower = sequence[1..][i];
            if rod_upper > rod_lower {
                sequence.swap(i, i + 1)
            }
        }
    }
}

/// A pure Rust implementation of the bogosort algorithm.
/// The function successively generates permutations of its input
/// until it finds one that is sorted
/// This is a highly inefficient sorting algorithm so it is not
/// useful for sorting but may be good for educational purposes
///
/// <https://en.wikipedia.org/wiki/Bogosort>
///
/// # Arguments
///
/// * `collection` - some mutable ordered collection with heterogeneous
/// comparable items inside
pub fn bogosort<T>(collection: &mut [T])
where
    T: PartialEq + PartialOrd,
{
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();

    fn is_sorted<T>(collection: &mut [T]) -> bool
    where
        T: PartialEq + PartialOrd,
    {
        if collection.len() < 2 {
            return true;
        }
        for i in 0..collection.len() - 1 {
            if collection[i] > collection[i + 1] {
                return false;
            }
        }
        true
    }

    while !is_sorted(collection) {
        collection.shuffle(&mut rng);
    }
}

/// A pure Rust implementation of the comb sort algorithm.
/// The function sorts its input in-place
///
/// # Arguments
///
/// * `collection` - some mutable ordered collection with heterogeneous
/// comparable items inside
pub fn bubble_sort<T>(collection: &mut [T])
where
    T: PartialEq + PartialOrd,
{
    let mut swapped = true;
    let mut i = 0;
    while swapped {
        swapped = false;
        for j in 0..collection.len() - i - 1 {
            if collection[j] > collection[j + 1] {
                collection.swap(j, j + 1);
                swapped = true;
            }
        }
        i += 1;
    }
}

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

/// Wiggle Sort.
///
/// Given an unsorted array `nums` reorder it such
/// that `nums[0]` < `nums[1]` > `nums[2]` < `nums[3]`
///
/// #Arguments
///
/// * `data` - The array being sorted
/// # Examples
///
/// ```
/// use algorithms::sorts::wiggle_sort;
///
/// let mut data = [3, 5, 2, 1, 6, 4];
/// wiggle_sort(&mut data);
///
/// println!("{:?}", data); // [3, 5, 1, 6, 2, 4]
///
/// ```
pub fn wiggle_sort<T>(nums: &mut [T])
where
    T: Ord,
{
    (0..nums.len()).for_each(|i| {
        if i % 2 == 1 {
            if nums[i - 1] > nums[i] {
                nums.swap(i, i - 1);
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bead_sort_test() {
        let mut vector = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        bead_sort(&mut vector);
        let actual = vector;
        assert_eq!(expected, actual);
    }

    #[test]
    fn bogosort_test() {
        let mut collection = vec![0, 5, 3, 2, 2];
        let expected = vec![0, 2, 2, 3, 5];
        bogosort(&mut collection);
        let actual = collection;
        assert_eq!(expected, actual);
    }
    #[test]
    fn comb_sort_test() {
        let mut vector = vec![99, 45, -7, 8, 2, 0, -15, 3];
        let expected = vec![-15, -7, 0, 2, 3, 8, 45, 99];
        comb_sort(&mut vector);
        let actual = vector;
        assert_eq!(expected, actual);
    }

    #[test]
    fn wiggle_sort_test() {
        let mut vector = vec![0, 5, 3, 2, 2];
        let expected = vec![0, 5, 2, 3, 2];
        wiggle_sort(&mut vector);
        let actual = vector;
        assert_eq!(expected, actual);
    }

    #[test]
    fn bubble_sort_test() {
        let mut vector = vec![0, 5, 3, 2, 2];
        let mut expected = vec![0, 2, 2, 3, 5];
        bubble_sort(&mut vector);
        let actual = vector;
        assert_eq!(expected, actual);
        vector = vec![-2, -45, -5];
        expected = vec![-45, -5, -2];
        bubble_sort(&mut vector);
        let actual = vector;
        assert_eq!(expected, actual);
        vector = vec![-23, 0, 6, -4, 34];
        expected = vec![-23, -4, 0, 6, 34];
        bubble_sort(&mut vector);
        let actual = vector;
        assert_eq!(expected, actual);
    }
}
