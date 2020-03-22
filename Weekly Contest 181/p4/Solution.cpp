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
    // both end inclusive
    bool _equal(string & s1, int i1, int j1, string & s2, int i2, int j2) {
        if (j1 - i1 != j2 - i2) return false;
        int len = j1 - i1 + 1;
        for (int i = 0; i < len ; i++) {
            if (s1[i1+i] != s2[i2+i]) return false;
        }
        return true;
    }
    
    string longestPrefix(string s) {
        int len = s.size();
        int maxl = 0;
        for (int i =0; i+1<len; i++) {
            if (_equal(s,0,i, s,len-1-i,len-1)) maxl = i+1;
            if (i % 100 == 0) cout << i / double (len) * 100 << "%\r";
        }
        string res;
        // time 
        res.assign(s.begin(),s.begin()+maxl);
        return res;
    }

    // speed of cpu : 50e8 instruction per second
    // N^2 Algorithm
    // if N = 3e5, 30w
    // (3e5)^2 * 10 (10 instruction per N) / 5e9 / 60 = 3 min
    // actual (include IO) : 4min30s
};

int main() {
    Solution a;
    string str;
    cin >> str;
    string x = a.longestPrefix(str);
    cout << x << "hel\n";
    return 0;
}

