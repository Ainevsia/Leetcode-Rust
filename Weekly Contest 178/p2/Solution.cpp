#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
public:
    bool isSubPath(ListNode* head, TreeNode* root) {
        if ( head == NULL ) return true;
        if ( root == NULL) return false;
        if ( head->val == root->val and this->con(head, root) )
            return true;
        auto l = this->isSubPath(head, root->left);
        auto r = this->isSubPath(head, root->right);
        return l or r;
    }

    bool con(ListNode* head, TreeNode* root) {  //consecutive
        if ( head == NULL ) return true;
        if ( root == NULL) return false;
        if ( head->val == root->val ) {
            auto l = this->con(head->next, root->left);
            auto r = this->con(head->next, root->right);
            return l or r;
        }
        return false;
    }
};

int main() {
    Solution s;
}
