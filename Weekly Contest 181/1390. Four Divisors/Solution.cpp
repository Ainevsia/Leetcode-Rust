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
    int sumFourDivisors(vector<int>& nums) {
        int res = 0;
        for (int n : nums) {
            int last_d = 0;
            for (int d = 2; d*d <= n; d ++) {
                if (n % d == 0) {
                    if (last_d != 0) {
                        last_d = 0;
                        break;
                    } else {
                        last_d = d;
                    }
                }
            }
            if (last_d > 0 and last_d * last_d != n) 
                res += 1 + n + last_d + n / last_d;
        }
        return res;
    }
};

int main() {
    Solution a;
    vector <int > sv = {1,2,3,4,5,6,7,8,9,10};
    auto x = a.sumFourDivisors(sv);
    cout << x;
    return 0;
}

