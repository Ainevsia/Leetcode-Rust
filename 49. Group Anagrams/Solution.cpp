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
    unsigned long prime = 51;

public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        vector<unsigned long> hash;
        vector<vector<string>> res;
        for (string str : strs) {
            unsigned s_hash = getHash(str);
            bool found = false;
            for (int i = 0; i < hash.size(); i ++) {
                if (hash[i] == s_hash) {
                    res[i].push_back(str);
                    found = true;
                    break;
                }
            }
            if (not found) {
                hash.push_back(s_hash);
                res.push_back({str});
            }
        }
        return res;
    }

    unsigned long getHash(string str) {
        int cnt[26] = {0};
        for (char c : str) {
            cnt[c - 'a'] ++ ;
        }
        unsigned long res = 0;
        for (int i = 0; i < 26; i++) {
            res += cnt[i] * pow(prime, i);
        }
        return res;
    }


    // O(1)
    unsigned long pow(unsigned long x, unsigned int y) {
        // bitwise traverse y
        unsigned long res = 1;
        for (int offset = sizeof(int) * 8 - 1; offset >= 0; offset --) {
            res *= res; // right shift
            if (y & (1 << offset)) res *= x;
        }
        return res;
    }
};

int main() {
    Solution a;
    vector<string> b = {"eat","tea","tan","ate","nat","bat"};
    a.groupAnagrams(b);
    return 0;
}

