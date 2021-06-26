use algorithms::strings_algorithms::capitalize;

fn main() {
    let mut text = String::from("hello world!");
    let capitalized_text = capitalize(&mut text);
    println!("{}", capitalized_text);
}
