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
    // trival recursive version, passing vector
    vector<int> inorderTraversal(TreeNode* root) {
        vector<int>res;
        if (root == NULL)
            return res;
        res = inorderTraversal(root->left);
        res.push_back(root->val);
        auto r = inorderTraversal(root->right);
        res.insert(res.end(), r.begin(), r.end());
        return res;
    }
};

int main() {
    Solution s;
    return 0;
}

