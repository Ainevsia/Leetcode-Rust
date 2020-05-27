#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
#include <unordered_map>
#include <map>
#include <cmath>
#include <queue>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

// Recursive Solution
class Solution {
public:
    bool isSymmetric(TreeNode* root) {
        if (root == NULL) return true;
        else return isSymmetric(root->left, root->right);
    }

    bool isSymmetric(TreeNode * left, TreeNode * right) {
        if (left == NULL and right == NULL) return true;
        else if (left == NULL or right == NULL) return false;
        if (left->val != right->val) return false;
        return isSymmetric(left->right, right->left) and isSymmetric(left->left, right->right);
    }
};

int main() {
    Solution a;

    return 0;
}

