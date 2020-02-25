#include <string>
#include <vector>
#include <iostream>
#include <sstream>
using namespace std;

// 19
class Solution {
public:
    vector<vector<int>> combine(int n, int k) {
        vector<vector<int>> v;
        vector<int> buf;
        this->recur(n, k, v, buf);
        return v;
    }
//recur
    void recur(int n, int k, vector<vector<int>> & v, vector<int> & buf) {
        if ( k == 0 ) {
            v.push_back(buf);
            return;
        } else {
            for (int i=k; i<=n; i++) {
                buf.push_back(i);
                this->recur(i-1, k-1, v, buf);
                buf.pop_back();
            }
        }
    }
};

int main() {
    Solution a;
}