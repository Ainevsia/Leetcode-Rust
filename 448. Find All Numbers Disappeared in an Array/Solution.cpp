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
            cout << i;
            if (nums[i] != i + 1 and nums[nums[i] - 1] != nums[i]) {
                //swap
                int t = nums[i];
                nums[i] = nums[nums[i] - 1];
                nums[nums[i] - 1] = t;              // buggy code ! here nums[i] changed
                                                    // in the previous line !
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
    cout << "hello" << endl;
    auto x = s.findDisappearedNumbers(v);
    for (auto i: x) {
        cout << i << endl;
    }
}
