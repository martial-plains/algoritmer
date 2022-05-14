# Sort Algorithms

## [Bead](crate::sorts::bead)

Excerpt From [Wikipedia][bead-wiki]: **Bead sort**, also called **gravity sort**, is a natural sorting algorithm, developed by Joshua J. Arulanandham, Cristian S. Calude and Michael J. Dinneen in 2002, and published in The Bulletin of the European Association for Theoretical Computer Science. Both digital and analog hardware implementations of bead sort can achieve a sorting time of O(n); however, the implementation of this algorithm tends to be significantly slower in software and can only be used to sort lists of positive integers. Also, it would seem that even in the best case, the algorithm requires O(n²) space.

![alt text][bead-image]

[bead-wiki]: https://en.wikipedia.org/wiki/Bead_sort
[bead-image]: https://upload.wikimedia.org/wikipedia/commons/thumb/c/cf/BeadSort-Figure1.svg/1920px-BeadSort-Figure1.svg.png

## [Bitonic](crate::sorts::bitonic)

Excerpt From [Wikipedia][bitonic-wiki]: **Bitonic mergesort** is a parallel algorithm for sorting. It is also used as a construction method for building a sorting network. The algorithm was devised by Ken Batcher. The resulting sorting networks consist of O(n log² (n)) comparators and have a delay of O(log² ⁡(n)) where n is the number of items to be sorted.

![alt text][bitonic-image]

- Performance
  - Time Complexity
    - Worst case performance:
      - Parallel Time: O(log² (n))
    - Best case performance:
      - Parallel Time: O(log² (n))
    - Average case performance:
      - Parallel Time: O(log² (n))
  - Space Complexity
    - Worst case complexity:
      - Non-parallel Time: O(n log² (n))

[bitonic-wiki]: https://en.wikipedia.org/wiki/Bitonic_sorter
[bitonic-image]: https://upload.wikimedia.org/wikipedia/commons/thumb/9/98/Batcher_Bitonic_Mergesort_for_eight_inputs.svg/610px-Batcher_Bitonic_Mergesort_for_eight_inputs.svg.png

## [Bogo](crate::sorts::bogo)

Excerpt From [Wikipedia][bogo-wiki]: In computer science, **bogosort** (also known as **permutation sort**, **stupid sort**, or **slowsort**) is a sorting algorithm based on the generate and test paradigm. The function successively generates permutations of its input until it finds one that is sorted. It is not considered useful for sorting, but may be used for educational purposes, to contrast it with more efficient algorithms.

![alt text][bogo-image]

- Performance
  - Time Complexity
    - Worst case performance: O(1)
    - Best case performance: O(n)
    - Average case performance: O((n+1)!)
  - Space Complexity
    - Worst case complexity: O((n+1)!)

[bogo-wiki]: https://en.wikipedia.org/wiki/Bogo_sort
[bogo-image]: https://upload.wikimedia.org/wikipedia/commons/0/0e/ExperimentalBogosort.png

## [Bubble](crate::sorts::bubble)

Excerpt From [Wikipedia][bubble-wiki]: **Bubble sort**, sometimes referred to as **sinking sort**, is a simple sorting algorithm that repeatedly steps through the list, compares adjacent elements and swaps them if they are in the wrong order. The pass through the list is repeated until the list is sorted. The algorithm, which is a comparison sort, is named for the way smaller or larger elements "bubble" to the top of the list.

![alt text][bubble-image]

- Performance
  - Time Complexity
    - Worst case performance:
      - Comparisons: O(n²)
      - Swaps: O(n²)
    - Best case performance:
      - Comparisons: O(n)
      - Swaps: O(1)
    - Average case performance:
      - Comparisons: O(n²)
      - Swaps: O(n²)
  - Space Complexity
    - Worst case complexity:
      - Total: O(n)
      - Auxiliary: O(1)

