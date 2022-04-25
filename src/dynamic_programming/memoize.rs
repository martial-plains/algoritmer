use core::hash::Hash;
use hashbrown::HashMap;

/// Memoization is a method used to reduce function calls in recursive functions or other functions that are called very frequently
///
/// [Wikipedia](https://en.wikipedia.org/wiki/Memoization)
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
