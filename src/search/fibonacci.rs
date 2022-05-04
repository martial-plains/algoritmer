use crate::math::fibonacci::iterative as fibs;
/// A Rust implementation of the [Fibonacci search algorithm](https://en.wikipedia.org/wiki/Fibonacci_search).
pub fn fibonacci<T>(arr: &[T], key: T) -> Option<usize>
where
    T: Ord,
{
    let length = arr.len();
    // Find m such that F_m >= n where F_i is the i_th fibonacci number.
    let mut i = 0;
    let mut next: usize;

    loop {
        if fibs(i) >= length {
            next = i;
            break;
        }
        i += 1;
    }

    let mut offset = 0;

    while next > 0 {
        let index_k = (offset + fibs(next - 1)).min(length - 1); // Prevent out of bounds

        match arr[index_k].cmp(&key) {
            core::cmp::Ordering::Equal => return Some(index_k),
            core::cmp::Ordering::Greater => next -= 1,
            core::cmp::Ordering::Less => {
                offset += fibs(next - 1);
                next -= 1;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(fibonacci(&arr, 1), Some(0));
        assert_eq!(fibonacci(&arr, 2), Some(1));
        assert_eq!(fibonacci(&arr, 3), Some(2));
        assert_eq!(fibonacci(&arr, 4), Some(3));
        assert_eq!(fibonacci(&arr, 5), Some(4));
        assert_eq!(fibonacci(&arr, 6), Some(5));
        assert_eq!(fibonacci(&arr, 7), Some(6));
        assert_eq!(fibonacci(&arr, 8), Some(7));
        assert_eq!(fibonacci(&arr, 9), Some(8));
        assert_eq!(fibonacci(&arr, 10), Some(9));
        assert_eq!(fibonacci(&arr, 11), None);
        assert_eq!(fibonacci(&arr, 0), None);
    }
}
