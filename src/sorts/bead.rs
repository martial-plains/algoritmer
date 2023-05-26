/// Bead sort is a sorting algorithm that works for sequences of non-negative integers.
///
/// # Arguments
///
/// * `sequence` - The array being sorted.
///
/// # Examples
///
/// ```
/// use algorithms::sorts::bead;
///
/// let mut sequence = [9, 8, 7, 6, 5, 4, 3, 2, 1];
/// bead(&mut sequence);
/// assert_eq!(sequence, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
///
/// # Explanation
///
/// Bead sort operates by simulating the process of gravity pulling beads arranged on rods. Each value in the sequence represents the number of beads on a rod, and the rods are arranged vertically. The algorithm proceeds as follows:
///
/// 1. For each position in the sequence, starting from the first position:
///   - Compare the value at the current position (`rod_upper`) with the value at the next position (`rod_lower`).
///   - If `rod_upper` is greater than `rod_lower`, swap the values.
/// 2. Repeat step 1 for each position in the sequence, iterating from the first position to the last.
/// 3. After iterating through all positions in the sequence, the beads will have settled in sorted order due to the gravity simulation.
///
/// In the provided example, the initial sequence is `[9, 8, 7, 6, 5, 4, 3, 2, 1]`.
///
/// 1. In the first pass, the algorithm compares 9 and 8. Since 9 is greater than 8, the values are swapped.
///    - The sequence becomes `[8, 9, 7, 6, 5, 4, 3, 2, 1]`.
/// 2. In the second pass, the algorithm compares 9 and 7. The values remain in the same order because 9 is not greater than 7.
/// 3. The process continues until the end of the sequence, performing swaps where necessary.
/// 4. After completing all passes, the beads have settled, resulting in the sorted sequence: `[1, 2, 3, 4, 5, 6, 7, 8, 9]`.
///
/// Therefore, the original sequence `[9, 8, 7, 6, 5, 4, 3, 2, 1]` is sorted using the bead sort algorithm into `[1, 2, 3, 4, 5, 6, 7, 8, 9]`.
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Bead_sort)
pub fn bead<T>(sequence: &mut [T])
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

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;

    #[test]
    fn bead_sort_test() {
        let mut vector = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        bead(&mut vector);
        let actual = vector;
        assert_eq!(expected, actual);
    }
}
