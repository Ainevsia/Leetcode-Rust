#include <unistd.h>

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
    ListNode* deleteDuplicates(ListNode* head) {
        if (head == NULL || head->next == NULL) return head;
        auto p = head, c = head->next;
        while (c != NULL && p->val == c->val) c = c->next;
        if (c != p->next) return this->deleteDuplicates(c);
        auto rest = this->deleteDuplicates(c);
        p->next = rest;
        return p; 
    }
};