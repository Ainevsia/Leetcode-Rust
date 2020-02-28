#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
using namespace std;



class Solution {
public:
    bool isPart(string str) {
        if (str.size() > 3 or str.size() == 0) return false;
        int i = stoi(str);
        return i <= 255 and 0 <= i and (i == 0 ? str.size() == 1 : str[0] != '0');
    }

    vector<string> ipFromNPart(int n, string str) {
        vector<string> res ;
        if (n == 1) {
            if (this->isPart(str)) res.push_back(str);
            return res;
        }
        for (int i=1; i<=3 and i < str.size() ; i++) {
            if (this->isPart(str.substr(0,i))) {
                auto possible_postfix = this->ipFromNPart(n-1,str.substr(i));
                for (auto word: possible_postfix) {
                    auto append = str.substr(0,i) + "." + word;
                    res.push_back(append);
                }
            }
        }
        return res;
    }

    // Input: "25525511135"
    // Output: ["255.255.11.135", "255.255.111.35"]
    vector<string> restoreIpAddresses(string s) {
        return this->ipFromNPart(4, s);
    }
};


int main() {
    Solution s;
    auto x = s.restoreIpAddresses("25525511135");
    // auto x = s.restoreIpAddresses("0000");
    
    // auto x = s.restoreIpAddresses("010010");
    for (auto i: x) {
        cout << i << endl;
    }
}
