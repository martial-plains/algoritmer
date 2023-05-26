/// A Rust implementation of the [stooge sort](https://en.wikipedia.org/wiki/Stooge_sort).
///
/// # Arguments
///
/// * `array` - The array to sort.
///
/// # References
///
/// * [Wikipedia](https://en.wikipedia.org/wiki/Stooge_sort)
pub fn stooge<T>(array: &mut [T])
where
    T: PartialOrd,
{
    let len = array.len();

    if array.first().unwrap() > array.last().unwrap() {
        array.swap(0, len - 1);
    }
    if len - 1 > 1 {
        let t = len / 3;
        stooge(&mut array[..len - 1]);
        stooge(&mut array[t..]);
        stooge(&mut array[..len - 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stooge() {
        let mut array = [5, 3, 2, 4, 1, 6, 7, 8, 9, 10];

        stooge(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
