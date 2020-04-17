# Thoughts

Stack ? Cannot 

Substring Problem Pattern ? Not

Then what type of Problem is it?

## My first attempt

Treat `0` as `-1` and `1` as `+1`. Then the whole binary sequence can be transferred into a height mountain digram. The solution is the same as finding the maximum distance between two same height.

This can be Done in O(n^2).

But unfortunately, it will cause TLE.

## My second attempt

To get rid of the TLE, can we use DP?

This is a (2n+1) x 1 one dimensional DP.

The DP states are the two occurance of the current height.

## Insights

The possible `Heights` are in the range of `[-n, +n]`, total 2n + 1 cases.

So we can use a map to record the appearance of each height.