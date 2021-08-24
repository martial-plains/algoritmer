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

pub mod fibonacci {
    use std::collections::HashMap;

    use crate::dynamic_programming::memoize;

    pub fn recursive(nth: usize) -> u128 {
        if nth <= 1 {
            nth as u128
        } else {
            recursive(nth - 1) + recursive(nth - 2)
        }
    }

    pub fn imperative(nth: usize) -> u128 {
        let mut a = 0;
        let mut b = 1;
        let mut c = 0;
        for _ in 1..nth {
            c = a + b;
            a = b;
            b = c;
        }
        c
    }

    pub fn memoized(nth: usize) -> u128 {
        fn fib_memo(cache: &mut HashMap<usize, u128>, arg: usize) -> u128 {
            match arg {
                0 => 0,
                1 => 1,
                n => (memoize(cache, fib_memo, n - 1) + memoize(cache, fib_memo, arg - 2)).into(),
            }
        }
        memoize(&mut HashMap::new(), fib_memo, nth)
    }
}

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
