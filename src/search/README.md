# Search Algorithms

## [Binary](crate::search::binary)

Excerpt From [Wikipedia][binary-wiki]: In computer science, binary search, also known as half-interval search, logarithmic search, or binary chop, is a search algorithm that finds the position of a target value within a sorted array. Binary search compares the target value to the middle element of the array. If they are not equal, the half in which the target cannot lie is eliminated and the search continues on the remaining half, again taking the middle element to compare to the target value, and repeating this until the target value is found. If the search ends with the remaining half being empty, the target is not in the array.

![alt text][binary-image]

- Performance
  - Time Complexity
    - Worst case performance O(log n)
    - Best case performance O(1)
    - Average case performance O(log n)
  - Space Complexity
    - Worst case complexity O(1)

## [Exponential](crate::search::exponential)

Excerpt From [Wikipedia][exponential-wiki]: In computer science, an exponential search [also called doubling search or galloping search or Struzik search] is an algorithm, created by Jon Bentley and Andrew Chi-Chih Yao in 1976, for searching sorted, unbounded/infinite lists. There are numerous ways to implement this with the most common being to determine a range that the search key resides in and performing a binary search within that range. This takes O(log i) where i is the position of the search key in the list, if the search key is in the list, or the position where the search key should be, if the search key is not in the list.

![alt text][exponential-image]

- Performance
  - Time Complexity
    - Worst case performance O(log n)
    - Best case performance O(1)
    - Average case performance O(log n)
  - Space Complexity
    - Worst case complexity O(1)

## [Fiboanacci](crate::search::fibonacci)

Excerpt From [Wikipedia][fibonacci-wiki]: In computer science, the Fibonacci search technique is a method of searching a sorted array using a divide and conquer algorithm that narrows down possible locations with the aid of Fibonacci numbers. Compared to binary search where the sorted array is divided into two equal-sized parts, one of which is examined further, Fibonacci search divides the array into two parts that have sizes that are consecutive Fibonacci numbers. On average, this leads to about 4% more comparisons to be executed, but it has the advantage that one only needs addition and subtraction to calculate the indices of the accessed array elements, while classical binary search needs bit-shift, division or multiplication, operations that were less common at the time Fibonacci search was first published. Fibonacci search has an average- and worst-case complexity of O(log n).

- Performance
  - Time Complexity
    - Worst case performance O(log n)
    - Best case performance O(1)
    - Average case performance O(log n)
  - Space Complexity
    - Worst case complexity O(1)

## [Jump](crate::search::jump)

Excerpt From [Wikipedia][jump-wiki]: In computer science, a jump search or block search refers to a search algorithm for ordered lists.

- Performance
  - Time Complexity
    - Worst case performance O(√n)
    - Best case performance O(1)
    - Average case performance O(√n)
  - Space Complexity
    - Worst case complexity O(1)

## [Linear](crate::search::linear)

Excerpt From [Wikipedia][linear-wiki]: In computer science, a linear search or sequential search is a method for finding an element within a list. It sequentially checks each element of the list until a match is found or the whole list has been searched.

- Performance
  - Time Complexity
    - Worst case performance O(n)
    - Best case performance O(1)
    - Average case performance O(n)
  - Space Complexity
    - Worst case complexity O(1)

## [Ternary](crate::search::ternary)

Excerpt From [Wikipedia][ternary-wiki]: A ternary search algorithm is a technique in computer science for finding the minimum or maximum of a unimodal function. A ternary search determines either that the minimum or maximum cannot be in the first third of the domain or that it cannot be in the last third of the domain, then repeats on the remaining two thirds. A ternary search is an example of a divide and conquer algorithm (see search algorithm).

- Performance
  - Time Complexity
    - Worst case performance O(log3 n)
    - Best case performance O(log3 n)
    - Average case performance O(log3 n)
  - Space Complexity
    - Worst case complexity O(1)

[binary-wiki]: https://en.wikipedia.org/wiki/Binary_search_algorithm
[binary-image]: https://upload.wikimedia.org/wikipedia/commons/thumb/8/83/Binary_Search_Depiction.svg/2880px-Binary_Search_Depiction.svg.png

[exponential-wiki]: https://en.wikipedia.org/wiki/Exponential_search
[exponential-image]: https://upload.wikimedia.org/wikipedia/commons/4/45/Exponential_search.svg

[fibonacci-wiki]: https://en.wikipedia.org/wiki/Fibonacci_search

[jump-wiki]: https://en.wikipedia.org/wiki/Jump_search

[linear-wiki]: https://en.wikipedia.org/wiki/Linear_search

[ternary-wiki]: https://en.wikipedia.org/wiki/Ternary_search
