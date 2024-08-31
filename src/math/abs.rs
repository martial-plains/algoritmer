//! This module provides traits and implementations for finding the absolute maximum and minimum values in a collection.
//!
//! The `AbsMax` and `AbsMin` traits define methods for calculating the absolute maximum and minimum values, respectively.

use alloc::vec::Vec;
use num::Signed;

/// Trait for returning the absolute minimum and maximum of a collection of items.
pub trait AbsExtrema {
    /// The type of the return item.
    type Item;

    /// Return the absolute maximum value of the collection.
    fn abs_max(&self) -> Self::Item;

    /// Return the absolute minimum value of the collection.
    fn abs_min(&self) -> Self::Item;
}

impl<T> AbsExtrema for Vec<T>
where
    T: Signed + Copy + Ord,
{
    type Item = Option<T>;
    fn abs_min(&self) -> Self::Item {
        self.iter()
            .map(|&x| (x.abs(), x))
            .min_by_key(|&(a, _)| a)
            .map(|(_, x)| x)
    }

    fn abs_max(&self) -> Self::Item {
        self.iter()
            .map(|&x| (x.abs(), x))
            .max_by_key(|&(a, _)| a)
            .map(|(_, x)| x)
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;

    #[test]
    fn test_abs_min() {
        let v = vec![-1, -2, -3, -4, -5];
        assert_eq!(v.abs_min(), Some(-1));
    }

    #[test]
    fn test_abs_max() {
        let v = vec![-1, -2, -3, -4, -5];
        assert_eq!(v.abs_max(), Some(-5));
    }
}
