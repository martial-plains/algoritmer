use alloc::vec::Vec;

/// Illustrate how to implement bucket sort algorithm.
///
/// Author: OMKAR PATHAK
/// This program will illustrate how to implement bucket sort algorithm
///
/// Wikipedia says: Bucket sort, or bin sort, is a sorting algorithm that works
/// by distributing the elements of an array into a number of buckets.
/// Each bucket is then sorted individually, either using a different sorting
/// algorithm, or by recursively applying the bucket sorting algorithm. It is a
/// distribution sort, and is a cousin of radix sort in the most to least
/// significant digit flavour.
/// Bucket sort is a generalization of pigeonhole sort. Bucket sort can be
/// implemented with comparisons and therefore can also be considered a
/// comparison sort algorithm. The computational complexity estimates involve the
/// number of buckets.
///
/// Time Complexity of Solution:
/// Worst case scenario occurs when all the elements are placed in a single bucket.
/// The overall performance would then be dominated by the algorithm used to sort each
/// bucket. In this case, O(n log n), because of TimSort
///
/// Average Case O(n + (n^2)/k + k), where k is the number of buckets
///
/// If k = O(n), time complexity is O(n)
///
/// Source: https://en.wikipedia.org/wiki/Bucket_sort
pub fn bucket_sort<T>(arr: &mut [T])
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
        bucket_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
