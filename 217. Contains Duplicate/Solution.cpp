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
    bool containsDuplicate(vector<int>& nums) {
        unordered_map<int, int > map;
        for (int i : nums) {
            if (map.find(i) == map.end()) {
                map[i] = 1;
            } else {
                return true;
            }
        }
        return false;
    }
};

int main() {
    Solution a;

    return 0;
}

