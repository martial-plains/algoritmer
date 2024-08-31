use algoritmer::strings::capitalize;

fn main() {
    let text = String::from("hello world!");
    let capitalized_text = capitalize(&text);
    println!("{}", capitalized_text);
}
