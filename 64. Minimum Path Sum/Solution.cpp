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
    int minPathSum(vector<vector<int>>& grid) {
        if (grid.size() == 0) return 0;
        if (grid[0].size() == 0) return 0;
        int m = grid.size(), n = grid[0].size();
        // deal with the first line
        for (int i = 1; i < m; i ++) {
            grid[0][i] += grid[0][i - 1];
        }
        for (int i = 1; i < m; i ++) {
            // deal with the first colomn
            grid[i][0] += grid[i - 1][0];
            for (int j = 1; j < n; j ++) {
                grid[i][j] = min(grid[i][j - 1], grid[i - 1][j]) + grid[i][j];
            }
        }
        return grid[m - 1][n - 1];
    }
};

int main() {
    Solution a;

    return 0;
}

