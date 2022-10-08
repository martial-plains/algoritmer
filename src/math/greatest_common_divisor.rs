use core::marker::Copy;

/// Compute the greatest common divisor of a list of nonnegative integers
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

/// Compute the greatest common divisor of two nonnegative integers `p` and `q` as follows:
///
/// if `q` is 0, the answer is `p`. If not, divide `p` by `q` and take the remainder `r`. The answer is the greatest common divisor of `q` and `r`.
pub fn gcd_cmp<T>(p: T, q: T) -> T
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
    use crate::math::gcd;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(&[1usize, 2, 3, 4, 5]), 1);
        assert_eq!(gcd(&[2usize, 4, 6, 8, 10]), 2);
        assert_eq!(gcd(&[3usize, 6, 9, 12, 15]), 3);
        assert_eq!(gcd(&[10usize]), 10);
        assert_eq!(gcd(&[21usize, 110]), 1);
    }
}
