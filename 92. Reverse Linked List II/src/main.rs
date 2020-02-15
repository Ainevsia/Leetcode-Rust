fn main() {
    let n = ListNode::new(5);
    let n = ListNode { val: 4, next: Some(Box::new(n)) };
    let n = ListNode { val: 3, next: Some(Box::new(n)) };
    let n = ListNode { val: 2, next: Some(Box::new(n)) };
    let n = ListNode { val: 1, next: Some(Box::new(n)) };
    let head = Some(Box::new(n));
    let n = ListNode::new(5);
    let n = ListNode { val: 2, next: Some(Box::new(n)) };
    let n = ListNode { val: 3, next: Some(Box::new(n)) };
    let n = ListNode { val: 4, next: Some(Box::new(n)) };
    let n = ListNode { val: 1, next: Some(Box::new(n)) };
    let head2 = Some(Box::new(n));
    assert_eq!(Solution::reverse_between(head, 2, 4), head2);
}

struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        if m == n { return head }
        let mut cnt = 1;
        let mut header_node = Some(Box::new(ListNode{ val: 0, next: head }));
        let mut ptr = header_node.as_mut().unwrap();
        while cnt < m { ptr = ptr.next.as_mut().unwrap(); cnt += 1}
        let mut tail = ptr.next.take();
        let mut prev = None;
        if let Some(mut node) = tail {
            prev = node.next.take();
            tail = Some(node);
        }
        while let Some(mut node) = prev {
            prev = node.next.take();
            node.next = tail;
            tail = Some(node);
            cnt += 1;
            if cnt == n { break }
        }
        ptr.next = tail;
        while ptr.next.is_some() { ptr = ptr.next.as_mut().unwrap(); }
        ptr.next = prev;
        header_node.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic1() {
        let n = ListNode::new(5);
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        let head = Some(Box::new(n));
        let n = ListNode::new(5);
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        let head2 = Some(Box::new(n));
        assert_eq!(Solution::reverse_between(head, 2, 4), head2);
    }

    #[test]
    fn basic2() {
        let n = ListNode::new(5);
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        let head = Some(Box::new(n));
        let n = ListNode::new(1);
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 5, next: Some(Box::new(n)) };
        let head2 = Some(Box::new(n));
        assert_eq!(Solution::reverse_between(head, 1, 5), head2);
    }

    #[test]
    fn basic3() {
        let n = ListNode::new(5);
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        let head = Some(Box::new(n));
        let n = ListNode::new(5);
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        let head2 = Some(Box::new(n));
        assert_eq!(Solution::reverse_between(head, 3, 4), head2);
    }

    #[test]
    fn edge() {
        let n = ListNode::new(5);
        let head = Some(Box::new(n));
        let n = ListNode::new(5);
        let head2 = Some(Box::new(n));
        assert_eq!(Solution::reverse_between(head, 1, 1), head2);
    }
}
