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
private:
    vector<vector<int>> subo;   /* subordinates */
    vector<int> info_time;
public:
    int dfs(int id) {
        if (subo[id].size() == 0) return 0;
        int res = 0;
        for (int sub: subo[id])
            res = max(res, dfs(sub) + info_time[id]);
        return res;
    }

    int numOfMinutes(int n, int headID, vector<int>& manager, vector<int>& informTime) {
        subo.resize(n, vector<int>());
        info_time.assign(informTime.begin(), informTime.end());
        
        for (int i=0; i<n; i ++)
            if (manager[i] != -1)
                subo[manager[i]].push_back(i);
        /* this is called  `Adjacency list`  */
        // dfs can use recursive, do not need stack
        return dfs(headID);
    }
};

int main() {
    Solution s;
    return 0;
}

