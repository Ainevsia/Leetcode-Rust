fn main() {
    let n5 = ListNode { val: 5, next: None };
    let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
    let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
    let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
    let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
    let input = Some(Box::new(n1));
    Solution::rotate_right(input, 2);
    Solution::rotate_right(Some(Box::new(ListNode { val: 1, next: None })), 1);
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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k <= 0 { return head }
        let mut ptr = head.as_ref();
        let mut list_len = 0;
        while let Some(node) = ptr {
            ptr = node.next.as_ref();
            list_len += 1;
        }
        let cutoff_cnt = list_len - k % list_len;
        if cutoff_cnt == list_len { return head }
        println!("list_len = {:#?}", list_len);
        println!("cutoff_cnt = {:#?}", cutoff_cnt);
        let mut ptr = head.as_mut().unwrap();
        let mut i = 1;
        while i < cutoff_cnt {
            ptr = ptr.next.as_mut().unwrap();
            i += 1;
        }
        let mut new_head = ptr.next.take();
        println!("new_head = {:#?}", new_head);
        println!("head = {:#?}", head);
        let mut ptr = new_head.as_mut();
        while let Some(node) = ptr {
            if node.next.is_none() { ptr = Some(node); break }
            ptr = node.next.as_mut();
        }
        println!("ptr = {:#?}", ptr);
        ptr.unwrap().next = head;
        println!("new_head = {:#?}", new_head);
        new_head
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic1() {
        let n5 = ListNode { val: 5, next: None };
        let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
        let input = Some(Box::new(n1));

        let n5 = ListNode { val: 3, next: None };
        let n4 = ListNode { val: 2, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 1, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 5, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 4, next: Some(Box::new(n2)) };
        let output = Some(Box::new(n1));

        assert_eq!(
            Solution::rotate_right(input, 2),
            output
        )
    }

    #[test]
    fn basic2() {
        let n3 = ListNode { val: 2, next: None };
        let n2 = ListNode { val: 1, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 0, next: Some(Box::new(n2)) };
        let input = Some(Box::new(n1));

        let n3 = ListNode { val: 1, next: None };
        let n2 = ListNode { val: 0, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 2, next: Some(Box::new(n2)) };
        let output = Some(Box::new(n1));

        assert_eq!(
            Solution::rotate_right(input, 4),
            output
        )
    }

    #[test]
    fn edge() {
        assert_eq!(
            Solution::rotate_right(None, 4),
            None
        );
        assert_eq!(
            Solution::rotate_right(Some(Box::new(ListNode { val: 1, next: None })), 1),
            Some(Box::new(ListNode { val: 1, next: None }))
        )
    }
}