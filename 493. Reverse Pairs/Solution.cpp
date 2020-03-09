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
    Solution():rev_pair(0) {}

    void mergeSort(int l, int r) {
        if (l + 1 >= r) return; // zero or one element
        if (l + 2 == r) {
            if (n[l] >= n[l + 1]) {
                if (n[l] > n[l + 1] * 2L) rev_pair ++ ;
                n[l] ^= n[l + 1] ^= n[l] ^= n[l + 1];
            }
            return;
        }
        int m = (l + r) / 2;
        mergeSort(l, m);
        mergeSort(m, r);
        int i = 0, j = 0;
        // i dont know why i am getting wrong
        while (i + l < m and j + m < r) {
            while (j + m < r and n[i + l] > 2L * n[j + m]) j++;
            rev_pair += j; 
            i ++;
        }
        // merge sort
        vector<int> buf;
        buf.reserve(r - l);
        for (i = j = 0; i + l < m and j + m < r;) {
            if (n[i + l] > n[j + m]) buf.push_back(n[j++ + m]);
            else buf.push_back(n[i++ + l]);
        }
        while (i + l < m) buf.push_back(n[i++ + l]);
        while (j + m < r) buf.push_back(n[j++ + m]);
        for (int i = 0; i < buf.size(); i++)
            n[i + l] = buf[i];
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
    vector <int > v {-5,-5};
    int x = a.reversePairs(v);
    cout << x;
    for (auto i:a.n) cout << i << ' ';
    cout << (-1 << 1);
    return 0;
}

