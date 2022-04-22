/// A pure Rust implementation of the bogosort algorithm.
/// The function successively generates permutations of its input
/// until it finds one that is sorted
/// This is a highly inefficient sorting algorithm so it is not
/// useful for sorting but may be good for educational purposes
///
/// <https://en.wikipedia.org/wiki/Bogosort>
///
/// # Arguments
///
/// * `collection` - some mutable ordered collection with heterogeneous
/// comparable items inside
pub fn bogosort<T>(collection: &mut [T])
where
    T: PartialEq + PartialOrd,
{
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();

    fn is_sorted<T>(collection: &mut [T]) -> bool
    where
        T: PartialEq + PartialOrd,
    {
        if collection.len() < 2 {
            return true;
        }
        for i in 0..collection.len() - 1 {
            if collection[i] > collection[i + 1] {
                return false;
            }
        }
        true
    }

    while !is_sorted(collection) {
        collection.shuffle(&mut rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bogosort_test() {
        let mut vector = vec![0, 5, 3, 2, 2];
        let expected = vec![0, 2, 2, 3, 5];
        bogosort(&mut vector);
        let actual = vector;
        assert_eq!(expected, actual);
    }
}