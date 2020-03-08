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
    string generateTheString(int n) {
        string s;
        if (n % 2 == 0) {
            for (int i=0; i<n-1; i++) {
                s += 'a';
            }
            s += 'b';
            return s;
        } else {
            for (int i=0; i<n; i++) {
                s += 'a';
            }
            return s;
        }
    }
};

int main() {
    Solution s;
    return 0;
}

