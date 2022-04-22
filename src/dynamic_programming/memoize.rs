use core::hash::Hash;
use std::collections::HashMap;

pub fn memoize<A, R, F>(cache: &mut HashMap<A, R>, func: F, arg: A) -> R
where
    A: Hash + Eq + Clone,
    R: Clone,
    F: Fn(&mut HashMap<A, R>, A) -> R,
{
    match cache.get(&arg).map(|x| x.clone()) {
        Some(result) => result,
        None => {
            let result = func(cache, arg.clone());
            cache.insert(arg, result.clone());
            result
        }
    }
}
