/// Wiggle Sort.
///
/// Given an unsorted array `nums` reorder it such
/// that `nums[0]` < `nums[1]` > `nums[2]` < `nums[3]`
///
/// #Arguments
///
/// * `arr` - The array being sorted
/// # Examples
///
/// ```
/// use algoritmer::sorts::wiggle;
///
/// let mut data = [3, 5, 2, 1, 6, 4];
/// wiggle(&mut data);
///
/// println!("{:?}", data); // [3, 5, 1, 6, 2, 4]
///
/// ```
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Wiggle_sort)
pub fn wiggle<T>(arr: &mut [T])
where
    T: Ord,
{
    (0..arr.len()).for_each(|i| {
        if i % 2 == 1 && arr[i - 1] > arr[i] {
            arr.swap(i, i - 1);
        }
    });
}

#[cfg(test)]
mod tests {

    use alloc::vec;

    #[test]
    fn wiggle_sort_test() {
        let mut vector = vec![0, 5, 3, 2, 2];
        let expected = vec![0, 5, 2, 3, 2];
        super::wiggle(&mut vector);
        let actual = vector;
        assert_eq!(expected, actual);
    }
}
