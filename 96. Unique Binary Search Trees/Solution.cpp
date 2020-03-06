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
    // for each round i
    // `buf[x]` means **the number of trees** that has `x` right nodes
    // base on the `buf` vector of the `i - 1` round, we can calculate the `buf` of the `i` round:
    //      since the element now add is `cur_n + 1` which is the largest element so far,
    // 
    int numTrees(int n) {
        const int len = 1 << 10;
        vector<int> buf (len, 0);
        vector<int> add (len, 0);
        buf[1] = 1;
        for (int cur_n = 1; cur_n < n; cur_n ++) {   // now at state cur_n --> state cur_n + 1
            for (int p = 1; p <= cur_n; p ++)       // --> previus buf vector
                for (int c = 1; c <= p + 1; c ++)   // --> current add vector
                    if (c != p)
                        add[c] += buf[p];
            for (int _ = 1; _ <= cur_n + 1; _ ++) {
                buf[_] += add[_];
                add[_] = 0; // clear the previous `add` vector
            }
            // now (cur_n + 1) state done
        }
        int res = 0;
        for (int i=1; i<=n; i++)
            res += buf[i];
        return res;
    }

    // dp thought O(n) time O(n) space
    int numTrees_dp(int n) {
        vector<int> dp (n+1,0);
        dp[0] = dp[1] = 1;
        for (int i=2; i<=n; i ++ ) {
            for (int j=0; j<i; j ++ ) {
                dp[i] += dp[j] * dp[i - j - 1];
            }
        }
        return dp[n];
    }

    // cantalan tree
    // http://www-math.mit.edu/~rstan/ec/catalan.pdf
    int numTrees_cantalan(int n) {
        //  h(n)=h(n-1)*(4*n-2)/(n+1);
        // this formula is can be derived from dp version
        long long p = 1;
        for (int i=1; i<=n; i++)
            p = p * (4 * i - 2) / (i + 1);
        return p;
    }
};


int main() {
    Solution s;
    auto x = s.numTrees_cantalan(8);
    cout << x;
    return 0;
}

