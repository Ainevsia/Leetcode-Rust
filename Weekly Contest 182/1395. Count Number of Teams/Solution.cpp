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
    int numTeams(vector<int>& rating) {
        int len = rating.size(), res = 0;
        for (int i = 0; i + 2 < len; i ++) {
            for ( int j = i + 1; j + 1 < len; j ++) {
                if (rating[i] < rating[j]) {
                    int small = rating[j];
                    for ( int k = j + 1; k < len; k ++) {
                        if (rating[k] > small) res ++ ;
                    }
                } else if (rating[i] > rating[j]) {
                    int large = rating[j];
                    for ( int k = j + 1; k < len; k ++) {
                        if (rating[k] < large) res ++;
                    }
                }
            }
        }
        return res;
    }
};

int main() {
    Solution a;

    return 0;
}

