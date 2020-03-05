#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
#include <unordered_map>
#include <cmath>
#include <stack>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
public:
    // Morris Traversal, do not need extra space
    // https://stackoverflow.com/questions/5502916/explain-morris-inorder-tree-traversal-without-using-stacks-or-recursion
    // just reorder the tree in the output order
    vector<int> inorderTraversal(TreeNode* p) {
        vector<int>res;
        while (p != NULL) {
            if (p->left != NULL) {
                auto lr = p->left, newp = p->left;
                while (lr->right != NULL) lr = lr->right;
                lr->right = p;
                p->left = NULL;
                p = newp;
            } else {
                res.push_back(p->val);
                p = p->right;
            }
        }
        return res;
    }
};

int main() {
    Solution s;
    return 0;
}

