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
    // trival recursive version, optimize space
    vector<int> inorderTraversal(TreeNode* root) {
        vector<int>res;
        help(root, res);
        return res;
    }

    void help(TreeNode * root, vector<int> & res) {
        if (root == NULL) return;
        help(root->left, res);
        res.push_back(root->val);
        help(root->right, res);
    }
};

int main() {
    Solution s;
    return 0;
}

