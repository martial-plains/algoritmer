/// A Gnome sort implementation.
///
/// # Arguments
///
/// * `arr` - The array to sort.
///
/// # Performance
///
/// ## Time Complexity
///
/// - Worst: O(n²)
/// - Best: O(n)
/// - Average: O(n²)
///
/// # References
///
/// [Wikipedia](https://en.wikipedia.org/wiki/Gnome_sort)
pub fn gnome<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    let mut i: usize = 1;
    let mut j: usize = 2;
    while i < len {
        if arr[i - 1] <= arr[i] {
            // for descending sort, use >= for comparison
            i = j;
            j += 1;
        } else {
            arr.swap(i - 1, i);
            i -= 1;
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn gnome_sort() {
        use super::gnome;
        let mut arr = [1, 3, 2, 4, 5, 6, 7, 8, 9, 10];
        gnome(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
