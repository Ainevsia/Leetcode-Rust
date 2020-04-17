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
    int findMaxLength(vector<int>& nums) {
        if (nums.size() == 0) return 0;
        vector<int> height (nums.size() + 1, 0);
        int sum = 0;
        unordered_map<int, pair<int, int>> map;
        map[0].first = 0;
        map[0].second = 0;
        for (int i = 0; i < nums.size(); i ++ ) {
            sum += nums[i] == 1 ? 1 : -1;
            if (map.count(sum) == 0) {
                map[sum].first = i + 1;
                map[sum].second = 0;
            }
            else map[sum].second = i + 1;
        }
        int res = 0;
        for (auto &&[x, y] : map) {
            if (y.second != 0 and y.second - y.first > res)
                res = y.second - y.first; 
        }
        return res;
    }
};

int main() {
    Solution a;
    vector<int> v = {1, 0};
    int res = a.findMaxLength(v);
    cout << res;
    return 0;
}

