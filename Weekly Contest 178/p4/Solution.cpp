#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
using namespace std;

class Solution {
public:
    /* this is a graph dfs problem to search the min distance */
    int minCost(vector<vector<int>>& grid) {
        deque<tuple<int, int, int>> q { make_tuple(0,0,0) };    // search queue
        const int m = grid.size(), n = grid[0].size();          // m * n matrix
        vector<vector<int>> dp (m, vector<int>(n, 1 << 30));    // currnt min distance
        int dirs[4][2] { {0, 1}, {0, -1}, {-1, 0}, {1, 0} };    // four directions
        while (not q.empty()) { // always true
            auto node = q.front();
            q.pop_front();
            int i = get<0>(node), j = get<1>(node), cost = get<2>(node);
            if (i + 1 == m and j + 1 == n) return cost;
            dp[i][j] = cost;    // this should be the best cost at this time
            for (auto dir: dirs) {
                int ni = i + dir[0], nj = j + dir[1];
                if (ni < 0 or ni >= m or nj < 0 or nj >= n or dp[ni][nj] <= cost)
                    continue;
                if (grid[i][j] == 1 and dir[0] == 0 and dir[1] == 1 or
                    grid[i][j] == 2 and dir[0] == 0 and dir[1] ==-1 or
                    grid[i][j] == 3 and dir[0] == 1 and dir[1] == 0 or
                    grid[i][j] == 4 and dir[0] ==-1 and dir[1] == 0)
                    q.push_front(make_tuple(ni, nj, cost));
                else q.push_back(make_tuple(ni, nj, cost + 1));
            }
        }
        return 0;
    }
};

int main() {
    Solution s;
    return 0;
}
