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
    vector<int> createTargetArray(vector<int>& nums, vector<int>& index) {
        vector<int> res;
        int len = nums.size();
        for (int i=0; i<len; i++) {
            res.insert(res.begin()+index[i], nums[i]);
        }
        return res;
    }
};

int main() {
    Solution a;

    return 0;
}

