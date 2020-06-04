#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
#include <unordered_map>
#include <map>
#include <cmath>
#include <queue>
using namespace std;

class Solution {
public:
    bool containsNearbyDuplicate(vector<int>& nums, int k) {
        unordered_map<int, vector<int>> map;
        for (int j = 0; j < nums.size(); j ++) {
            if (map.find(nums[j]) != map.end()) {
                int i = map[nums[j]].back();
                // cout << "find i = " << i << endl;
                // cout << "now  j = " << j << endl;
                
                if ( j - i <= k) {
                    return true;
                } else {
                    map[nums[j]].push_back(j);
                }
            } else {
                map[nums[j]] = {j};
                // cout << "inserting " << j << endl;
            }
        }
        return false;
    }
};

int main() {
    Solution a;
    vector<int> v = {1,2,3,1,2,3};
    int k = 2;
    auto res = a.containsNearbyDuplicate(v, k);
    cout << res;
    return 0;
}

