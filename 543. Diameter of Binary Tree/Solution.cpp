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
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
private:
    int cnt;
public:
    Solution():cnt(0) {}

    int depth(TreeNode * root) {
        if (root == NULL) return 0;
        int l = depth(root->left), r = depth(root->right);
        if (l + r > cnt) cnt = l + r;
        return max(l, r) + 1;
    }

    int diameterOfBinaryTree(TreeNode* root) {
        depth(root);
        return cnt;
    }
};

int main() {
    Solution a;

    return 0;
}

