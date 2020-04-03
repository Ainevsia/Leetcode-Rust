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

// this is a two pointer method
class Solution {
public:
    int next(int n) {
        int sum = 0;
        while (n != 0) {
            int d = n % 10;
            n /= 10;
            sum += d * d;
        }
        return sum;
    }
    
    // two pointer
    bool isHappy(int n) {
        if (n <= 0) return false;
        int slow = next(n);
        int fast = next(slow);
        while (true) {
            if (fast == slow) break;
            slow = next(slow);
            fast = next(next(fast));
            if (fast == 1) return true;
        }
        return fast == 1 ? true : false;
    }
};

int main() {
    Solution a;
    return 0;
}

