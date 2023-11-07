use core::hash::Hash;

#[cfg(not(target_feature = "std"))]
use hashbrown::HashMap;
#[cfg(target_feature = "std")]
use std::collections::HashMap;

/// Memoizes the result of a function to avoid redundant function calls.
///
/// # Arguments
///
/// * `cache` - A [`HashMap`] that will be used to store the results of the function.
/// * `func` - The function to memoize.
/// * `arg` - args to be memoized.
///
/// # Returns
///
/// Returns the result of the function.
///
/// # Example
///
/// ```
/// use hashbrown::HashMap;
/// use algorithms::dynamic_programming::memoize;
///
/// fn fibonacci(cache: &mut HashMap<u32, u64>, n: u32) -> u64 {
///     if n == 0 || n == 1 {
///         return n as u64;
///     }
///
///     memoize(cache, fibonacci, n - 1) + memoize(cache, fibonacci, n - 2)
/// }
///
/// fn main() {
///     let mut cache: HashMap<u32, u64> = HashMap::new();
///     let result = memoize(&mut cache, fibonacci, 10);
///     println!("Fibonacci(10) = {}", result);
/// }
/// ```
///
/// # How it works
///
/// The `memoize` function takes three arguments: `cache`, `func`, and `arg`. The purpose of this function is to cache
/// the results of the `func` function calls and avoid redundant computations for the same `arg`.
///
/// The `cache` argument is a mutable reference to a `HashMap` that will store the results. The `func` argument is the
/// function to memoize, which should accept the `cache` and `arg` as arguments and return the result. The `arg` argument
/// is the value to be memoized.
///
/// When `memoize` is called, it first checks if the result for the given `arg` already exists in the `cache` using
/// `cache.get(&arg).cloned()`. If the result exists in the cache, it is returned directly without invoking `func`. This
/// avoids redundant computations and improves performance.
///
/// If the result does not exist in the cache, `func` is called with `cache` and `arg` to compute the result. The result is
/// then stored in the `cache` using `cache.insert(arg, result.clone())` for future reference. Finally, the result is
/// returned.
///
/// By memoizing function results, especially in recursive functions or functions called frequently with the same
/// arguments, we can avoid redundant computations and improve performance.
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
    if let Some(result) = cache.get(&arg).cloned() {
        result
    } else {
        let result = func(cache, arg.clone());
        let _ = cache.insert(arg, result.clone());
        result
    }
}
