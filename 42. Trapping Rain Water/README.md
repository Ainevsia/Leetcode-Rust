## Insigths

think step by step: for a certain index i, how to find the left most wall and the right most wall.

This can lead to the thought of dp.



Two iteration DP:

```cpp
#include <algorithm>
#include <iostream>
using namespace std;
class Solution {
public:
    int trap(vector<int>& v) {
        int len = v.size();
        vector <int> lmax(len, 0), rmax(len, 0);
        int l = 0, r = 0;
        for (int i=0; i<len; i++) {
            l = v[i] > l ? v[i] : l;
            r = v[len-1-i] > r ? v[len-1-i] : r;
            lmax[i] = l; 
            rmax[len-1-i] = r;
        }
        int water=  0;
        for (int i=0; i<len; i++) {
            water += std::min(lmax[i], rmax[i]) - v[i];
        }
        return water; 
    }
};
```

One iteration using two pointers

```cpp
#include <algorithm>
#include <iostream>
using namespace std;
class Solution {
public:
    int trap(vector<int>& v) {
        if (v.size() == 0) return 0;
        int l = 0, r = v.size() - 1;
        int lm = v[l], rm = v[r];
        int water = 0;
        while (l <= r) {
            if (lm <= rm) {
                lm = lm > v[l] ? lm : v[l];
                water += lm - v[l]; 
                l++;
            } else {
                rm = rm > v[r] ? rm : v[r];
                water += rm - v[r];
                r--;
            }
        }
        return water;
    }
};
```