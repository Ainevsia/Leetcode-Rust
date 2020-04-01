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


/// KMP version 
class Solution_algo4 {
private:
    vector<vector<int>> dfa;
    string pat;
public:
    void init() {
        string pat = this->pat;
        dfa.clear();
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
        if (needle.empty()) return 0;
        if (haystack.empty()) return -1;
        this->pat = needle;
        this->init();
        string txt = haystack;
        int state = 0, found = needle.size();
        for (int i = 0; i < txt.size(); i ++ ) {
            state = this->dfa[state][haystack.at(i)];
            if (state == found) return i - needle.size() + 1;
        }
        return -1;
    }
};

// kmp next version
class Solution {
public:

    vector<int> getNext(string pat) {
        vector<int> next (pat.size());
        next[0] = -1;
        int i = 0, j = -1;
        while ( i < pat.size() - 1 ) {
            if (j == -1 or pat[i] == pat[j]) {
                i ++ ;
                j ++ ;
                next[i] = j;    
            } else {
                j = next[j];
            }
        }
        return next;
    }

    int strStr(string haystack, string needle) {
        if (needle.empty()) return 0;
        if (haystack.empty()) return -1;
        auto next = getNext(needle);
        // for (auto i : next) cout << i << ' ';
        // cout << endl;
        int i = 0, j = 0;
        while ( i < haystack.size() and j < (int) needle.size() ) {
            // cout << i << ' ' << j << endl;
            if (j == -1 or haystack[i] == needle[j]) {
                i ++ ;
                j ++ ;
            } else {
                j = next[j];
            }
            // cout << i << ' ' << j << endl;
            // cout << haystack.size() << needle.size();
            // cout << (j < needle.size());
        }
        // cout << j;
        if (j == needle.size()) return i - j;
        else return -1;
    }
};

int main() {
    Solution a;
    auto x = a.strStr("hello", "ll");
    cout << x;
    return 0;
}

