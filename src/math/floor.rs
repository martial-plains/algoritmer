/// Return the floor of `number` as an Integral.
///
/// # Arguments
///
/// * `number` - The number to calculate the floor of
pub fn floor(number: f32) -> i32 {
    if (number as i32) as f32 - number == 0.0 {
        number as i32
    } else {
        ((number as i32) as f32 - number) as i32
    }
}
