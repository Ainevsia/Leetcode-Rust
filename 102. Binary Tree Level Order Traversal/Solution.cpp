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
    vector<vector<int>> levelOrder(TreeNode* root) {
        vector<vector<int>> res ;
        if (root == NULL) return res;
        deque<TreeNode*> q (1, root);
        while (not q.empty()) {
            // traverse this level , ie nodes in the q
            int cnt = q.size();
            vector<int> levelres ;
            for (int i=0; i<cnt; i++) {
                TreeNode * p = q.front();
                q.pop_front();
                if (p->left  != NULL) q.push_back(p->left);
                if (p->right != NULL) q.push_back(p->right);
                levelres.push_back(p->val);
            }
            res.push_back(levelres);
        }
        return res;
    }
};

int main() {
    Solution a;

    return 0;
}

