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
    int numOfMinutes(int n, int headID, vector<int>& manager, vector<int>& informTime) {
        vector<vector<int>> subo (n, vector<int>());
        for (int i=0; i<n; i ++) {
            if (manager[i] != -1) {
                subo[manager[i]].push_back(i);
            }
        }
        vector<int> time(n, 0); // time to get informed
        deque<int> send;
        send.push_back(headID);
        while (not send.empty()) {
            int sender = send.front();
            send.pop_front();
            for (auto i : subo[sender]) {
                time[i] = time[sender] + informTime[sender];
                send.push_back(i);
            }
        }
        int res = 0;
        for (auto i : time) if (i > res) res = i;
        return res;
    }
};

int main() {
    Solution s;
    return 0;
}

