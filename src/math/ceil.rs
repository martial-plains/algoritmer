/// Return the ceiling of x as an Integral.
///
/// # Arguments
///
/// * `number` - The number to calculate the ceiling of
pub fn ceil(number: f32) -> i32 {
    if number - (number as i32) as f32 <= 0.0 {
        number as i32
    } else {
        number as i32 + 1
    }
}
