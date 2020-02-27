fn main() {
    let n = ListNode::new(5);
    let n = ListNode { val: 4, next: Some(Box::new(n)) };
    let n = ListNode { val: 4, next: Some(Box::new(n)) };
    let n = ListNode { val: 3, next: Some(Box::new(n)) };
    let n = ListNode { val: 3, next: Some(Box::new(n)) };
    let n = ListNode { val: 2, next: Some(Box::new(n)) };
    let n = ListNode { val: 1, next: Some(Box::new(n)) };
    let head1 = Some(Box::new(n));
    let n = ListNode::new(5);
    let n = ListNode { val: 2, next: Some(Box::new(n)) };
    let n = ListNode { val: 1, next: Some(Box::new(n)) };
    let head2 = Some(Box::new(n));
    assert_eq!(Solution::delete_duplicates(head1), head2);
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
    /// [not implemented] use c++
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let newhead = None;
        
        newhead
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let n = ListNode::new(5);
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        let head1 = Some(Box::new(n));
        let n = ListNode::new(5);
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        let head2 = Some(Box::new(n));
        assert_eq!(Solution::delete_duplicates(head1), head2);
    }
}