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
    vector<int> productExceptSelf(vector<int>& nums) {
        int l = nums.size();
        vector<int> res (l, 1);
        int x = 1;
        for (int i = 1; i < l; i ++ )
            res[i] = x *= nums[i - 1];
        x = 1;
        for (int i = l - 2; i >= 0; i -- )
            res[i] *= x *= nums[i + 1];
        return res;
    }
};

int main() {
    Solution a;
    vector <int > v = {1,2,3,4};
    a.productExceptSelf(v);
    return 0;
}

