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
    int superEggDrop(int K, int N) {
        vector<int> oldv (K + 1, 0);
        vector<int> newv (K + 1, 0);
        int cnt = 0;
        while (newv[K] < N) {
            for (int i = 1; i <= K; i ++) {
                newv[i] = oldv[i] + oldv[i - 1] + 1;
            }
            cnt ++ ;
            oldv.assign(newv.begin(), newv.end());
        }
        return cnt;
    }
};

int main() {
    Solution a;

    return 0;
}

