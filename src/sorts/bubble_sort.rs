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

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;

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
