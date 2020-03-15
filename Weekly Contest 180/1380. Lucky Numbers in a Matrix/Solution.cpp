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

class Solution {
public:
    vector<int> luckyNumbers (vector<vector<int>>& matrix) {
        vector<int> res;
        int m=matrix.size(),n=matrix[0].size();
        vector<int> rowmin(m,1<<30), colmax(n,0);
        
        for (int i=0;i<m;i++) {
            for(int j=0;j<n;j++) {
                if (matrix[i][j] < rowmin[i]) rowmin[i] = matrix[i][j];
                if (matrix[i][j] > colmax[j]) colmax[j] = matrix[i][j];
            }
        }
        for (int i=0;i<m;i++) {
            for(int j=0;j<n;j++) {
                if (matrix[i][j] == rowmin[i] and matrix[i][j] == colmax[j]) 
                    res.push_back(matrix[i][j]);
            }
        }
        return res;
    }
};

int main() {
    Solution s;
    return 0;
}

