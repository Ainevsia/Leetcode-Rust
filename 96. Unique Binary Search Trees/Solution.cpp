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
    int numTrees(int n) {
        deque<int> buf;
        buf.push_back(1);
        while (n-- > 1) {
            int l = buf.size();
            for(int i=0; i<l; i++) {
                int x = buf.front();
                buf.pop_front();
                for(int j=1; j<=x+1; j++) {
                    buf.push_back(j);
                }
            }
        }
        return buf.size();
    }
};


int main() {
    Solution s;
    return 0;
}

