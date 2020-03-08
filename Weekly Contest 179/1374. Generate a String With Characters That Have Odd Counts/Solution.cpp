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
    string generateTheString(int n) {   /* 1 <= n <= 500 */
        string res(n-1,'a');
        if (n%2)    /* n is odd number */
            res += 'a';
        else
            res += 'b';
        return res;
    }
};

int main() {
    Solution s;
    s.generateTheString(1);
    return 0;
}

