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
    int rev_pair;
    vector<int> n;
public:
    Solution():rev_pair(0) {}

    void mergeSort(int l, int r) {
        if (l + 1 >= r) return; // zero or one element
        if (l + 2 == r) {
            int large = max(n[l], n[l + 1]);
            int small = min(n[l], n[l + 1]);
            if (small != n[l]) {
                rev_pair ++;
                n[l] = small;
                n[l + 1] = large;
            }
        } else {
            int m = (l + r) / 2;
            mergeSort(l, m);
            mergeSort(m, r);
            // merge the two arrays
            vector<int> buf;
            buf.reserve(r - l);
            int i = 0, j = 0;
            while (i + l < m and j + m < r) {
                if (n[i + l] > n[j + m]) {
                    buf.push_back(n[j++ + m]);
                    rev_pair += m - l - i;
                } else buf.push_back(n[i++ + l]);
            }
            while (i + l < m) buf.push_back(n[i++ + l]);
            while (j + m < r) buf.push_back(n[j++ + m]);
            for (int i = 0; i < buf.size(); i++)
                n[i + l] = buf[i];
        }

    }

    int reversePairs(vector<int>& nums) {
        if (nums.size() <= 1) return 0;
        n.assign(nums.begin(), nums.end());
        mergeSort(0, nums.size());
        return rev_pair;
    }
};

int main() {
    Solution a;
    vector <int > v {1,3,2,3,1};
    int x = a.reversePairs(v);
    cout << x;
    return 0;
}

