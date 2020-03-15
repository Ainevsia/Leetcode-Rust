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

class mpair {
public:
    vector<long long> sums;
    vector<long long> meff;
    long long perf;
};

#define r 1000000007
class Solution {
public:
    int maxPerformance(int n, vector<int>& speed, vector<int>& efficiency, int k) {
        vector<vector<mpair>> dp(k + 1, vector<mpair>(n+1));
        for (int i=0; i<dp.size();i++) {
            dp[i][0].meff.push_back(1<<30); dp[i][0].sums.push_back(0);dp[i][0].perf = 0;
        }
        for (int i=0; i<dp[0].size();i++) {
            dp[0][i].meff.push_back(1<<30); dp[0][i].sums.push_back(0);dp[0][i].perf = 0;
        }

        for (int i=1; i<dp.size();i++ ) {
            for (int j = 1;j<dp[0].size();j++) {
                long long addme = 0;
                for (int idx=0;idx<dp[i-1][j-1].meff.size();idx++) {
                    if (addme < (dp[i][j].sums[idx]+ speed[j-1]) * min(dp[i][j].meff[idx], (long long)efficiency[j-1])) {
                        addme = (dp[i][j].sums[idx]+ speed[j-1]) * min(dp[i][j].meff[idx], (long long)efficiency[j-1]);
                    }
                }
                long long sums = dp[i-1][j-1].sums;
                long long meff = dp[i-1][j-1].meff;
                long long addme = (sums + speed[j-1])
                                *min(meff,(long long)efficiency[j-1]);
                                // cout << addme << ' ';
                long long lesspeople = dp[i-1][j].sums * dp[i-1][j].meff;
                if (addme == dp[i][j-1].meff * dp[i][j-1].sums  or 
                addme == lesspeople or 
                dp[i][j-1].meff * dp[i][j-1].sums == lesspeople) cout << "h=hahah";
                if ( addme > dp[i][j-1].meff * dp[i][j-1].sums 
                    and addme > lesspeople) {
                    dp[i][j].sums = sums + speed[j-1];
                    dp[i][j].meff = min(meff,(long long)efficiency[j-1]);
                } else if (dp[i][j-1].meff * dp[i][j-1].sums > addme 
                    and dp[i][j-1].meff * dp[i][j-1].sums > lesspeople) {
                    dp[i][j].sums = dp[i][j-1].sums;
                    dp[i][j].meff = dp[i][j-1].meff;
                } else {
                    dp[i][j].sums = dp[i-1][j].sums;
                    dp[i][j].meff = dp[i-1][j].meff;
                }
            }
        }
        for (int i=0; i<dp.size();i++ ) {
            for (int j = 0;j<dp[0].size();j++) {
                cout << dp[i][j].sums << ' ';
            }
            cout << endl;
        }
        return (dp[k][n].meff*dp[k][n].sums) % r;
    }
};

int main() {
    Solution s;
    return 0;
}

