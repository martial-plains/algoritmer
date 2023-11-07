//! A module that contains a collection of higher-order functions

use alloc::vec::Vec;

/// A trait that provides a method for creating an iterator of partial reductions of a sequence.
///
/// A partial reduction is the result of applying a binary operation to the initial value and the first element,
/// then to the result and the second element, and so on.
///
///  # Examples
/// ```
/// use algorithms::higher_order_functions::Reductions;
///
/// let numbers = vec![1, 2, 3, 4];
/// let sum = |a, b| a + b;
/// let sums: Vec<_> = numbers.reductions(0, sum).collect();
/// assert_eq!(sums, vec![0, 1, 3, 6, 10]);
/// ```
pub trait Reductions {
    /// Returns an array containing the accumulated results of combining the
    /// elements of the sequence using the given closure.
    ///
    /// This can be seen as applying the reduce function to each element and
    /// providing the initial value followed by these results as a sequence.
    #[inline]
    fn reductions<B, F>(self, initial: B, mut transform: F) -> impl Iterator<Item = B>
    where
        Self: Sized + IntoIterator,
        F: FnMut(B, Self::Item) -> B,
        B: Clone,
    {
        let mut output = Vec::new();
        output.push(initial.clone());

        let mut initial = initial;

        for element in self {
            initial = transform(initial.clone(), element);
            output.push(initial.clone());
        }

        output.into_iter()
    }
}

impl<T: IntoIterator<Item = I>, I> Reductions for T {}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::Reductions;

    #[test]
    fn test_reductions() {
        let binding = [1, 2, 3, 4];
        let actual = binding
            .iter()
            .copied()
            .reductions(0, |initial, current| initial + current)
            .collect::<Vec<i32>>();
        let expected = [0, 1, 3, 6, 10];
        assert_eq!(actual, expected);
    }
}
