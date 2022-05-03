# Search Algorithms

## [Binary](crate::search::binary_search)

Excerpt From [Wikipedia][binary-wiki]: Binary search, also known as half-interval search or logarithmic search, is a search algorithm that finds the position of a target value within a sorted array. It compares the target value to the middle element of the array; if they are unequal, the half in which the target cannot lie is eliminated and the search continues on the remaining half until it is successful.

![alt text][binary-image]

- Performance
  - Time Complexity
    - Worst case performance O(log n)
    - Best case performance O(1)
    - Average case performance O(log n)
    - Worst case space complexity O(1)

## [Jump](./jump_search.rs)

Excerpt From [Wikipedia][jump-wiki]: In computer science, a jump search or block search refers to a search algorithm for ordered lists. It works by first checking all items L(km), where k ∈ N and m is the block size, until an item is found that is larger than the search key. To find the exact position of the search key in the list a linear search is performed on the sublist L[(k-1)m, km].

![alt text][jump-image]

- Performance
  - Time Complexity
    - Worst case performance O(√n)
    - Best case performance O(1)
    - Average case performance O(√n)
    - Worst case space complexity O(1)

## [Linear](./linear_search.rs)

Excerpt From [Wikipedia][linear-wiki]: linear search or sequential search is a method for finding a target value within a list. It sequentially checks each element of the list for the target value until a match is found or until all the elements have been searched.
  Linear search runs in at worst linear time and makes at most n comparisons, where n is the length of the list.

![alt text][linear-image]

- Performance
  - Time Complexity
    - Worst case performance O(n)
    - Best case performance O(1)
    - Average case performance O(n)
    - Worst case space complexity O(1)

## [Exponential](./exponential_search.rs)

Excerpt From [Wikipedia][exponential-wiki]: Exponential search allows for searching through a sorted, unbounded list for a specified input value (the search "key"). The algorithm consists of two stages. The first stage determines a range in which the search key would reside if it were in the list. In the second stage, a binary search is performed on this range. In the first stage, assuming that the list is sorted in ascending order, the algorithm looks for the first exponent, j, where the value 2^j is greater than the search key. This value, 2^j becomes the upper bound for the binary search with the previous power of 2, 2^(j - 1), being the lower bound for the binary search.

![alt text][exponential-image]

- Performance
  - Time Complexity
    - Worst case performance O(log n)
    - Best case performance O(1)
    - Average case performance O(log n)
    - Worst case space complexity O(1)

[binary-wiki]: https://en.wikipedia.org/wiki/Binary_search_algorithm
[binary-image]: https://upload.wikimedia.org/wikipedia/commons/f/f7/Binary_search_into_array.png

[jump-wiki]: https://en.wikipedia.org/wiki/Jump_search
[jump-image]: https://static.studytonight.com/data-structures/images/Jump%20Search%20technique.PNG

[linear-wiki]: https://en.wikipedia.org/wiki/Linear_search
[linear-image]: http://www.tutorialspoint.com/data_structures_algorithms/images/linear_search.gif

[exponential-wiki]: https://en.wikipedia.org/wiki/Exponential_search
[exponential-image]: https://upload.wikimedia.org/wikipedia/commons/4/45/Exponential_search.svg
