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
class CustomStack {
public:
    vector<int> s;
    int l;
    
    CustomStack(int maxSize) {
        l = maxSize;
        s.clear();
        s.reserve(l);
    }
    
    void push(int x) {
        if (s.size()>=l) return;
        s.push_back(x);
    }
    
    int pop() {
        if (s.size()==0) return -1;
        int res = s.back();
        s.pop_back();
        return res;
    }
    
    void increment(int k, int val) {
        int i = 0;
        while (k>0 and i<s.size()) {
            s[i] += val;
            k--;
            i++;
        }
    }
};

/**
 * Your CustomStack object will be instantiated and called as such:
 * CustomStack* obj = new CustomStack(maxSize);
 * obj->push(x);
 * int param_2 = obj->pop();
 * obj->increment(k,val);
 */

int main() {
    return 0;
}

