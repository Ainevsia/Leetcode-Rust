fn main() {
    let n = ListNode::new(2);
    let n = ListNode { val: 1, next: Some(Box::new(n)) };
    assert_eq!(Solution::is_palindrome(Some(Box::new(n))), false);
    let n = ListNode::new(1);
    let n = ListNode { val: 2, next: Some(Box::new(n)) };
    let n = ListNode { val: 2, next: Some(Box::new(n)) };
    let n = ListNode { val: 1, next: Some(Box::new(n)) };
    assert_eq!(Solution::is_palindrome(Some(Box::new(n))), true);
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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() { return true }
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();
        loop {
            // fast should not be none
            if let Some(node) = fast.unwrap().next.as_ref() {
                if node.next.is_some() {
                    fast = node.next.as_ref();
                } else { break }
            } else { break }
            slow = slow.unwrap().next.as_ref();
        }
        slow = slow.unwrap().next.as_ref();
        let mut vec = vec![];
        while let Some(node) = slow {
            vec.push(node.val);
            slow = node.next.as_ref();
        }
        println!("{:?}", vec);
        let mut ptr = head.as_ref();
        for &v in vec.iter().rev() {
            if ptr.is_none() || ptr.unwrap().val != v { return false }
            ptr = ptr.unwrap().next.as_ref();
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let n = ListNode::new(2);
        assert_eq!(Solution::is_palindrome(Some(Box::new(n))), true);
        let n = ListNode::new(2);
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        assert_eq!(Solution::is_palindrome(Some(Box::new(n))), false);
        let n = ListNode::new(1);
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        assert_eq!(Solution::is_palindrome(Some(Box::new(n))), true);
        assert_eq!(Solution::is_palindrome(None), true);
    }
}
