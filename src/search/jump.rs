use core::cmp::min;

use num::integer::sqrt;

/// A jump search implementation.
///
/// This algorithm works best when the array is sorted.
///
/// # Arguments
///
/// * `arr` - The array being searched
/// * `key` - The target value being searched for
///
/// # Returns
///
/// The index of the target value if found, or None if not found
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Jump_search)
pub fn jump<T>(arr: &[T], key: &T) -> Option<usize>
where
    T: PartialOrd + Copy,
{
    let n = arr.len();
    let mut step = sqrt(n);
    let mut prev = 0;

    while arr[min(step, n) - 1] < *key {
        prev = step;
        step += sqrt(n);

        if prev >= n {
            return None;
        }
    }

    while arr[prev] < *key {
        prev += 1;

        if prev == min(step, n) {
            return None;
        }
    }

    if arr[prev] == *key {
        Some(prev)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_search() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(jump(&arr, &1), Some(0));
        assert_eq!(jump(&arr, &10), Some(9));
        assert_eq!(jump(&arr, &11), None);
    }
}
