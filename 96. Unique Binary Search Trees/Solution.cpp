#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
#include <unordered_map>
#include <cmath>
using namespace std;

class Solution {
public:
    // my version
    int numTrees(int n) {
        const int len = 1 << 10;
        vector<int> buf (len, 0);
        buf[1] = 1;
        for (int i=1; i<n; i++) {  // now at state i --> state i + 1
            vector<int> add (len, 0);
            for (int j = 1; j <= i; j ++)
                for (int k = 1; k <= j + 1; k ++)
                    if (k != j)
                        add[k] += buf[j];
            for (int j = 1; j <= i + 1; j++)
                buf[j] += add[j];
            // now (i + 1) th done
        }
        int res = 0;
        for (int i=1; i<=n; i++)
            res += buf[i];
        return res;
    }
};


int main() {
    Solution s;
    auto x = s.numTrees(8);
    cout << x;
    return 0;
}

