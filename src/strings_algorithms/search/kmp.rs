/// Searches for occurrences of a "word" w within a main "text string"
/// s
///
/// Returns an array of integers (positions in `s` at which `w` is
/// found) and an integer (number of positions)
///
/// # Wikipedia
/// https://en.wikipedia.org/wiki/Knuth–Morris–Pratt_algorithm
///
/// # Arguments
///
/// * `s` - The text to be searched
/// * `w` - The word sought
pub fn kmp_search(s: &str, w: &str) -> (Vec<usize>, i32) {
    let mut j = 0; // The position of the current character in `s`
    let mut k = 0; // The position of the current character in `w`
    let table = kmp_table(w);

    let mut n_p = 0;
    let mut p: Vec<usize> = Vec::new();

    while j < s.len() {
        if w.chars().nth(k).unwrap() == s.chars().nth(j).unwrap() {
            j += 1;
            k += 1;
            if k == w.len() {
                // occurrence found if only first occurrence is needed, m <- j - k may be returned here
                p.push(j - k);
                n_p += 1;
                k = table[k] as usize; // table[k] can't be -1
            } else {
                k = table[k] as usize;
            }
        }
    }
    (p, n_p as i32)
}

/// Returns an array of integers (the table to be filled)
///
/// # Arguments
///
/// * `w` - The word to be analyzed
pub fn kmp_table(w: &str) -> Vec<i32> {
    let mut t: Vec<i32> = Vec::default();
    let mut pos = 1; // The current position we are computing in `t`
    let mut cnd = 0; // The zero-based index in `w` of the next character of the current candidate substring

    t.push(-1);

    while pos < w.len() {
        if w.chars().nth(pos).unwrap() == w.chars().nth(cnd).unwrap() {
            t.push(t[cnd]);
        } else {
            t.push(cnd as i32);
            while w.chars().nth(pos).unwrap() != w.chars().nth(cnd).unwrap() {
                cnd = t[cnd] as usize
            }
        }
        pos += 1;
        cnd += 1;
    }
    t.push(cnd as i32); // only needed when all word occurrences are searched)
    t
}
