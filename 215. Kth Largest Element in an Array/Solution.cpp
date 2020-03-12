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
    /// l and r are both inclusive
    int quickSelect(vector<int> &nums, int l, int r, int k) {
        int pivot = l;
        for (int i = l; i < r; i ++)
            if (nums[i] <= nums[r])
                swap(nums[pivot ++], nums[i]);
        swap(nums[pivot], nums[r]);
        int cnt = r - pivot + 1; // numbers that are >= pivot
        if (cnt == k) return nums[pivot];
        else if (cnt < k) return quickSelect(nums, l, pivot - 1, k - cnt);
        else /* if (cnt > k) */ return quickSelect(nums, pivot + 1, r, k);
    }

    int findKthLargest(vector<int>& nums, int k) {
        return quickSelect(nums, 0, nums.size() - 1, k);
    }
};

int main() {
    Solution a;

    return 0;
}

