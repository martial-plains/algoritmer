/// A pure Rust implementation of the cycle sort algorithm.
///
/// # Arguments
///
/// * `arr` - The array to sort.
///
/// # Performance
///
/// ## Time Complexity
///
/// - Worst: Θ(n²)
///    - The worst case occurs when the array is in reverse order.
/// - Best: Θ(n²)
///   - The best case occurs when the array is already sorted.
/// - Average: Θ(n²)
///  - The average case occurs when the array is randomly ordered.
///
/// ## Space Complexity
///
/// - Worse: Θ(n) total, Θ(1) auxiliary
///
/// # References
///
/// * [Wikipedia](https://en.wikipedia.org/wiki/Cycle_sort)
pub fn cycle<T>(arr: &mut [T])
where
    T: Ord + Clone + Copy,
{
    let arr_len = arr.len();

    for cycle in 0..arr_len {
        let mut item = arr[cycle];
        let mut pos = cycle;

        for i in arr.iter().take(arr_len).skip(cycle + 1) {
            if *i < item {
                pos += 1;
            }
        }

        if pos == cycle {
            continue;
        }

        while item == arr[pos] {
            pos += 1;
        }

        (arr[pos], item) = (item, arr[pos]);

        while pos != cycle {
            pos = cycle;

            for i in arr.iter().take(arr_len).skip(cycle + 1) {
                if *i < item {
                    pos += 1;
                }
            }

            while item == arr[pos] {
                pos += 1;
            }

            (arr[pos], item) = (item, arr[pos])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle() {
        let mut unsorted = [7, 3, 2, 1, 5, 4, 6];
        let sorted = [1, 2, 3, 4, 5, 6, 7];
        cycle(&mut unsorted);
        assert!(unsorted == sorted);
    }
}
