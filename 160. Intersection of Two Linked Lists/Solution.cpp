#include <assert.h>

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:

    /* two pointers */
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        if (!headA || !headB) { return NULL; }
        ListNode * p1 = headA, * p2 = headB;
        bool switched1 = false, switched2 = false;
        while (true) {
            if (p1 == p2) {
                return p1;
            }
            p1 = p1->next;
            if (p1 == NULL) {
                switch (switched1) {
                    case true:
                        return NULL;
                    default:
                        switched1 = true;
                        p1 = headB;
                }
            }
            p2 = p2->next;
            if (p2 == NULL) {
                switch (switched2) {
                    case true:
                        return NULL;
                    default:
                        switched2 = true;
                        p2 = headA;
                }
            }
        }
    }
};

int main(int argc, char const *argv[])
{
    ListNode n1 = ListNode(1);
    ListNode n3 = ListNode(3);
    ListNode n5 = ListNode(5);
    ListNode n7 = ListNode(7);
    ListNode n9 = ListNode(9);
    ListNode n11 = ListNode(11);
    ListNode n2 = ListNode(2);
    ListNode n4 = ListNode(4);
    n1.next = &n3;
    n3.next = &n5;
    n5.next = &n7;
    n7.next = &n9;
    n9.next = &n11;
    n2.next = &n4;
    n4.next = &n9;
    Solution a;
    assert(a.getIntersectionNode(&n1, &n2) == &n9);
    
    return 0;
}
