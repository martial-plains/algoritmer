pub fn abs_max(x: Vec<i32>) -> i32 {
    let mut j = x[0];
    for i in x {
        if i.abs() > j.abs() {
            j = i;
        };
    }
    j
}

pub fn abs_min(x: Vec<i32>) -> i32 {
    let mut j = x[0];
    for i in x {
        if i.abs() < j.abs() {
            j = i;
        };
    }
    j
}

pub fn calculate_factorial(num: i32) -> i32 {
    if num < 0 {
        panic!("No Factorial for negative numbers");
    } else {
        (1..=num).fold(1, |acc, n| acc * n)
    }
}

/// Return the ceiling of x as an Integral.
pub fn ceil(number: f32) -> i32 {
    if number - (number as i32) as f32 <= 0.0 {
        number as i32
    } else {
        number as i32 + 1
    }
}

pub mod fibonacci;

/// Return the floor of x as an Integral.
pub fn floor(number: f32) -> i32 {
    if (number as i32) as f32 - number == 0.0 {
        number as i32
    } else {
        ((number as i32) as f32 - number) as i32
    }
}

pub fn is_perfect(number: i32) -> bool {
    match number {
        x if x <= 0 => false,
        _ => (1..number).filter(|n| number % n == 0).sum::<i32>() == number,
    }
}

pub fn power(x: i32, power_of: i32) -> i32 {
    (1..=power_of).fold(1, |acc, _| -> i32 { acc * x })
}
