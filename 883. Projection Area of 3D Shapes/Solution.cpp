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
    // O(n) extra space can optimize?
    // can be optimized to O(1) space but iterate through the grid more than once
    int projectionArea(vector<vector<int>>& grid) {
        int n = grid.size();
        if (n < 1) return 0;
        vector<int> x(n, 0), y(n, 0);
        int sz = 0;
        for (int i=0; i<n; i++) {
            for (int j=0; j<n; j++) {
                int h = grid[i][j];
                if (h != 0) sz ++ ;
                if (h > x[i]) x[i] = h;
                if (h > y[j]) y[j] = h;
            }
        }
        int sx = 0, sy = 0;
        for (int i=0; i<n; i++) {
            sx += x[i];
            sy += y[i];
        }
        return sz + sx + sy;
    }
};

int main() {
    Solution a;

    return 0;
}

