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
    int climbStairs(int n) {
        vector <int> fib (n+1, 1);
        for (int i=2; i<=n;i++) {
            fib[i] = fib[i-1] + fib[i-2];
        }
        // for (int i: fib) cout << i;
        return fib[n];
    }
};

int main() {
    Solution a;

    return 0;
}

