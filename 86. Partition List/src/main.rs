fn main() {
    //  head = 1->4->3->2->5->2, x = 3
    let n = ListNode::new(2);
    let n = ListNode { val: 5, next: Some(Box::new(n)) };
    let n = ListNode { val: 2, next: Some(Box::new(n)) };
    let n = ListNode { val: 3, next: Some(Box::new(n)) };
    let n = ListNode { val: 4, next: Some(Box::new(n)) };
    let n = ListNode { val: 1, next: Some(Box::new(n)) };
    let head1 = Some(Box::new(n));
    // 1->2->2->4->3->5
    let n = ListNode::new(5);
    let n = ListNode { val: 3, next: Some(Box::new(n)) };
    let n = ListNode { val: 4, next: Some(Box::new(n)) };
    let n = ListNode { val: 2, next: Some(Box::new(n)) };
    let n = ListNode { val: 2, next: Some(Box::new(n)) };
    let n = ListNode { val: 1, next: Some(Box::new(n)) };
    let head2 = Some(Box::new(n));
    assert_eq!(Solution::partition(head1, 3), head2);
}

struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut h1 = ListNode::new(0);
        let mut h2 = ListNode::new(0);
        let mut p1 = &mut h1;
        let mut p2 = &mut h2;
        while head.is_some() {
            let mut node = head.unwrap();
            head = node.next.take();
            if node.val < x {
                p1.next = Some(node);
                p1 = p1.next.as_mut().unwrap();
            } else {
                p2.next = Some(node);
                p2 = p2.next.as_mut().unwrap();
            }
        }
        p1.next = h2.next;
        h1.next
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        //  head = 1->4->3->2->5->2, x = 3
        let n = ListNode::new(2);
        let n = ListNode { val: 5, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        let head1 = Some(Box::new(n));
        // 1->2->2->4->3->5
        let n = ListNode::new(5);
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        let head2 = Some(Box::new(n));
        assert_eq!(Solution::partition(head1, 3), head2);
    }
}