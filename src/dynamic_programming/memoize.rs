use core::hash::Hash;
use hashbrown::HashMap;

/// Memoization is a method used to reduce function calls in recursive functions or other functions that are called very frequently
///
/// # Arguments
///
/// * `cache` - A HashMap that will be used to store the results of the function.
/// * `func` - The function to memoize.
/// * `arg` - args to be memoized.
///
/// # Returns
///
/// Returns the result of the function.
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Memoization)
pub fn memoize<A, R, F>(cache: &mut HashMap<A, R>, func: F, arg: A) -> R
where
    A: Hash + Eq + Clone,
    R: Clone,
    F: Fn(&mut HashMap<A, R>, A) -> R,
{
    match cache.get(&arg).cloned() {
        Some(result) => result,
        None => {
            let result = func(cache, arg.clone());
            let _ = cache.insert(arg, result.clone());
            result
        }
    }
}
