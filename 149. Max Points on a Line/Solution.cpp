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
#include <map>
using namespace std;

// it teaches me how to use two ints to represent a double .........
class Solution {
public:
    int gcd(int a, int b) {
        if ( a < b ) {
            // make a the larger one
            int tmp = b;
            b = a;
            a = tmp;
        }
        while (b != 0) {
            int remaind = a % b;
            a = b;
            b = remaind;
        }
        return a;
    }

    /// how to use 2 integers to represent a double
    /// if 0 -> up = 0, do not represent inf
    /// if has sign, sign is on the up
    int maxPoints(vector<vector<int>>& points) {
        // edge case 
        if (points.size() <= 2) return points.size();
        int res = 0;    // max points on line
        for (int i=0; i<points.size(); i++) {
            map<pair<int,int>,int> lines;
            int overlap = 0, vertical = 0;

            for (int j=i+1; j<points.size(); j++) {
                // handle overlap cases
                if (points[i][0] == points[j][0] and points[i][1] == points[j][1]){
                    overlap ++ ;
                    continue;
                } else if (points[i][0] == points[j][0]) {
                    vertical ++ ;
                    continue;
                } else if (points[j][1] == points[i][1]) {
                    lines[make_pair(1, 0)] ++ ;
                } else {
                    int a = points[j][0] - points[i][0], b = points[j][1] - points[i][1];
                    // check sign of a and b, a and b are both not zero
                    if (a^b < 0 and a > 0) {
                        // different sign need to switch sign
                        a = -a;
                        b = -b;
                    }
                    int max_remainder = gcd(abs(a), b); // might overflow
                    a /= max_remainder;
                    b /= max_remainder;
                    lines[make_pair(a, b)] ++ ;
                }
            }
            int localres = vertical;
            for (auto i : lines) {
                if (i.second > localres) localres = i.second;
                // cout << i.first.first << i.first.second << i.second << endl;
            }
            res = max(res, localres + 1 + overlap);
        }
        return res;
    }
};

int main() {
    Solution a;
    vector<vector<int>> v = {{3,1},{12,3},{3,1},{-6,-1}};
    int res = a.maxPoints(v);
    cout << res;
    return 0;
}

