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
    // dp orz
    int maximalRectangle(vector<vector<char>>& matrix) {
        if (matrix.size() == 0 or matrix[0].size() == 0) return 0;
        vector<int> height  (matrix[0].size(), 0);
        vector<int> left    (matrix[0].size(), 0);
        vector<int> right   (matrix[0].size(), matrix[0].size()-1);
        int max_area = 0;
        for (int i=0; i<matrix.size(); i++) {
            // update height
            for (int j=0; j<matrix[0].size(); j++) {
                if (matrix[i][j] == '1') {
                    height[j] ++ ;
                } else {
                    height[j] = 0;
                }
            }
            
            // update left
            int lb = 0;
            for (int j=0; j<matrix[0].size(); j++) {
                if (matrix[i][j] == '1') {
                    left[j] = max(lb, left[j]);
                } else {
                    left[j] = 0;
                    lb = j + 1;
                }
            }
            
            // update right
            int rb = matrix[0].size() - 1;
            for (int j=rb; j>=0; j--) {
                if (matrix[i][j] == '1') {
                    right[j] = min(rb, right[j]);
                } else {
                    right[j] = matrix[0].size() - 1;
                    rb = j - 1;
                }
            }
            
            // update max_area
            for (int j=0; j<matrix[0].size(); j++) {
                if (matrix[i][j] == '1') {
                    max_area = max(max_area, height[j]*(right[j] - left[j] + 1));
                }
            }
            // for (auto i:left ) cout << i << ' ';
            // cout << endl;
        }
        return max_area;
    }
};

int main() {
    Solution s;
    return 0;
}

