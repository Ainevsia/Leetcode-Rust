#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
using namespace std;

class Solution {
public:
    vector<int> findDisappearedNumbers(vector<int>& nums) {
        vector<int> res ;
        for (int i=0; i< nums.size(); ) {
            if (nums[i] != i + 1 and nums[nums[i] - 1] != nums[i]) {
                //swap
                int t = nums[i], p = nums[i] - 1;
                nums[i] = nums[nums[i] - 1];
                nums[p] = t;
            } else i ++ ;
        }
        for (int i=0; i<nums.size(); i++) {
            if (i + 1 != nums[i]) res.push_back(i + 1);
        }
        return res;
    }
};

int main() {
    Solution s;
    vector<int> v {4,3,2,7,8,2,3,1};
    auto x = s.findDisappearedNumbers(v);
    for (auto i: x) cout << i << endl;
}
