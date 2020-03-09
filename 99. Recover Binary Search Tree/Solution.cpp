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

// never use magic number !
class Solution {
private:
    TreeNode * sw1;
    TreeNode * sw2;
    TreeNode * prev;
public:
    Solution():sw1(NULL), sw2(NULL), prev(new TreeNode(1 << 31)) {}

    void traverse(TreeNode * root) {
        if (root == NULL) return ;
        traverse(root->left);
        // in-order traverse
        if (sw1 == NULL and prev->val >= root->val) sw1 = prev;
        if (sw1 != NULL and prev->val >= root->val) sw2 = root;
        prev = root;
        traverse(root->right);
    }

    void swap() {
        int tmp = sw1->val;
        sw1->val = sw2->val;
        sw2->val = tmp;
    }

    void recoverTree(TreeNode* root) {
        traverse(root);
        swap();
    }
};


int main() {
    Solution s;
    return 0;
}

