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

class Solution_my {
public:
    int findLucky(vector<int>& arr) {
        int res = -1, f = 0;
        for (int i = 0; i < arr.size(); i ++ ) {
            f = 0;
            int target = arr[i];
            for (int j : arr) {
                if (j == target) f ++ ;
            }
            if ( f == target ) {
                res = target > res ? target : res;
            }
        }
        return res;
    }
};

class Solution {
public:
    int findLucky(vector<int>& arr) {
        map<int, int> cnt;
        for (int i : arr) cnt[i] ++ ;
        vector<int> res;
        for ( auto & [k, v] : cnt ) {
            if (k == v) res.push_back(k);
        }
        if (res.empty()) return -1;
        return *max_element(res.begin(), res.end());
    }
};

int main() {
    Solution a;

    return 0;
}

