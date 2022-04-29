/// A Cocktail shaker sort implementation.
///
/// [Cocktail shaker sort](https://en.wikipedia.org/wiki/Cocktail_shaker_sort)
pub fn cocktail_shaker_sort<T>(arr: &mut [T])
where
    T: Ord + Copy,
{
    for i in 0..arr.len() - 1 {
        for j in 0..i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
        for j in (1..i).rev() {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cocktail_shaker_sort() {
        let mut arr = [1, 5, 2, 3, 4, 6, 7, 8, 9, 10];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
