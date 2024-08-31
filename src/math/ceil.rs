//! This module provides a function to calculate the ceiling of a floating-point number.
//!
//! The `ceil` function returns the smallest integer greater than or equal to the given number.

/// Return the ceiling of x as an Integral.
///
/// The ceiling of a number is the smallest integer greater than or equal to the number.
///
/// # Arguments
///
/// * `number` - The number to calculate the ceiling of
///
/// # Examples
///
/// ```
/// use std::f32;
///
/// use algoritmer::math::ceil;
///
/// let result = ceil(3.7);
/// assert_eq!(result, 4);
///
/// let result = ceil(5.0);
/// assert_eq!(result, 5);
///
/// let result = ceil(-2.3);
/// assert_eq!(result, -2);
/// ```
///
/// # Explanation
///
/// The function `ceil` takes a single argument `number` of type `f32` and returns the ceiling of `number`
/// as an `i32`. It does so by checking if the decimal part of the number is greater than zero. If it is,
/// the function adds 1 to the integer part of the number to obtain the ceiling. If the decimal part is zero
/// or negative, the function simply returns the integer part of the number as the ceiling.
///
/// For example, when calling `ceil(3.7)`, the decimal part is 0.7 which is greater than zero. Therefore, the
/// function returns the integer part 3 plus 1, resulting in a ceiling of 4.
///
/// Similarly, when calling `ceil(-2.3)`, the decimal part is -0.3, which is not greater than zero. Hence, the
/// function returns the integer part -2 as the ceiling.
///
/// It is important to note that this implementation assumes that the input `number` is within the range of
/// representable `f32` values. If the input exceeds this range, the behavior of the function is undefined.
#[must_use]
pub fn ceil(number: f32) -> i32 {
    if number - (number as i32) as f32 <= 0.0 {
        number as i32
    } else {
        number as i32 + 1
    }
}
