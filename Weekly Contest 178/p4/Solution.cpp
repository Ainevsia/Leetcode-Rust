#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
using namespace std;

class Solution {
public:
    int minCost(vector<vector<int>>& grid) {
        
    }

    bool bt(vector<vector<int>>& g, vector<vector<int>>& v, 
            vector<vector<int>>& c, int & cost, int i, int j ) {
        int m = g.size(), n = g[0].size();
        if (i < 0 or i >= m or j < 0 or j >= n) return false;
        if (i==m-1 and j == n-1) return true;
        if (v[i][j] == true) return false;
        int dir = g[i][j], mincost = 1<<30;
        v[i][j] = true;
        for (int d = 0; d<4; d++) {
            int cur_dir = (dir + d) % 4;
            switch (cur_dir) {
                case 1:
                    if (this->bt(g,v,c,mincost,i,j+1)) {
                        if 
                    }
                    break;
                case 2:
                case 3:
                case 4:
            }
            if (this->bt(g,v,c,cost,i,j))
        }

    }
};

int main() {
    Solution s;

}
