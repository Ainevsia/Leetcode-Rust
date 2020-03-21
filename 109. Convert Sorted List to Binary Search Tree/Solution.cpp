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

    // two pointers
    TreeNode* sortedListToBST(ListNode* head) {
        if (head == NULL) return NULL;
        if (head->next == NULL) return new TreeNode(head->val);
        // at least 2 nodes
        ListNode * fast = head , * slow = head;
        while (fast != NULL and fast->next != NULL) {
            slow = slow->next;
            fast = fast->next->next;
        }
        // 1 -> 2 -> 3 -> 4 -> NULL
        //           ^slow     ^fast
        // 1 -> 2 -> 3 -> NULL
        //      ^s   ^fast
        TreeNode * res = new TreeNode(slow->val);
        if (slow != head) {
            ListNode * p = head;
            while (p->next != slow) p = p->next;
            p->next = NULL;
            res->left = sortedListToBST(head);
        } else {
            res->left = NULL;
        }
        res->right = sortedListToBST(slow->next);
        return res;
    }
};

int main() {
    Solution a;

    return 0;
}

