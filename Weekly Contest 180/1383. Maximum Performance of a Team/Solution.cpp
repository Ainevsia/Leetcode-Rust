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

#define r 1000000007

class Engineer {
public:

    int spe;
    int eff;

    Engineer(int s, int e):spe(s), eff(e){};
};

class Solution {
public:
    int maxPerformance(int n, vector<int>& speed, vector<int>& efficiency, int k) {
        priority_queue<int, vector<int>, greater<int>> sp;
        vector<Engineer> engs(n, {0,0});
        for (int i =0;i < n; i++) {
            engs[i].eff = efficiency[i];
            engs[i].spe = speed[i];
        }
        auto cmp = [&] (const Engineer & a, const Engineer & b) -> bool {
            return b.eff < a.eff;
        };
        sort(engs.begin(), engs.end(), cmp);
        long long total_speed = 0;
        long long perf = 0;
        for (int i=0;i<n;i++) {
            if (sp.size() == k) {
                total_speed -= sp.top();
                sp.pop();
            }
            sp.push(engs[i].spe);
            total_speed += engs[i].spe;
            perf = max(perf, total_speed * engs[i].eff);
        }
        return perf % r;
    }
};

int main() {
    Solution s;
    vector<int> speed = {2,10,3,1,5,8};
    vector<int> efficiency = {5,4,3,9,7,2};
    int k = 2, n = 6;;
    // 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 2
    s.maxPerformance(n, speed, efficiency, k );
    return 0;
}

