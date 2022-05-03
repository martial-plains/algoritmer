/// Insertion sort is a sort that in each iteration, it
/// removes a data point from the data structure and then
/// inserts into it's right position.
///
/// | Time | Space |
/// |:----:|:-----:|
/// | O(n^2) | O(1) |
///
/// # Arguments
///
/// * `arr` - The array to be sorted.
///
/// # Examples
///
/// |        |        |        |        |        |        |        |
/// |--------|--------|--------|--------|--------|--------|--------|
/// | **25** |   26   |   22   |   24   |   27   |   23   |   21   |
/// | **25** | **26** |   22   |   24   |   27   |   23   |   21   |
/// | **22** | **25** | **26** |   24   |   27   |   23   |   21   |
/// | **22** | **24** | **25** | **26** |   27   |   23   |   21   |
/// | **22** | **24** | **25** | **26** | **27** |   23   |   21   |
/// | **22** | **23** | **24** | **25** | **26** | **27** |   21   |
/// |   21   |   22   |   23   |   24   |   25   |   26   |   27   |
///
/// ```
/// use algorithms::sorts::insertion_sort;
///
/// let mut list = vec![25, 26, 22, 24, 27, 23, 21];
/// insertion_sort(&mut list);
/// assert_eq!(list, vec![21, 22, 23, 24, 25, 26, 27]);
/// ```
pub fn insertion<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    for i in 1..arr.len() {
        let cur = arr[i];
        let mut j = i - 1;

        while arr[j] > cur {
            arr.swap(j + 1, j);
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
}

/// Insertion sort function that sorts until a given pivot.
///
/// # Arguments
///
/// * `arr` - The array to sort.
/// * `pivot` - The pivot to sort until.
pub fn insertion_until<T>(arr: &mut [T], pivot: usize)
where
    T: PartialOrd + Copy,
{
    for i in 1..pivot {
        let cur = arr[i];
        let mut j = i - 1;

        while arr[j] > cur {
            arr.swap(j + 1, j);
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use alloc::vec;
    use alloc::vec::Vec;

    use super::*;

    #[test_case(vec![25, 26, 22, 24, 27, 23, 21], vec![21, 22, 23, 24, 25, 26, 27 ] ; "insertion sort")]
    fn test_insertion_sort(mut actual: Vec<isize>, expected: Vec<isize>) {
        insertion(&mut actual);
        assert_eq!(actual, expected);
    }

    #[test_case(vec![25, 26, 22, 24, 27, 23, 21], vec![21, 22, 23, 24, 25, 26, 27 ] ; "insertion sort until")]
    fn test_insertion_sort_until(mut actual: Vec<isize>, expected: Vec<isize>) {
        let pivot = actual.len();
        insertion_until(&mut actual, pivot);
        assert_eq!(actual, expected);
    }
}
