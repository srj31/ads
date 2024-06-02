# Cardinality Estimation

## Problem

You have a very large unsorted dataset with many duplicates, how do you estimate the unique elements in the dataset?

## Setup

1. Generate `n` evenly distributed random numbers
2. Create duplicates of them at random
3. Shuffle arbitrarily

## Solution

1. Naive
   With the maximum value being `M` and finding the smallest value `m` we can estimate there to be roughly
   `M/m` unique values in the dataset.

   Simple solution though can be very inaccurate.

2. Probabilistic Counting
   From the [paper](https://www.cse.unsw.edu.au/~cs9314/07s1/lectures/Lin_CS9314_References/fm85.pdf) we can use the idea of
   having consecutive leading/trailing zeroes to estimate the number of unique elements in the dataset. The probability of having
   `k` zeroes would be `1/(2^(k+1))`. Performing multiple iterations of this can give us an estimate of the number of unique

## References

- [Morris Algorithm](https://gregorygundersen.com/blog/2019/11/11/morris-algorithm/)
- [Probabilistic Counting Algorithms for Data base algorithms](https://www.cse.unsw.edu.au/~cs9314/07s1/lectures/Lin_CS9314_References/fm85.pdf)
