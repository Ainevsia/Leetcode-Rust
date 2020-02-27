#include <vector>

using namespace std;

class Solution {
public:
    /// edge case n.size() == 0
    /// buffer overflow : not exist case !!! two element !
    bool search(vector<int>& n, int t) {
        int l = 0, r = n.size() - 1;
        if (l > r) return false;
        while (l < r) {
            int m = (l + r) / 2;
            if (m == l) return n[l] == t ? true : n[r] == t ? true: false;
            if (n[m] == t) return true;
            if (n[l] == n[r]) l++;
            else if (n[l] < n[r]) {
                if (t < n[l] or t > n[r]) return false;
                if (n[m] > t) r = m - 1;
                else l = m + 1;
            } else {
                if (n[m] >= n[l])
                    if (n[l] <= t and t <= n[m-1]) r = m-1;
                    else l = m + 1;
                else
                    if (n[m+1] <= t and t <= n[r]) l = m+1;
                    else r = m - 1;
            }
        }
        return n[l] == t ? true : false;
    }
};