/// Bead sort only works for sequences of non-negative integers.
/// # Arguments
///
/// * `sequence` - The array being sorted
///
/// # Examples
///
/// ```
///
/// use algorithms::sorts::bead;
///
/// let mut sequence = [9, 8, 7, 6, 5, 4, 3, 2, 1];
/// bead(&mut sequence);
/// println!("{:?}", sequence); // [1, 2, 3, 4, 5, 6, 7, 8, 9]
///
/// ```
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
