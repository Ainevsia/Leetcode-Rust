#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
using namespace std;

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
    ListNode* partition(ListNode* head, int x) {
        if (head == NULL) return head;
        ListNode * header = new ListNode(0);
        header->next = head;
        auto p1 = header, p2 = header;  // p1, p2 not NULL
        // p1 -> the last number < x, p2 -> cur
        while (p2 != NULL and p2->next != NULL) {    // see the next node value
            if (p2->next->val < x) {
                if ( p1 == p2 ) {
                    p1 = p1->next;
                    p2 = p2->next;
                    continue;
                }
                // swap 
                auto p3 = p2->next, p4 = p3->next;
                auto p1n = p1->next;
                p1->next = p3;
                p3->next = p1n;
                p2->next = p4;
                p1 = p1->next;
            } else 
                p2 = p2->next;
        }
        return header->next;
    }
};

int main() {
    Solution s;
    ListNode a(1);
    ListNode b(4);
    ListNode c(3);
    ListNode d(2);
    ListNode e(5);
    ListNode f(2);
    a.next = &b;
    b.next = &c;
    c.next = &d;
    d.next = &e;
    e.next = &f;
    auto x = s.partition(&a,3);
    while (x != NULL) {
        cout << x->val << endl;
        x = x->next;
    }
    return 0;
}
