#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
#include <unordered_map>
using namespace std;

class Solution {
public:

    // just like 3 sum
    vector<vector<int>> threeSum(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        vector<vector<int>> res;
        if (nums.size() < 3) return res;
        for (int i=0; i<nums.size()-2; i++) {
            if (i>0 and nums[i] == nums[i-1]) continue;
            int l = i + 1, r = nums.size() - 1;
            // find all the twoSums and don't duplicate
            while (l < r) {
                if (nums[l] + nums[r] + nums[i] == 0) {
                    res.push_back({nums[i], nums[l], nums[r]});
                    // prevent dup
                    while (l + 1 < nums.size() and nums[l+1] == nums[l]) l ++ ;
                    while (r - 1 >= 0 and nums[r-1] == nums[r]) r -- ;
                    l ++ ; r -- ;
                } 
                else if (nums[l] + nums[r] + nums[i] < 0) l ++;
                else if (nums[l] + nums[r] + nums[i] > 0) r --;
            }
        }
        return res;
    }
};

int main() {
    Solution s;
    return 0;
}

