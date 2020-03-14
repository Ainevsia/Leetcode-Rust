#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
#include <unordered_map>
#include <cmath>
#include <queue>
using namespace std;

class Solution {
public:
    void nextPermutation(vector<int>& nums) {
        if (nums.size() <= 1) return;
        // traverse from back to front to find the first i that
        // nums[i] < nums[i+1]
        int i = nums.size() - 2;
        while (nums[i] >= nums[i+1]) {
            if (i == 0) {
                reverse(nums.begin(), nums.end());
                return;
            } else i -- ;
        }
        // traverse from back to front to find the first j that 
        // nums[j] > nums[i] , i < j < nums.size()
        int j = nums.size() - 1;
        while (nums[j] <= nums[i]) j -- ;

        swap(nums[i], nums[j]);

        // reverse the nums[i+1..nums.size()] so that it is the smallest
        reverse(nums.begin() + i + 1, nums.end());
    }
};

int main() {
    Solution a;
    string s = "hello world!";
    reverse(s.begin(), s.end());
    cout << s;
    return 0;
}

