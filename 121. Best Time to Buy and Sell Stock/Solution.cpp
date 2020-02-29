#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
using namespace std;

class Solution {
public:
    int maxProfit_increasing_stack(vector<int>& prices) {
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

    // changes to max consecutive sum problem , the same
    int maxProfit_max_consecutive_sum(vector<int>& prices) {
        if (prices.size() <= 1) return 0;
        vector < int > r (prices.size());
        for (int i=1; i<prices.size(); i++) {
            r[i] = prices[i] - prices[i-1];
        }
        int max = 0, sum = 0;
        for (int i=0; i<prices.size(); i++) {
            sum += r[i];
            if (sum > max) max = sum;
            if (sum < 0 ) sum = 0;
        }
        return max;
    }

    // reasonalbe : easy to reasion about
    int maxProfit(vector<int>& prices) {
        if (prices.size() <= 1) return 0;
        int minprice = 1 << 30, maxprofit = 0;
        for (auto i : prices) {
            minprice = i < minprice ? i : minprice;
            maxprofit = i - minprice > maxprofit ? i - minprice : maxprofit;
        }
        return maxprofit;
    }
};

int main() {
    Solution s;
    vector<int> v {7,1,5,3,6,4};
    s.maxProfit(v);
    return 0;
}
