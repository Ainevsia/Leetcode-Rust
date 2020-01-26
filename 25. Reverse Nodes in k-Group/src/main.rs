fn main() {
    let n5 = ListNode::new(5);
    let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
    let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
    let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
    let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
    let head1 = Some(Box::new(n1));
    let n5 = ListNode::new(5);
    let n4 = ListNode { val: 3, next: Some(Box::new(n5)) };
    let n3 = ListNode { val: 4, next: Some(Box::new(n4)) };
    let n2 = ListNode { val: 1, next: Some(Box::new(n3)) };
    let n1 = ListNode { val: 2, next: Some(Box::new(n2)) };
    let head2 = Some(Box::new(n1));
    Solution::reverse_k_group(head1, 2);
}

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

struct Solution {}

impl Solution {
    /// 0 < k <= len, list is not empty
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() { return head }
        let mut buf: Vec<Box<ListNode>> = Vec::with_capacity(k as usize);
        let mut nodeptrd = head;
        for _ in 0..k {
            if let Some(mut node) = nodeptrd {
                nodeptrd = node.next.take();
                buf.push(node);
            } else {
                break
            }
        }
        let end = if buf.len() < k as usize { true } else { false };
        if end {
            // in the origin order
            buf = buf.into_iter().rev().collect();
        }
        let mut head = Some(buf.pop().unwrap());
        let mut nodeptr = head.as_mut().unwrap();
        // construct the list from 1 to n
        while let Some(node) = buf.pop() {
            nodeptr.next = Some(node);
            nodeptr = nodeptr.next.as_mut().unwrap();
        }
        if nodeptrd.is_none() { return head }
        loop {
            for _ in 0..k {
                if let Some(mut node) = nodeptrd {
                    nodeptrd = node.next.take();
                    buf.push(node);
                } else {
                    break
                }
                // println!("buf = {:#?}", buf);
            }
            let end = if buf.len() < k as usize { true } else { false };
            if end {
                // in the origin order
                buf = buf.into_iter().rev().collect();
            }
            while let Some(node) = buf.pop() {
                nodeptr.next = Some(node);
                nodeptr = nodeptr.next.as_mut().unwrap();
            }
            if end { break }
        }
        // println!("head = {:#?}", head);
        head
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic1() {
        let n5 = ListNode::new(5);
        let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
        let head1 = Some(Box::new(n1));
        let n5 = ListNode::new(1);
        let n4 = ListNode { val: 2, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 4, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 5, next: Some(Box::new(n2)) };
        let head2 = Some(Box::new(n1));
        assert_eq!(
            Solution::reverse_k_group(head1, 5),
            head2
        );
    }

    #[test]
    fn basic2() {
        let n5 = ListNode::new(5);
        let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
        let head1 = Some(Box::new(n1));
        let n5 = ListNode::new(5);
        let n4 = ListNode { val: 3, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 4, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 1, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 2, next: Some(Box::new(n2)) };
        let head2 = Some(Box::new(n1));
        assert_eq!(
            Solution::reverse_k_group(head1, 2),
            head2
        );
    }

    #[test]
    fn basic3() {
        let n5 = ListNode::new(5);
        let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
        let head1 = Some(Box::new(n1));
        let n5 = ListNode::new(5);
        let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 1, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 3, next: Some(Box::new(n2)) };
        let head2 = Some(Box::new(n1));
        assert_eq!(
            Solution::reverse_k_group(head1, 3),
            head2
        );
    }

    #[test]
    fn none() {
        assert_eq!(
            Solution::reverse_k_group(None, 1),
            None
        );
    }

    #[test]
    fn name() {
        let n5 = ListNode::new(5);
        let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
        let head1 = Some(Box::new(n1));
        let n5 = ListNode::new(5);
        let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
        let head2 = Some(Box::new(n1));
        assert_eq!(
            Solution::reverse_k_group(head1, 123),
            head2
        );
    }
}