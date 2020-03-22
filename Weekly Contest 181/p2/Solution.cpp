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
enum From {up, down, _left, _right} ;

class Solution {
public:
    int m;
    int n;
    
    bool go(vector<vector<int>>& grid, int i, int j, From from) {
        if (i < 0 or i >= m or j < 0 or j >= n) return false;
        if (i == 0 and j == 0) // loop
            return false;
        bool isfinal =( i + 1 == m and j + 1 == n );
        switch (grid[i][j]) {
            case 1:
                if (from == _left) return go (grid, i, j+1, _left) or isfinal;
                if (from == _right) return go(grid, i, j-1, _right) or isfinal;
                break;
            case 2:
                if (from == up) return go (grid, i+1, j, up) or isfinal;
                if (from == down) return go(grid, i-1, j, down) or isfinal;
                break;
            case 3:
                if (from == _left) return go (grid, i+1, j, up) or isfinal;
                if (from == down) return go(grid, i, j-1, _right) or isfinal;
                break;
            case 4:
                if (from == _right) return go (grid, i+1, j, up) or isfinal;
                if (from == down) return go(grid, i, j+1, _left) or isfinal;
                break;
            case 5:
                if (from == _left) return go (grid, i-1, j, down) or isfinal;
                if (from == up) return go(grid, i, j-1, _right) or isfinal;
                break;
            case 6:
                if (from == up) return go (grid, i, j+1, _left) or isfinal;
                if (from == _right) return go(grid, i-1, j, down) or isfinal;
                break;
        }
        return false;
    }
    
    bool hasValidPath(vector<vector<int>>& grid) {
        m = grid.size();
        n = grid[0].size();
        if (m == 1 and n == 1) return true;
        switch (grid[0][0]) {
            case 2:
            case 3:
                return go(grid, 1, 0, up);
            case 5:
                return false;
            case 1:
            case 6:
                return go(grid, 0, 1, _left);
            case 4:
                return go(grid, 0, 1, _left) or go(grid, 1, 0, up);
        }
        return false;
    }
};

int main() {
    Solution a;

    return 0;
}

