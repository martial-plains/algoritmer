/// Create a `HashMap` from a list of key-value pairs
///
/// ## Example
///
/// ```
/// use algorithms::macros::hashmap;
///
/// # fn main() {
///
/// let map = hashmap!{
///     "one" => 1,
///     "two" => 2,
/// };
/// assert_eq!(map["one"], 1);
/// assert_eq!(map["two"], 2);
/// assert_eq!(map.get("six"), None);
/// # }
/// ```
pub macro hashmap {
    (@single $($x:tt)*) => (()),
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*])),

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) },
    ($($key:expr => $value:expr),*) => {
        {
            #[cfg(not(feature = "std"))]
            use hashbrown::HashMap;
            #[cfg(feature = "std")]
            use std::collections::HashMap;

            let capacity = hashmap!(@count $($key),*);
            let mut map = HashMap::with_capacity(capacity);
            $(
                let _ = map.insert($key, $value);
            )*
            map
        }
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashmap_contains_apple() {
        let map = hashmap! {
            "apple" => "red",
            "banana" => "yellow",
        };

        assert_eq!(map["apple"], "red");
    }
}
