#include <string>
#include <vector>
#include <iostream>
#include <stdio.h>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
#include <unordered_map>
#include <map>
#include <cmath>
#include <queue>
using namespace std;

enum Ops {Add, Minus, Mul, Div, NotValid};

Ops operator++ (Ops & rs, int) {
    Ops old_enum = rs;
    rs = (Ops)(rs + 1);
    return old_enum;
}

class Solution {
public:
    bool judgePoint24(vector<int>& nums) {
        for (int i = 0; i < 24; i ++ ) {
            next_permutation(nums);
            int x1 = nums[0], x2 = nums[1], x3 = nums[2], x4 = nums[3];
            // printf("%d %d %d %d\n", x1, x2, x3, x4);
            for (Ops i = Add; i <= Div; i ++) {
                for (Ops j = Add; j <= Div; j ++) {
                    for (Ops k = Add; k <= Div; k ++) {
                        // if (x1 == 6 and x2 == 1 and x3 == 3 and x4 == 4 and 
                        //     i == Div and j == Minus and k == Div) {
                        //         cout << to_str(i) << to_str(j) << to_str(k) << endl;
                        //         cout << abs(oper(i, oper(j, oper(k, x3, x4), x2), x1)) << endl;
                        //     }
                            
                        if (1e-5 > abs(oper(k, oper(j, oper(i, x1, x2), x3), x4) - 24) ||
                            1e-5 > abs(oper(j, oper(i, x1, x2), oper(k, x3, x4)) - 24) ||
                            1e-5 > abs(oper(i, x1, oper(k, oper(j, x2, x3), x4)) - 24) ||
                            1e-5 > abs(oper(i, x1, oper(j, x2, oper(k, x3, x4))) - 24) )
                            return true;
                    }
                }
            }
        }
        return false;
    }

    double oper(Ops op, double x, double y) {
        switch (op) {
            case Add:
                return x + y;
            case Minus:
                return x - y;
            case Mul:
                return x * y;
            case Div:
                return x / y;
            default:
                return x;
        }
    }

    void next_permutation(vector<int> & nums) {
        int i = nums.size() - 1;
        while (i >= 0) {
            if (i != 0 and nums[i - 1] < nums[i] ) {
                int j = 3;
                while (nums[j] <= nums[i - 1]) j -- ;
                swap(nums[i - 1], nums[j]);
                reverse(nums.begin() + i, nums.end());
                return;
            } else if (i == 0) {
                reverse(nums.begin(), nums.end());
                return;
            }
            i -- ;
        }
    }

    string to_str(Ops op) {
        switch (op) {
            case Add:
                return "Add ";
            case Minus:
                return "Minus ";
            case Mul:
                return "Mul ";
            case Div:
                return "Div ";
            default:
                return "Not ";
        }
    }
};

int main() {
    Solution a;
    vector<int> v ;
    vector<vector<int>> b;
    b.push_back(v);
    v.clear();
    // v.push_back(4);
    // v.push_back(1);
    // v.push_back(8);
    // v.push_back(7);
    v.push_back(1);
    v.push_back(3);
    v.push_back(4);
    v.push_back(6);
    auto x = a.judgePoint24(v);
    cout << x;
    return 0;
}

