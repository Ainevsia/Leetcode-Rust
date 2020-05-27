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

class Solution {
public:
    // 2 <= n <= 58 
    int integerBreak(int n) {
        if (n == 2) return 1;
        if (n == 3) return 2;
        if (n == 4) return 4;
        int n3 = 0;
        while (n >= 5) {
            n3 ++ ;
            n -= 3;
        }
        return fast_pow(3, n3) * n;
    }

    int fast_pow(int x, int y) {
        int res = 1;
        for (int offset = sizeof(int) * 8 - 1; offset >= 0; offset --) {
            res *= res;
            if (y & (1 << offset)) res *= x;
        } 
        return res;
    }

};

int main() {
    Solution a;

    return 0;
}

