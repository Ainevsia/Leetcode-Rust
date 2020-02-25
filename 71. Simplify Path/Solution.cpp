#include <string>
#include <vector>
#include <iostream>
#include <sstream>
using namespace std;

class Solution {
public:
    string simplifyPath(string path) {
        string buf;
        vector<string> stk;
        stringstream ss(path);
        while (getline(ss, buf, '/')) {
            if (buf == "" || buf == ".") continue;
            if (buf == ".." and not stk.empty()) stk.pop_back();
            else if (buf != "..") stk.push_back(buf);
        }
        string res;
        for (auto str: stk) {
            res += '/' + str;
        }
        return res.empty() ? "/" : res;
    }
};


int main() {
    Solution a;
    string x = a.simplifyPath("sg");
    cout << x;
}