use algorithms::math_algorithms::fibonacci::memoized;

fn main() {
    let value = memoized(2);
    println!("{}", value);
}
