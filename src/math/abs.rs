//! This module provides traits and implementations for finding the absolute maximum and minimum values in a collection.
//!
//! The `AbsMax` and `AbsMin` traits define methods for calculating the absolute maximum and minimum values, respectively.

use alloc::vec::Vec;

/// Return the absolute maximum of a list of items.
pub trait AbsMax {
    /// The type of the return item.
    type Item;
    /// Return the minimum value of the collection.
    fn abs_max(&self) -> Self::Item;
}

/// Return the minimum value of the collection.
pub trait AbsMin {
    /// The type of the return item.
    type Item;
    /// Return the minimum value of the collection.
    fn abs_min(&self) -> Self::Item;
}

impl AbsMin for Vec<i8> {
    type Item = Option<i8>;
    fn abs_min(&self) -> Self::Item {
        self.iter()
            .map(|&x| (x.abs(), x))
            .min_by_key(|&(a, _)| a)
            .map(|(_, x)| x)
    }
}

impl AbsMin for Vec<i32> {
    type Item = Option<i32>;
    fn abs_min(&self) -> Self::Item {
        self.iter()
            .map(|&x| (x.abs(), x))
            .min_by_key(|&(a, _)| a)
            .map(|(_, x)| x)
    }
}

impl AbsMin for Vec<i64> {
    type Item = Option<i64>;
    fn abs_min(&self) -> Self::Item {
        self.iter()
            .map(|&x| (x.abs(), x))
            .min_by_key(|&(a, _)| a)
            .map(|(_, x)| x)
    }
}

impl AbsMax for Vec<i8> {
    type Item = Option<i8>;
    fn abs_max(&self) -> Self::Item {
        self.iter()
            .map(|&x| (x.abs(), x))
            .max_by_key(|&(a, _)| a)
            .map(|(_, x)| x)
    }
}

impl AbsMax for Vec<i32> {
    type Item = Option<i32>;
    fn abs_max(&self) -> Self::Item {
        self.iter()
            .map(|&x| (x.abs(), x))
            .max_by_key(|&(a, _)| a)
            .map(|(_, x)| x)
    }
}

impl AbsMax for Vec<i64> {
    type Item = Option<i64>;
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
