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
private:
    TreeNode * sw1;
    TreeNode * sw2;
public:
    bool isValid(TreeNode* root, int mi, int ma, bool llimit, bool rlimit) {
        if (root == NULL) return true;
        if (llimit and root->val <= mi or rlimit and root->val >= ma) {
            sw2 = root;
            return false;
        }
        bool lis_valid = isValid(root->left, mi, min(ma, root->val), llimit, true);
        bool ris_valid = isValid(root->right, max(mi, root->val), ma, true, rlimit);
        if (not lis_valid and not ris_valid) {
            sw1 = root->left;
            sw2 = root->right;
            return false;
        } else if (not (lis_valid and ris_valid)) {
            sw1 = root;
            return false;
        } else return true;
    }
    
    bool isValidBST(TreeNode* root) {
        int ma = ((unsigned int) ~0) >> 1, mi = 1 << 31;
        return isValid(root, mi, ma, false, false);
    }

    void swap() {
        int tmp = sw1->val;
        sw1->val = sw2->val;
        sw2->val = tmp;
    }

    void recoverTree(TreeNode* root) {
        isValidBST(root);
        swap();
    }
};


int main() {
    Solution s;
    return 0;
}

