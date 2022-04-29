/// Bitonic sort implementation.
///
/// Note that this program works only when size of input is a power of 2.
fn comp_and_swap<T>(arr: &mut [T], left: usize, right: usize, dir: i32)
where
    T: PartialOrd,
{
    if left >= right {
        return;
    }
    if dir == 1 {
        if arr[left] > arr[right] {
            arr.swap(left, right);
        }
    } else if arr[left] < arr[right] {
        arr.swap(left, right);
    }
}

/// It recursively sorts a bitonic sequence in ascending order, if direction = 1, and in
/// descending if direction = 0.
/// The sequence to be sorted starts at index position low, the parameter length is the
/// number of elements to be sorted.
fn merge<T>(arr: &mut [T], low: usize, length: usize, dir: i32)
where
    T: PartialOrd,
{
    if length > 1 {
        let middle = length / 2;
        for i in low..low + middle {
            comp_and_swap(arr, i, i + middle, dir);
        }

        merge(arr, low, middle, dir);
        merge(arr, low + middle, middle, dir);
    }
}

/// This function first produces a bitonic sequence by recursively
/// sorting its two
/// halves in opposite sorting orders, and then calls bitonic_merge to
/// make them in the
/// same order.
pub fn bitonic_sort<T>(arr: &mut [T], low: usize, length: usize, dir: i32)
where
    T: PartialOrd,
{
    if length > 1 {
        let middle = length / 2;
        bitonic_sort(arr, low, middle, 1);
        bitonic_sort(arr, low + middle, middle, 0);
        merge(arr, low, length, dir);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitonic_sort() {
        let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
        bitonic_sort(&mut arr, 0, 16, 1);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    }
}
