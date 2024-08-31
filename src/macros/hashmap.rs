/// The `hashmap!` macro provides a convenient way to create a `HashMap` in Rust.
///
/// It allows you to initialize a `HashMap` with a list of key-value pairs,
/// similar to how you might initialize a dictionary in other programming languages.
#[macro_export]
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let capacity = hashmap!(@count $($key),*);
            let mut map = HashMap::with_capacity(capacity);
            $(
                let _ = map.insert($key, $value);
            )*
            map
        }
    };
}

#[cfg(test)]
mod tests {
    use hashbrown::HashMap;

    #[test]
    fn hashmap_contains_apple() {
        let map = hashmap! {
            "apple" => "red",
            "banana" => "yellow",
        };

        assert_eq!(map["apple"], "red");
    }
}
