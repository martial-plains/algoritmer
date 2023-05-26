/// Return the floor of `number` as an Integral.
///
/// # Arguments
///
/// * `number` - The number to calculate the floor of
///
/// # Examples
///
/// ```
/// let result = floor(3.8);
/// assert_eq!(result, 3);
///
/// let result = floor(-2.5);
/// assert_eq!(result, -3);
/// ```
///
/// # Explanation
///
/// The `floor` function calculates the largest integer less than or equal to `number` and returns it as an `i32`.
///
/// The function first checks if the given `number` is already an integer by comparing it to the result of converting it to an `i32`
/// and then back to an `f32`. If the difference between these two values is zero, it means that `number` is already an integer,
/// and thus the function simply casts it to an `i32` and returns the result.
///
/// If the difference is non-zero, it implies that `number` has a decimal part. In this case, the function subtracts the decimal
/// part from `number` to obtain the integral part. The result is then cast to an `i32` and returned.
///
/// # Note
///
/// Be cautious when using this function with extremely large or small floating-point numbers, as the precision limitations of
/// floating-point arithmetic may lead to inaccuracies in the calculated floor value.
pub fn floor(number: f32) -> i32 {
    if (number as i32) as f32 - number == 0.0 {
        number as i32
    } else {
        ((number as i32) as f32 - number) as i32
    }
}
