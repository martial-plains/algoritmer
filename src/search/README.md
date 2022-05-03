# Search Algorithms

## [Binary](crate::search::binary_search)

Excerpt From [Wikipedia][binary-wiki]: In computer science, binary search, also known as half-interval search, logarithmic search, or binary chop, is a search algorithm that finds the position of a target value within a sorted array. Binary search compares the target value to the middle element of the array. If they are not equal, the half in which the target cannot lie is eliminated and the search continues on the remaining half, again taking the middle element to compare to the target value, and repeating this until the target value is found. If the search ends with the remaining half being empty, the target is not in the array.

![alt text][binary-image]

- Performance
  - Time Complexity
    - Worst case performance O(log n)
    - Best case performance O(1)
    - Average case performance O(log n)
    - Worst case space complexity O(1)

## [Exponential](./exponential_search.rs)

Excerpt From [Wikipedia][exponential-wiki]: In computer science, an exponential search [also called doubling search or galloping search or Struzik search] is an algorithm, created by Jon Bentley and Andrew Chi-Chih Yao in 1976, for searching sorted, unbounded/infinite lists. There are numerous ways to implement this with the most common being to determine a range that the search key resides in and performing a binary search within that range. This takes O(log i) where i is the position of the search key in the list, if the search key is in the list, or the position where the search key should be, if the search key is not in the list.

![alt text][exponential-image]

- Performance
  - Time Complexity
    - Worst case performance O(log n)
    - Best case performance O(1)
    - Average case performance O(log n)
    - Worst case space complexity O(1)

## [Jump](./jump_search.rs)

Excerpt From [Wikipedia][jump-wiki]: In computer science, a jump search or block search refers to a search algorithm for ordered lists.

- Performance
  - Time Complexity
    - Worst case performance O(√n)
    - Best case performance O(1)
    - Average case performance O(√n)
    - Worst case space complexity O(1)

## [Linear](./linear_search.rs)

Excerpt From [Wikipedia][linear-wiki]: In computer science, a linear search or sequential search is a method for finding an element within a list. It sequentially checks each element of the list until a match is found or the whole list has been searched.

- Performance
  - Time Complexity
    - Worst case performance O(n)
    - Best case performance O(1)
    - Average case performance O(n)
    - Worst case space complexity O(1)

[binary-wiki]: https://en.wikipedia.org/wiki/Binary_search_algorithm
[binary-image]: https://upload.wikimedia.org/wikipedia/commons/thumb/8/83/Binary_Search_Depiction.svg/2880px-Binary_Search_Depiction.svg.png

[jump-wiki]: https://en.wikipedia.org/wiki/Jump_search

[linear-wiki]: https://en.wikipedia.org/wiki/Linear_search

[exponential-wiki]: https://en.wikipedia.org/wiki/Exponential_search
[exponential-image]: https://upload.wikimedia.org/wikipedia/commons/4/45/Exponential_search.svg
