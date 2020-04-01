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
private:
    vector<vector<int>> dfa;
    string pat;
public:
    void init() {
        string pat = this->pat;
        dfa.resize(pat.size());
        int shadow_state = 0;
        dfa.push_back(vector<int>(256, 0));
        dfa[0][pat.at(0)] = 1;
        for (int i = 1; i < pat.size(); i ++ ) {
            dfa.push_back(dfa[shadow_state]);
            dfa[i][pat.at(i)] = i + 1;
            shadow_state = dfa[shadow_state][pat.at(i)];
        }
    }

    int strStr(string haystack, string needle) {
        this->pat = needle;
        if (needle.empty()) return 0;
        if (haystack.empty()) return -1;
        string txt = haystack;
        int state = 0, found = needle.size();
        for (int i = 0; i < txt.size(); i ++ ) {
            state = this->dfa[state][haystack.at(i)];
            if (state == found) return i - needle.size();
        }
        return -1;
    }
};

int main() {
    Solution a;
    a.strStr("ababac", "ababac");
    return 0;
}

