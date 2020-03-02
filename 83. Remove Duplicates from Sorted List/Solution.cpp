#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
using namespace std;


struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
    // this is a non recursive version
    ListNode* deleteDuplicates(ListNode* head) {
        if (head == NULL or head->next == NULL) return head;
        auto ptr = head;
        while (ptr != NULL and ptr->next != NULL) {
            if (ptr->next->val == ptr->val) {
                auto p = ptr->next;
                while (p != NULL and p->val == ptr->val) {
                    p = p->next;
                }
                ptr->next = p;
            }
            ptr = ptr->next;
        }
        return head;
    }

    // this is a recursive version
    ListNode* deleteDuplicates(ListNode* head) {
        if (head == NULL or head->next == NULL) return head;
        if (head->next->val != head->val) {
            head->next = this->deleteDuplicates(head->next);
            return head;
        }
        auto p = head->next;
        while (p != NULL and p->val == head->val) p = p -> next;
        head->next = p == NULL ? p : this->deleteDuplicates(p);
        return head;
    }

};

int main() {
    Solution s;
    return 0;
}

