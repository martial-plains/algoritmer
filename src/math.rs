mod abs;
pub mod fibonacci;
pub use abs::*;

/// Calculates the factorial of the input
/// 
/// # Arguments
/// 
/// * `num` - The number to calculate the factorial of
pub fn calculate_factorial(num: i32) -> i32 {
    if num < 0 {
        panic!("No Factorial for negative numbers");
    } else {
        (1..=num).fold(1, |acc, n| acc * n)
    }
}

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

/// Returns true if a number is perfect.
/// 
/// # Arguments
/// 
/// * `number` - The number to check
pub fn is_perfect(number: i32) -> bool {
    match number {
        x if x <= 0 => false,
        _ => (1..number).filter(|n| number % n == 0).sum::<i32>() == number,
    }
}

/// Returns the power of a number.
/// 
/// # Arguments
/// 
/// * `base` - The base of the power
/// * `exponent` - The exponent of the power
pub fn power(base: i32, exponent: i32) -> i32 {
    (1..=exponent).fold(1, |acc, _| -> i32 { acc * base })
}
