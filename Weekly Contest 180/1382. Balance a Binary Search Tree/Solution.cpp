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
    vector<int> n ;
        
    void into_vec(TreeNode * root) {
        if (root == NULL) return;
        into_vec(root->left);
        n.push_back(root->val);
        into_vec(root->right);
    }
    
    TreeNode * maketree(int l, int r) {
        // l inclusive, r exclusive
        if (l + 1 > r) return NULL;
        if (l + 1 == r) return new TreeNode(n[l]);
        if (l + 2 == r) {
            TreeNode * head = new TreeNode (n[l]);
            TreeNode * child = new TreeNode(n[l+1]);
            head->right = child;
            return head;
        }
        int m = (l+r)/2;
        TreeNode * left = maketree(l,m);
        TreeNode * right  = maketree(m+1,r);
        TreeNode * head = new TreeNode( n[m]);
        head->left = left;
        head->right = right;
        return head;
    }
    
    TreeNode* balanceBST(TreeNode* root) {
        into_vec(root);
        return maketree(0, n.size());
        
    }
};
int main() {
    Solution s;
    return 0;
}

