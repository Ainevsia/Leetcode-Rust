## Insigths

if v[i] < v[j], then for any pair (v[i], v[x], i < x < j) the water it can contain is <= the water the current pair can contain.

[Discuss](https://leetcode.com/problems/container-with-most-water/discuss/6099/yet-another-way-to-see-what-happens-in-the-on-algorithm)

```cpp
#include <algorithm>

class Solution {
public:
    int maxArea(vector<int>& v) {
        // v.size() is at least 2
        int i = 0, j = v.size() - 1;
        int max = 0;
        while (i < j) {
            int tmp = (j - i) * std::min(v[i], v[j]);
            max = tmp > max ? tmp : max;
            if ( v[i] < v[j] ) i++;
            else j--;
        }
        return max;
    }
};
```