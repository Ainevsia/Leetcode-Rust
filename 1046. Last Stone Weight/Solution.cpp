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
    int lastStoneWeight(vector<int>& stones) {
        priority_queue<int> pq (stones.begin(), stones.end());
        while (pq.size() >= 2) {
            int x = pq.top(); pq.pop();
            int y = pq.top(); pq.pop();
            if (x > y) {
                pq.push(x - y);
            }
        }
        return pq.size() == 0 ? 0 : pq.top();
    }
};


int main() {
    Solution a;

    return 0;
}

