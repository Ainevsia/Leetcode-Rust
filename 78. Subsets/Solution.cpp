#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
using namespace std;

class Solution {
public:
    // bit mask
    vector<vector<int>> subsets(vector<int>& nums) {
        int n = nums.size(), p = 1 << n;
        vector<vector<int>> sub(p);
        for (int i=0; i<p; i++) {
            for (int j=0; j<n; j++) {
                if ((i>>j)&1 == 1) {
                    sub[i].push_back(nums[j]);
                }
            }
        }
        return sub;
    }
};

int main() {
    Solution a;
}
