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
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        int x = matrix.size();
        if (x < 1) return false;
        int y = matrix[0].size();
        if (y < 1) return false;
        int i = x - 1, j = 0;
        while (i >= 0 and j < y) {
            if (matrix[i][j] == target) return true;
            else if (matrix[i][j] < target) j ++ ;
            else if (matrix[i][j] > target) i -- ;
        }
        return false;
    }
};

int main() {
    Solution a;

    return 0;
}

