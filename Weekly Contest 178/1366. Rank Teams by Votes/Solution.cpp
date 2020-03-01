#include <string>
#include <unistd.h>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>

using namespace std;
int cnt[26][26];
class Solution {
public:
    string rankTeams(vector<string>& votes) {
        string res = votes[0];
        const int n = votes[0].size();
        vector<vector<int>> rank (26, vector <int> (n, 0));
        
        for (auto str : votes)
            for (int i=0; i<str.size(); i++)
                rank[str[i]-'A'][i] ++ ;
        
        auto cmp = [&] (const char & a, const char & b) -> bool {
            for (int i=0; i<n; )
                if (rank[a-'A'][i] == rank[b-'A'][i]) i ++ ;
                else return rank[a-'A'][i] > rank[b-'A'][i];
            return a < b;
        };

        sort(res.begin(), res.end(), cmp);
        return res;
    }
};

int main() {
    Solution s;
    cout << sizeof(cnt);
    return 0;
}
