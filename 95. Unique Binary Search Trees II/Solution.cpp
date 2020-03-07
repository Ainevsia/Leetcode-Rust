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
    vector<TreeNode*> generateTrees(int n) {
        if (n == 0) return vector<TreeNode*> {};
        deque < TreeNode * > q;
        TreeNode * first = new TreeNode(1);
        q.push_back(first);
        for (int i=2; i<=n; i ++) { // i is the element to add
            int size = q.size();
            for(int _=0;  _<size; _ ++) {
                TreeNode * f = q.front();
                q.pop_front();
                TreeNode * hdr = new TreeNode(-1);
                hdr->right = f;
                int cnt = 0;
                while (true) {
                    TreeNode * newtree = deepcopy(hdr);
                    TreeNode * p = newtree;
                    int incnt = 0;
                    while (incnt++ < cnt and p->right != NULL) p = p->right; 
                    if (p != NULL and incnt == cnt + 1) {
                        cnt ++;
                        TreeNode * append = new TreeNode(i);
                        append->left = p->right;
                        p->right = append;
                        q.push_back(newtree->right);
                    } else {
                        break;
                    }
                }
            }
        }
        vector< TreeNode * > res (make_move_iterator(q.begin()), make_move_iterator(q.end()));
        return res;
    }
    
    TreeNode * deepcopy(TreeNode * root) {
        if (root == NULL) return NULL;
        TreeNode * p = new TreeNode(root->val);
        p->left = deepcopy(root->left);
        p->right = deepcopy(root->right);
        return p;
    }
};

int main() {
    Solution s;
    return 0;
}