[bubble-wiki]: https://en.wikipedia.org/wiki/Bubble_sort
[bubble-image]: https://upload.wikimedia.org/wikipedia/commons/c/c8/Bubble-sort-example-300px.gif

## [Bucket](crate::sorts::bucket)

Excerpt From [Wikipedia][bucket-wiki]: **Bucket sort**, or **bin sort**, is a sorting algorithm that works by distributing the elements of an array into a number of buckets. Each bucket is then sorted individually, either using a different sorting algorithm, or by recursively applying the bucket sorting algorithm. It is a distribution sort, a generalization of pigeonhole sort that allows multiple keys per bucket, and is a cousin of radix sort in the most-to-least significant digit flavor. Bucket sort can be implemented with comparisons and therefore can also be considered a comparison sort algorithm. The computational complexity depends on the algorithm used to sort each bucket, the number of buckets to use, and whether the input is uniformly distributed.

![alt text][bucket-image]

- Performance
  - Time Complexity
    - Worst case performance: 0(n²)
    - Average performance: ![alt text][<https://wikimedia.org/api/rest_v1/media/math/render/svg/b508fd973073e5379c61e04adfc2476a5f4803ad>], where k is the number of buckets.
    ![alt text][<https://wikimedia.org/api/rest_v1/media/math/render/svg/b043aed20b06a58bb04c557b0d9b6001d5f6aacc>]
  - Space Complexity
    - Worst case complexity: O(n + k)

[bucket-wiki]: https://en.wikipedia.org/wiki/Bucket_sort
[bucket-image]: https://upload.wikimedia.org/wikipedia/commons/thumb/e/e3/Bucket_sort_2.svg/1920px-Bucket_sort_2.svg.png

## [Cocktail Shaker](crate::sorts::cocktail_shaker)

Excerpt From [Wikipedia][cocktail-wiki]: **Cocktail shaker sort**, also known as **bidirectional bubble sort**, **cocktail sort**, **shaker sort** (which can also refer to a variant of selection sort), **ripple sort**, **shuffle sort**, or **shuttle sort**, is an extension of bubble sort. The algorithm extends bubble sort by operating in two directions. While it improves on bubble sort by more quickly moving items to the beginning of the list, it provides only marginal performance improvements.

![alt text][cocktail-image]

- Performance
  - Time Complexity
    - Worst case performance: O(n²)
    - Best case performance: O(n)
    - Average case performance: O(n²)
  - Space Complexity
    - Worst case complexity: O(1)

[cocktail-wiki]: https://en.wikipedia.org/wiki/Cocktail_shaker_sort
[cocktail-image]: https://upload.wikimedia.org/wikipedia/commons/e/ef/Sorting_shaker_sort_anim.gif

## [Comb](crate::sorts::comb)

Excerpt From [Wikipedia][comb-wiki]: **Comb sort** is a generalization of bubble sort that is both a comparison and exchange sort. Comb sort improves on bubble sort by making only one exchange for each pass through the list.

![alt text][comb-image]

- Performance
  - Time Complexity
    - Worst case performance: O(n²)
    - Best case performance: ![alt text][<https://wikimedia.org/api/rest_v1/media/math/render/svg/1b8781cea4259c3bd43204e02d08b9b9ce8fe0ff>]
    - Average case performance: ![alt text][<https://wikimedia.org/api/rest_v1/media/math/render/svg/e46c3bcb243a7c198840f5d4f79f09fa64fa3888>] , where p is the number of increments.
  - Space Complexity
    - Worst case complexity: O(1)

[comb-wiki]: https://en.wikipedia.org/wiki/Comb_sort
[comb-image]: https://upload.wikimedia.org/wikipedia/commons/4/46/Comb_sort_demo.gif

## [Counting](crate::sorts::counting)

Excerpt From [Wikipedia][counting-wiki]: In computer science, **counting sort** is an algorithm for sorting a collection of objects according to keys that are small positive integers; that is, it is an integer sorting algorithm. It operates by counting the number of objects that possess distinct key values, and applying prefix sum on those counts to determine the positions of each key value in the output sequence. Its running time is linear in the number of items and the difference between the maximum key value and the minimum key value, so it is only suitable for direct use in situations where the variation in keys is not significantly greater than the number of items. It is often used as a subroutine in radix sort, another sorting algorithm, which can handle larger keys more efficiently.

![alt text][counting-image]

- Performance
  - Time Complexity
    - Worst case performance: O(n + k), where k is the range of the non-negative key values.
  - Space Complexity
    - Worst case complexity: O(n + k)

[counting-wiki]: https://en.wikipedia.org/wiki/Counting_sort

## [Cycle](crate::sorts::cycle)

Excerpt From [Wikipedia][cycle-wiki]: **Cycle sort** is a comparison sort that operates by making multiple passes through the list. On each pass, the next item in the list is compared to the item previously stored in the gap. If the new item is larger than the previous one, it and the previous item are swapped. This process is repeated until the list is sorted.

![alt text][cycle-image]

- Performance
  - Time Complexity
    - Worst case performance: Θ(²)
    - Best case performance: Θ(n²)
    - Average case performance: Θ(n²)
  - Space Complexity
    - Worst case complexity: Θ(n) total, Θ(1) auxiliary

[cycle-wiki]: https://en.wikipedia.org/wiki/Cycle_sort
[cycle-image]: https://upload.wikimedia.org/wikipedia/commons/a/a7/Cyclesort.png

## [Gnome](crate::sorts::gnome)

Excerpt From [Wikipedia][gnome-wiki]: **Gnome sort** (nicknamed **stupid sort**) is a variation of the insertion sort sorting algorithm that does not use nested loops. Gnome sort was originally proposed by Iranian computer scientist Hamid Sarbazi-Azad (professor of Computer Science and Engineering at Sharif University of Technology) in 2000. The sort was first called stupid sort (not to be confused with bogosort), and then later described by Dick Grune and named gnome sort.

![alt text][gnome-image]

- Performance
  - Time Complexity
    - Worst case performance: Θ(n²)
    - Best case performance: Θ(n)
    - Average case performance: Θ(n²)
  - Space Complexity
    - Worst case complexity: Θ(1) auxiliary

[gnome-wiki]: https://en.wikipedia.org/wiki/Gnome_sort
[gnome-image]: https://upload.wikimedia.org/wikipedia/commons/8/89/Visualization_of_Gnome_sort.gif

## [Insertion](crate::sorts::insertion)

Excerpt From [Wikipedia][insertion-wiki]: **Insertion sort** is a simple sorting algorithm that builds the final sorted array (or list) one item at a time. It is much less efficient on large lists than more advanced algorithms such as quicksort, heapsort, or merge sort.

![alt text][insertion-image]

- Performance
  - Time Complexity
    - Worst case performance:
      - Comparisons: O(n²)
      - Swaps: O(n²)
    - Best case performance: Θ(n²)
      - Comparisons: O(n)
      - Swaps: O(1)
    - Average case performance: Θ(n²)
      - Comparisons: O(n²)
      - Swaps: O(n²)
  - Space Complexity
    - Worst case complexity: Θ(n) total, Θ(1) auxiliary

[insertion-wiki]: https://en.wikipedia.org/wiki/Insertion_sort
[insertion-image]: https://upload.wikimedia.org/wikipedia/commons/4/42/Insertion_sort.gif

## [Merge](crate::sorts::merge)

Excerpt From [Wikipedia][merge-wiki]: In computer science, **merge sort** (also commonly spelled as **mergesort**) is an efficient, general-purpose, and comparison-based sorting algorithm. Most implementations produce a stable sort, which means that the order of equal elements is the same in the input and output. Merge sort is a divide-and-conquer algorithm that was invented by John von Neumann in 1945. A detailed description and analysis of bottom-up merge sort appeared in a report by Goldstine and von Neumann as early as 1948.

![alt text][merge-image]

- Performance
  - Time Complexity
    - Worst case performance: O(n log n)
    - Best case performance: Ω(n log n) typically, Ω(n) natural variant
    - Average case performance: Θ(n log n)
  - Space Complexity
    - Worst case complexity: Θ(n) total with O(n) auxiliary, Θ(1) auxiliary with linked-list based implementation

[merge-wiki]: https://en.wikipedia.org/wiki/Merge_sort
[merge-image]: https://upload.wikimedia.org/wikipedia/commons/c/cc/Merge-sort-example-300px.gif

## [Quick](crate::sorts::quick)

Excerpt From [Wikipedia][quick-wiki]: **Quicksort** is an in-place sorting algorithm. Developed by British computer scientist Tony Hoare in 1959 and published in 1961, it is still a commonly used algorithm for sorting. When implemented well, it can be somewhat faster than merge sort and about two or three times faster than heapsort.

![alt text][quick-image]

- Performance
  - Time Complexity
    - Worst case performance: O(n²)
    - Best case performance: O(n log n) (simple partition) or O(n) (three-way partition and equal keys)
    - Average case performance: O(n log n)
  - Space Complexity
    - Worst case complexity: O(n) auxiliary (naive), O(log n) auxiliary (Hoare 1962)

[quick-wiki]: https://en.wikipedia.org/wiki/Quicksort
[quick-image]: https://upload.wikimedia.org/wikipedia/commons/6/6a/Sorting_quicksort_anim.gif

## [Selection](crate::sorts::selection)

Excerpt From [Wikipedia][selection-wiki]: In computer science, **selection sort** is an in-place comparison sorting algorithm. It has an O(n²) time complexity, which makes it inefficient on large lists, and generally performs worse than the similar insertion sort. Selection sort is noted for its simplicity and has performance advantages over more complicated algorithms in certain situations, particularly where auxiliary memory is limited.

![alt text][selection-image]

- Performance
  - Time Complexity
    - Worst case performance:
      - Comparisons: O(n²)
      - Swaps: O(n)
    - Best case performance:
      - Comparisons: O(n²)
      - Swaps: O(1)
    - Average case performance:
      - Comparisons: O(n²)
      - Swaps: O(n)
  - Space Complexity
    - Worst case complexity: Θ(1) auxiliary

[selection-wiki]: https://en.wikipedia.org/wiki/Selection_sort
[selection-image]: https://upload.wikimedia.org/wikipedia/commons/9/94/Selection-Sort-Animation.gif

## [Shell](crate::sorts::shell)

Excerpt From [Wikipedia][shell-wiki]: **Shellsort**, also known as **Shell sort** or **Shell's method**, is an in-place comparison sort. It can be seen as either a generalization of sorting by exchange (bubble sort) or sorting by insertion (insertion sort). The method starts by sorting pairs of elements far apart from each other, then progressively reducing the gap between elements to be compared. By starting with far apart elements, it can move some out-of-place elements into position faster than a simple nearest neighbor exchange. Donald Shell published the first version of this sort in 1959. The running time of Shellsort is heavily dependent on the gap sequence it uses. For many practical variants, determining their time complexity remains an open problem.

![alt text][shell-image]

- Performance
  - Time Complexity
    - Worst case performance: O(n²) (worst known worst case gap sequence)
    O(n log²n) (best known worst case gap sequence)
    - Best case performance: O(n log n) (most gap sequences)
      O(n log² n) (best known worst-case gap sequence)
    - Average case performance: depends on gap sequence
  - Space Complexity
    - Worst case complexity: Θ(n) total, Θ(1) auxiliary

[shell-wiki]: https://en.wikipedia.org/wiki/Shellsort
[shell-image]: https://upload.wikimedia.org/wikipedia/commons/d/d8/Sorting_shellsort_anim.gif

## [Wiggle](crate::sorts::wiggle)
