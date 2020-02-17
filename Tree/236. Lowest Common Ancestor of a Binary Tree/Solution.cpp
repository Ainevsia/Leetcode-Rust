#include <unistd.h>
#include <assert.h>
#include <stdio.h>

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};


class Solution {
    public:
    TreeNode *lowestCommonAncestor(TreeNode *root, TreeNode *p, TreeNode *q) {
        if (root == q) {
            if ( this->contains(root->left, p) || this->contains(root->right, p) ) { return root; }
        } else if (root == p) {
            if ( this->contains(root->left, q) || this->contains(root->right, q)) { return root; }
        } else if ( this->contains(root->left, p) && this->contains(root->right, q) || 
                    this->contains(root->right, p) && this->contains(root->left, q) ) { return root; }
        if ( this->contains(root->left, p) ) { return this->lowestCommonAncestor(root->left, p, q); }
        else { return this->lowestCommonAncestor(root->right, p, q); }
    }

    bool contains(TreeNode * root, TreeNode * p) {
        // handle null case here
        if ( root == NULL ) { return false; }
        if ( root == p ) { return true; }
        if ( this->contains(root->left, p) ) { return true; } 
        if ( this->contains(root->right, p) ) { return true; } 
        return false;
    }
};

int main(int argc, char const *argv[])
{
    TreeNode n0 = TreeNode(0);
    TreeNode n8 = TreeNode(8);
    TreeNode n1 = TreeNode(1);
    n1.left = &n0;n1.right = &n8;
    TreeNode n4 = TreeNode(4);
    TreeNode n7 = TreeNode(7);
    TreeNode n2 = TreeNode(2);
    n2.left = &n7;n2.right = &n4;
    TreeNode n6 = TreeNode(6);
    TreeNode n5 = TreeNode(5);
    n5.left = &n6;n5.right = &n2;
    TreeNode n3 = TreeNode(3);
    n3.left = &n5;n3.right = &n1;
    Solution a;
    TreeNode * x = a.lowestCommonAncestor(&n3, &n5, &n1);
    assert(x == &n3);
    TreeNode * y = a.lowestCommonAncestor(&n3, &n5, &n4);
    assert(y == &n5);
    printf("success");
    return 0;
}
