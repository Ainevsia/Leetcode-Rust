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
    // O(n) using hashmap
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> map;
        for (int i=0; i<nums.size(); i++) {
            auto x = map.find(target - nums[i]);
            if (x != map.end()) return {x->second, i};
            map[nums[i]] = i;
        }
    }
};

int main() {
    Solution s;
    return 0;
}

