#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
using namespace std;

class Solution {
public:
    vector<int> smallerNumbersThanCurrent(vector<int>& nums) {
        vector <int> res (nums.size(), 0);
        for (int i=0; i< nums.size(); i++) {
            for (int j=0; j < nums.size(); j++) {
                if (nums[j] < nums[i]) {res[i] ++;}
            }
        }
        return res;
    }
};

int main() {
    Solution s;

}
