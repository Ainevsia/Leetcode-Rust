#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
using namespace std;

class Solution {
public:
    // greedy ok
    int maxProfit(vector<int>& prices) {
        if (prices.size() <= 1) return 0;
        int maxprofit = 0, minprice = prices[0];
        int sum = 0;
        for (int i=1; i<prices.size(); i++) {
            if (prices[i] < prices[i-1]) {
                sum += maxprofit;
                minprice = prices[i];
                maxprofit = 0;
                continue;
            }
            minprice = prices[i] < minprice ? prices[i] : minprice;
            maxprofit = prices[i] - minprice > maxprofit ? prices[i] - minprice : maxprofit;
        }
        return sum + maxprofit;
    }


    // extremely short and clean
    int maxProfit(vector<int>& prices) {
        int sum = 0;
        for (int i=1; i<prices.size(); i++) {
            int diff = prices[i] > prices[i-1];
            sum += diff > 0 ? diff : 0;
        }
        return sum;
    }
};

int main() {
    Solution s;

}
