use alloc::vec::Vec;

/// A Rust bucket sort implementation
/// Wikipedia: <https://en.wikipedia.org/wiki/Bucket_sort>
pub fn bucket<T>(arr: &mut [T])
where
    T: Ord + Clone,
{
    let mut bucket: Vec<Vec<T>> = alloc::vec![Vec::new(); arr.len()];
    for i in 0..arr.len() {
        bucket[i].push(arr[i].clone());
    }
    for _ in arr.iter() {
        bucket.sort();
    }
    let mut i = 0;
    for j in bucket {
        for k in j {
            arr[i] = k.clone();
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_sort() {
        let mut arr = [1, 5, 2, 3, 4, 6, 7, 8, 9, 10];
        bucket(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
