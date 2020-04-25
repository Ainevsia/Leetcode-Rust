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

/// Nobody can code out his code without consistent learning
class Solution {
public:
    int m;
    int n;
    int cnt;
    vector<vector<char>> g;

    void traverse(int i, int j) {
        g[i][j] = '0';
        if (i > 0 and g[i - 1][j] == '1')
            traverse(i - 1, j);
        if (i < m - 1 and g[i + 1][j] == '1')
            traverse(i + 1, j);
        if (j > 0 and g[i][j - 1] == '1')
            traverse(i, j - 1);
        if (j < n - 1 and g[i][j + 1] == '1')
            traverse(i, j + 1);
    }

    int numIslands(vector<vector<char>>& grid) {
        m = grid.size();
        if (m == 0) return 0;
        n = grid[0].size();
        if (n == 0) return 0;
        cnt = 0;
        g.clear();
        for (auto & i : grid)
            g.push_back(i);
        for (int i = 0; i < m; i ++ ) {
            for (int j = 0; j < n; j ++ ) {
                if (g[i][j] == '1') {
                    cnt ++ ;
                    traverse(i, j);
                }
            }
        }
        return cnt;
    }
};

int main() {
    Solution a;
    return 0;
}

