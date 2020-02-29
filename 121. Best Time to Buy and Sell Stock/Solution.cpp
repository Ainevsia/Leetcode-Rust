#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
using namespace std;

class Solution {
public:
    int maxProfit(vector<int>& prices) {
        if (prices.size() <= 1) return 0;
        vector<int > stk {prices[0]};
        // stk is increasing, not only non-decreasing
        int res = 0;
        for (int i=1; i<prices.size(); ) {
            if (prices[i] <= stk.back()) {
                if (stk.size() == 1) stk.back() = prices[i++];
                else {
                    int diff = stk.back() - stk.front();
                    res = diff > res ? diff : res;
                    stk.pop_back();
                }
            } else {
                stk.push_back(prices[i++]);
            }
        }
        if (stk.size() >= 2) {
            int diff = stk.back() - stk.front();
            res = diff > res ? diff : res;
        }
        return res;
    }
};

int main() {
    Solution s;

}
