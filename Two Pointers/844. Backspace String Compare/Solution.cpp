#include <string>

using namespace std;

class Solution {
public:
    // go from back to front
    bool backspaceCompare(string s1, string s2) {
        int l1 = s1.size(), l2 = s2.size();
        string r1 = "", r2 = "";
        int scnt = 0;
        for (int i = l1 - 1; i >= 0 ; i -- ) {
            if (s1[i] == '#') scnt ++ ;
            else if (scnt > 0) scnt -- ;
            else {
                r1 += s1[i];
            }
        }
        scnt = 0;
        for (int i = l2 - 1; i >= 0 ; i -- ) {
            if (s2[i] == '#') scnt ++ ;
            else if (scnt > 0) scnt -- ;
            else {
                r2 += s2[i];
            }
        }
        return r1 == r2;
    }
};


int main(int argc, char const *argv[])
{
    
    return 0;
}
