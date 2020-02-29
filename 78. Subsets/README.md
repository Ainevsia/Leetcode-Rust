# Permutations / Combinations / Subsets

Given their exponential solution space, it is tricky to ensure that the generated solutions are **complete** and **non-redundant**.

There are generally three strategies to do it:

- Recursion
- Backtracking
- Lexicographic generation based on the mapping between **binary bitmasks** and the corresponding
permutations / combinations / subsets. (this method has the best time complexity, and as a bonus, it generates lexicographically sorted output for the sorted inputs)

## Subset

![leetcode](https://leetcode.com/articles/Figures/78/recursion.png)

**Recursion**, if I already know the subsets of the previous N items, how to construct the subset by adding just the N+1 item?







