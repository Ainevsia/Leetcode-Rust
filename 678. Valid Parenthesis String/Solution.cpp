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
public:
    bool checkValidString(string s) {
        int wildcard_cnt = 0;
        int stack = 0;
        for (int i = 0; i < s.size(); i ++ ) {
            switch (s[i]) {
                case '(':
                    stack  ++ ;
                    break;
                case ')':
                    if (stack > 0) stack -- ;
                    else if (stack == 0) {
                        if (wildcard_cnt > 0) wildcard_cnt -- ;
                        else return false;
                    }
                    break;
                case '*':
                    wildcard_cnt ++ ;
                    break;
            }
        }
        if (stack > wildcard_cnt) return false;
        stack = wildcard_cnt = 0;
        for (int i = s.size() - 1; i >= 0; i -- ) {
            switch (s[i]) {
                case ')':
                    stack  ++ ;
                    break;
                case '(':
                    if (stack > 0) stack -- ;
                    else if (stack == 0) {
                        if (wildcard_cnt > 0) wildcard_cnt -- ;
                        else return false;
                    }
                    break;
                case '*':
                    wildcard_cnt ++ ;
                    break;
            }
        }
        return stack <= wildcard_cnt;
    }
};

int main() {
    Solution a;
    bool x = a.checkValidString("(");
    cout << x;
    return 0;
}

