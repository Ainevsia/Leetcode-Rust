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
    int maxCoins(vector<int>& nums) {
        vector<int> num (nums.size() + 2, 0);
        int n = 1;
        for (int i: nums) if (i > 0) num[n++] = i;
        num[0] = num[n++] = 1;

        vector<vector<int>> dp (nums.size() + 2, vector<int> (nums.size() + 2, 0));
        for (int k=2; k<n; k++) {
            // k is the range length
            for (int left = 0; left < n - k; ++ left) {
                int right = left + k;
                for (int i = left + 1; i < right; i++) {
                    dp[left][right] = max(dp[left][right],
                        num[left] * num[i] * num[right] + dp[left][i] + dp[i][right]);
                }
            }
        }
        return dp[0][n-1];
    }
};

int main() {
    Solution a;

    return 0;
}

