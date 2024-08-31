//! This module provides functions to compute the greatest common divisor (GCD) of a list of nonnegative integers.

/// Computes the greatest common divisor (GCD) of a list of nonnegative integers.
///
/// # Examples
///
/// ```
/// use algoritmer::math::gcd;
///
/// let nums: [u32;3] = [12, 18, 24];
/// let result = gcd(&nums);
/// assert_eq!(result, 6);
/// ```
///
/// # Explanation
///
/// In this example, we have a list of three nonnegative integers: 12, 18, and 24. We want to find
/// their greatest common divisor. The GCD represents the largest positive integer that divides
/// each of the given numbers without leaving a remainder.
///
/// The function `gcd` is called with the array `&nums`, which represents a slice of the original
/// array. The `gcd` function recursively computes the GCD of the first element `a` (12) and the
/// remaining elements (`&nums[1..]`, which is [18, 24]) by recursively calling itself.
///
/// Inside the function, if the length of the `nums` slice is 1, indicating that it contains only
/// one element, the function returns that element as the GCD. Otherwise, it assigns the first
/// element `a` to `nums[0]` and recursively computes the GCD of the remaining elements, stored in
/// `b`, by calling `gcd` again.
///
/// Finally, the function calls the helper function `gcd_cmp(a, b)` to compute the GCD of `a` and
/// `b`, which returns the GCD of the entire list of numbers.
///
/// The helper function `gcd_cmp(a, b)` is assumed to be defined elsewhere and compares the values
/// `a` and `b` to compute their GCD. It is not included in the code provided here.
pub fn gcd<T>(nums: &[T]) -> T
where
    T: num::Unsigned + Copy,
{
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = gcd(&nums[1..]);

    gcd_cmp(a, b)
}

fn gcd_cmp<T>(p: T, q: T) -> T
where
    T: num::Unsigned + Copy,
{
    if q.is_zero() {
        return p;
    }

    let r = p % q;
    gcd_cmp(q, r)
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(&[1usize, 2, 3, 4, 5]), 1);
        assert_eq!(gcd(&[2usize, 4, 6, 8, 10]), 2);
        assert_eq!(gcd(&[3usize, 6, 9, 12, 15]), 3);
        assert_eq!(gcd(&[10usize]), 10);
        assert_eq!(gcd(&[21usize, 110]), 1);
    }
}
