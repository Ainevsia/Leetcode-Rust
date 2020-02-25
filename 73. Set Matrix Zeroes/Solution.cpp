#include <string>
#include <vector>
#include <iostream>
#include <sstream>
using namespace std;

// 5.9 ratio
class Solution {
public:
    // straight forward version
    void setZeroes1(vector<vector<int>>& matrix) {
        int m = matrix.size();
        if (m == 0) return;
        int n = matrix[0].size();
        vector<vector<int>> mx = matrix;
        for (int i=0; i<m; i++) {
            for (int j=0; j<n; j++) {
                if (mx[i][j] == 0) {
                    for (int i=0; i<m; i++) {
                        matrix[i][j] = 0;
                    }
                    for (int j=0; j<n; j++) {
                        matrix[i][j] = 0;
                    }
                }
            }
        }
        
    }

    // straight forward version
    void setZeroes2(vector<vector<int>>& matrix) {
        vector<bool> icz (matrix.size(), false);
        if (icz.size() == 0) return;
        vector<bool> jcz (matrix[0].size(), false);
        for (int i=0; i<matrix.size(); i++) {
            for (int j=0; j<matrix[0].size(); j++) {
                if ( matrix[i][j] == 0 ) {
                    icz[i] = true;
                    jcz[j] = true;
                }
            }
        }
        for (int i=0; i<matrix.size(); i++) {
            for (int j=0; j<matrix[0].size(); j++) {
                if ( icz[i] or jcz[j] ) {
                    matrix[i][j] = 0;
                }
            }
        }
        
    }

    // constant space
    void setZeroes(vector<vector<int>>& matrix) {
        bool frow = false;
        for (int i=0; i<matrix.size(); i++) {
            for (int j=0; j<matrix[0].size(); j++) {
                if ( matrix[i][j] == 0 ) {
                    if (i == 0) frow = true;
                    else matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for (int i=1; i<matrix.size(); i++) {
            for (int j=1; j<matrix[0].size(); j++) {
                if (matrix[i][0] == 0 or matrix[0][j] == 0)
                    matrix[i][j] = 0;
            }
        }
        if (matrix[0][0] == 0)
            for (int i=0; i<matrix.size(); i++)
                matrix[i][0] = 0;
        if (frow)
            for (int j=0; j<matrix[0].size(); j++)
                matrix[0][j] = 0;
        
    }
};

int main() {
    Solution a;
}