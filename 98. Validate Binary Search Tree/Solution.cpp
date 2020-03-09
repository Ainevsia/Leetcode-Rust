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

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
public:
    bool help(TreeNode* root, int mi, int ma, bool llimit, bool rlimit) {
        if (root == NULL) return true;
        if (llimit and root->val <= mi or rlimit and root->val >= ma) return false;
        return help(root->left, mi, min(ma, root->val), llimit, true)
            and help(root->right, max(mi, root->val), ma, true, rlimit);
    }
    
    bool isValidBST(TreeNode* root) {
        int ma = ((unsigned int) ~0) >> 1;
        int mi = 1 << 31;
        return help(root, mi, ma, false, false);
    }
};

int main() {
    Solution s;
    s.isValidBST(NULL);
    return 0;
}

