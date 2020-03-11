#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
#include <unordered_map>
#include <cmath>
using namespace std;

class Solution {
public:
    bool isInterleave(string s1, string s2, string s3) {
        int i=0, j=0, k=0;
        int l1 = s1.size(), l2 = s2.size(), l3 = s3.size();
        if (l1 + l2 != l3) return false;
        for (; k < l3; k ++ ) {
            cout << i << ' ' << j << ' ' << k << endl;
            if (i >= l1) {
                if (j >= l2 or s2[j] != s3[k]) return false;
                else j ++ ;
            } else if (j >= l2) {
                if (i >= l1 or s1[i] != s3[k]) return false;
                else i ++ ;
                // now both i j are valid
            } else if (s1[i] == s3[k] and s2[j] != s3[k]) {
                i ++ ;
            } else if (s1[i] != s3[k] and s2[j] == s3[k]) {
                j ++ ;
            } else if (s1[i] != s3[k] and s2[j] != s3[k]) {
                return false;
            } else {
                // s1[i] == s3[k] and s2[j] == s3[k]
                int target = k;
                char targetc = s3[k];
                while (target < l3 and s3[target] == targetc) target ++ ;
                if (target >= l3) /* can choose randomly */ {
                    i ++ ; continue;
                }
                cout << targetc;
                int idxi = i, idxj = j;
                while(idxi < l1 and s1[idxi] == targetc) idxi ++ ;
                while(idxj < l2 and s2[idxj] == targetc) idxj ++ ;
                // cout << idxi << idxj;
                if (idxi >= l1 and idxj >= l2) return false;
                else if (idxi >= l1 and idxj < l2) {
                    if (s2[idxj] == s3[target]) {
                        j ++ ; continue;
                    } else return false;
                } else if (idxi < l1 and idxj >= l2) {
                    if (s1[idxi] == s3[target]) {
                        i ++ ; continue;
                    } else return false;
                } else {
                    if (s1[idxi] == s3[target]) {
                        i ++ ;
                    } else if (s2[idxj] == s3[target]) {
                        j ++ ;
                    } else return false;
                }
            }
        }
        return true;
    }
};

int main() {
    Solution a;
    for (int i=0; i<10; i++) {
        cout << i;
        continue;
    }
    bool res = a.isInterleave("aabcc", "dbbca", "aadbbcbcac");
    cout << res;
    return 0;
}

