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
    int numTimesAllBlue(vector<int>& light) {
        int l = light.size();
        vector <bool > dp (l+1, false), on (l+1, false);
        dp[0] = on[0] = true;
        int maxn = 0, res = 0;
        for (int k=0; k<l; k++) {
            int target = light[k];
            maxn = maxn < target ? target : maxn;
            on[target] = true;
            if (dp[target - 1]) {
                dp [target] = true;
                for (int i=target+1; i<=maxn; i ++) {
                    if (dp[i-1] and on[i]) dp[i] = true;
                }
                if (dp[maxn]) res++ ;
            }
        }
        return res;
    }
};

int main() {
    Solution s;
    vector<int> a {2,1,3,5,4};
    int x = s.numTimesAllBlue(a);
    cout << x;
    return 0;
}

