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

We can search the height array from both end, each end maintaining a `found zone`:

If first Left < Right , then Left keeps the max height found.

## Insights

