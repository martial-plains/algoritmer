/// A Rust implementation of the [Heap Sort](https://en.wikipedia.org/wiki/Heapsort) algorithm.
///
/// # Arguments
///
/// * `array` - The array to sort.
/// * `order` - The order to sort the array in.
/// 
/// # References
/// 
/// * [Wikipedia](https://en.wikipedia.org/wiki/Heapsort)
pub fn heap<T, F>(array: &mut [T], order: F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = array.len();
    // Create heap
    for start in (0..len / 2).rev() {
        shift_down(array, &order, start, len - 1)
    }

    for end in (1..len).rev() {
        array.swap(0, end);
        shift_down(array, &order, 0, end - 1)
    }
}

fn shift_down<T, F>(array: &mut [T], order: &F, start: usize, end: usize)
where
    F: Fn(&T, &T) -> bool,
{
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        }
        if child < end && order(&array[child], &array[child + 1]) {
            child += 1;
        }
        if order(&array[root], &array[child]) {
            array.swap(root, child);
            root = child
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap() {
        let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        heap(&mut array, |a, b| a > b);
        assert_eq!(array, [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
}