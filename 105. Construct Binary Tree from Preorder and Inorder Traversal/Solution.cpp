#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
#include <unordered_map>
#include <cmath>
#include <queue>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
public:
    TreeNode * buildSubTree(vector<int> preorder, vector<int> inorder) {
        int n = preorder.size();
        if (n == 0) return NULL;
        TreeNode * head = new TreeNode(preorder[0]);
        if (n == 1) return head;
        int l = 0;
        for (; l < n; l ++ ) {
            if (inorder[l] == preorder[0]) break;
        }
        vector<int> leftpre, leftin, rightpre, rightin;
        leftpre.assign(preorder.begin() + 1, preorder.begin() + 1 + l);
        rightpre.assign(preorder.begin() + 1 + l, preorder.end());
        leftin.assign(inorder.begin(), inorder.begin() + l);
        rightin.assign(inorder.begin() + l + 1, inorder.end());
        head->left = buildSubTree(leftpre, leftin);
        head->right = buildSubTree(rightpre, rightin);
        return head;
        
    }

    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        return buildSubTree(preorder, inorder);
    }
};

int main() {
    Solution a;
    vector <int > preorder = {3,9,20,15,7};
    vector <int > inorder =  {9,3,15,20,7};
    a.buildTree(preorder, inorder);
    return 0;
}

