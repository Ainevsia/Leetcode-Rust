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
    double frogPosition(int n, vector<vector<int>>& edges, int t, int target) {
        vector<vector<int>> childs (n, vector<int>());
        deque<int> knownfather ;
        knownfather.push_back(1);
        while (not knownfather.empty()) {
            int father = knownfather.front();
            knownfather.pop_front();
            for (auto edge : edges) {
                int from = edge[0], to = edge[1];
                if (from == father) {
                    childs[father].push_back(to);
                    knownfather.push_back(to);
                } 
                else if (to == father) {
                    childs[father].push_back(from);
                    knownfather.push_back(from);
                }
            }
        }
        double p = 1;
        int round = 0;
        while (++round <= t)
        
        
    }
};

int main() {
    Solution s;
    return 0;
}

