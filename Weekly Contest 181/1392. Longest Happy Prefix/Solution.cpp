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

class Solution_ {
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

class Solution__ {
public:
    // i am too young to learn the power of stl
    string longestPrefix(string s) {
        for (int i =s.size()-1; i>0; i--) {
            if (equal(s.end()-i, s.end(), s.begin()) )
                return s.substr(0, i);
        }
        return "";
    }
};

class Solution {
public:
    long R = 1e11 + 9;
    int M = 26;

    // Rabin-Karp
    // s contains only lowercase English letters.
    // notice **longest** !
    string longestPrefix(string s) {
        // prefix hash
        long lhash = 0, rhash = 0, W = 1, found = 0;
        for (int l = 1; l < s.size(); l ++) {
            // l -> the length of the prefix
            lhash = (lhash * M + (s[l - 1] - 'a')) % R;
            rhash = (rhash + (s[s.size()- l] - 'a') * W) % R;
            W = W * M % R;
            if (lhash == rhash) {
                found = l;
            }
        }
        return string(s.begin(), s.begin() + found);
    }
};

int main() {
    Solution a;
    string str;
    cin >> str;
    // cout << str;
    string x = a.longestPrefix(str);
    cout << x ;
    return 0;
}

