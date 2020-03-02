#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
#include <bitset>
#include <cmath>
using namespace std;

class Solution {
public:
    // it requires prior knowledge: the graycode can be converted from normal binary
    // for n bits labeled from n-1, n-2, .. 0, here is the formula:
    // GrayBit(i) = NormalBit(i) xor NormalBit(i-1)
    vector<int> grayCode(int n) {
        unsigned long long binary = 0;
        int len = pow(2,n);
        vector <int > res (len, 0);
        for (int i=0; i<len; i ++, binary ++) {
            unsigned long long t = 0;
            for (int j=0; j<n; j ++) {
                int shift = n - j - 1;
                t <<= 1;
                t ^= ((binary >> shift) & 1) xor ((binary >> (shift + 1)) & 1);
            }
            res[i] = t;
        }
        return res;
    }
};

int main() {
    Solution s;
    auto x = s.grayCode(2);
    for (auto i: x) {
        cout << i << ' ';
    }
    return 0;
}