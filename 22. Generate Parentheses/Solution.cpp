#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
using namespace std;

class Solution {
public:
    // recursive : returning the result back
    vector<string> generateParenthesis_recur(int n) {
        return this->genp(n, n);
    }
    
    vector<string> genp(int l, int r) {
        vector<string> res;
        if (l == 0 and r == 1) {
            res.push_back(")");
            return res;
        }
        if (l > 0) {
            for (auto str: genp(l-1, r)) {
                str.insert(0,1,'(');
                res.push_back(str);
            }
        }
        if (r > 0 and r > l) {
            for (auto str: genp(l, r-1)) {
                str.insert(0,1,')');
                res.push_back(str);
            }
        }
        return res;
    }

    // backtracing
    /*  The idea is intuitive. 
        Use two integers to count the remaining left parenthesis (n) 
        and the right parenthesis (m) to be added. 
        At each function call add a left parenthesis if n > 0 and 
        add a right parenthesis if m>0. Append the result and terminate recursive calls 
        when both m and n are zero. 
    */
    vector<string> generateParenthesis(int n) {
        vector<string> res;
        string s = "";
        this->bt(res,n,n,s);
        return res;
    }
    
    void bt(vector<string> & v, int l, int r, string & str) {
        if (l == 0 and r == 0) {
            v.push_back(str);
            return;
        }
        if (l > 0) {
            str.push_back('(');
            this->bt(v, l-1, r, str);
            str.pop_back();
        }
        if (r > l and r > 0) {
            str.push_back(')');
            this->bt(v, l, r-1, str);
            str.pop_back();
        }
        return;
    }
};

int main() {
    Solution s;
    return 0;
}

