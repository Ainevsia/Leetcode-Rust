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
    int n;

    int next(int i) {
        return i + 1 == n ? 0 : i + 1;
    }

    //If car starts at A and can not reach B. Any station between A and B can not reach B.(B is the first station that A can not reach.)
    //If the total number of gas is bigger than the total number of cost. There must be a solution.
    int canCompleteCircuit(vector<int>& gas, vector<int>& cost) {
        n = gas.size();
        for (int i = 0; i < n; ) {
            if (gas[i] < cost[i]) { i ++ ; continue; }
            int dst = i, j = next(i), rg = gas[i] - cost[i];
            while (j != dst) {
                rg += gas[j] - cost[j];
                if (rg < 0) break;
                j = next(j);
            }
            if (j != dst) {
                if (j < i) break;
                else {
                    i = j;
                    continue;
                }
            }
            else return i;
        }
        return -1;
    }
};

int main() {
    Solution a;

    return 0;
}

