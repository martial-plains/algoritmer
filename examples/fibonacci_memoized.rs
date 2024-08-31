use algoritmer::math::memoized;

fn main() {
    let value = memoized(2);
    println!("{}", value);
}
