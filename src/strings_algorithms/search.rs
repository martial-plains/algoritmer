pub mod kmp;

pub fn naive_pattern_search(s: String, pattern: &str) -> Vec<i32> {
    s.char_indices()
        .map(|(i, x)| {
            let myv = pattern.chars().nth(0).unwrap();

            if x == myv {
                (i as i32, &s[i..(i + (pattern.len() - 1))])
            } else {
                (i as i32, "")
            }
        })
        .filter(|(_i, x)| pattern == *x)
        .map(|(i, _x)| i)
        .collect()
}
