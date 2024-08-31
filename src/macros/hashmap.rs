use hashbrown::HashMap;

pub macro hashmap {
    (@single $($x:tt)*) => (()),
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*])),

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) },
    ($($key:expr => $value:expr),*) => {
        {
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
