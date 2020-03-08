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
    vector<vector<int>> ut; // undirected tree
    vector<bool> visited;
    int target;
    double p = 0.0;
public:
    /* get the number of childs of the node id */
    int getchildn(int id) {
        for (vector<int>::iterator it = ut[id].begin(); it < ut[id].end(); ) {
            if (visited[*it]) ut[id].erase(it);
            else it ++ ;
        }
        visited[id] = true;
        return ut[id].size();
    }

    // p -> possibility to reach this node `id`
    void dfs(int id, double p, int round_left) {
        if (round_left == 0) {  // cannot travel any farther
            if (id == target) this->p = p;
            return;
        } 
        int childn = getchildn(id);
        if (id == target and childn == 0) { // reach target and nowhere else to go
            this->p = p;
            return;
        } else if (id == target or childn == 0) return;
        p /= childn;
        for (int cid : ut[id])
            dfs(cid, p, round_left-1);
    }

    double frogPosition(int n, vector<vector<int>>& edges, int t, int target) {
        visited.resize(n+1, false);
        ut.resize(n+1);
        this->target = target;
        for (vector<int> edge : edges) {
            int from = edge[0], to = edge[1];
            ut[from].push_back(to);
            ut[to].push_back(from);
        }
        dfs(1, 1, t);
        return p;
    }
};

int main() {
    Solution s;
    return 0;
}

