pub mod kmp;

/// A simple and inefficient way to see where one string occurs inside another 
/// is to check each place it could be, one by one, to see if it's there.
pub fn naive_pattern_search(s: String, pattern: &str) -> Result<Vec<i32>, &str> {
    Ok(s.char_indices()
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
        .collect::<Vec<i32>>())
}
