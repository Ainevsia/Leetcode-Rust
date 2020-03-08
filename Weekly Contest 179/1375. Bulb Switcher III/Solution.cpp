#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
#include <unordered_map>
#include <cmath>
using namespace std;

class Solution {
public:
    // do not try to simulate in algorithm contest
    // find the inner regulation and abstraction
    int numTimesAllBlue(vector<int>& light) {
        int res = 0, r = 0;
        for (int i=0; i<light.size(); i ++ )
            res += (r = max(light[i], r)) == i + 1;
        return res;
    }
};

int main() {
    Solution s;
    vector<int> a {2,1,3,5,4};
    int x = s.numTimesAllBlue(a);
    cout << x;
    return 0;
}

