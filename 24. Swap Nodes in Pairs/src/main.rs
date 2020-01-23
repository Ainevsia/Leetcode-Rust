fn main() {
    let n5 = ListNode::new(5);
    let n4 = ListNode { val: 4, next: Some(Box::new(n5)), };
    let n3 = ListNode { val: 3, next: Some(Box::new(n4)), };
    let n2 = ListNode { val: 2, next: Some(Box::new(n3)), };
    let n1 = ListNode { val: 1, next: Some(Box::new(n2)), };
    let head1 = Some(Box::new(n1));
    Solution::swap_pairs(head1);
    Solution::swap_pairs(None);

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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vec = vec![];
        let mut even = true;
        while let Some(node) = head {
            even = !even;
            if even {
                let temp = vec.pop()?;
                vec.push(node.val);
                vec.push(temp);
            } else {
                vec.push(node.val);
            }
            head = node.next;
        }
        // println!("vec = {:#?}", vec);
        let mut head = None;
        while let Some(val) = vec.pop() {
            head = Some(Box::new(ListNode { val, next: head, }))
        }
        // println!("head = {:#?}", head);
        head
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let n4 = ListNode::new(4);
        let n3 = ListNode { val: 3, next: Some(Box::new(n4)), };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)), };
        let n1 = ListNode { val: 1, next: Some(Box::new(n2)), };
        let head1 = Some(Box::new(n1));
        let n4 = ListNode::new(3);
        let n3 = ListNode { val: 4, next: Some(Box::new(n4)), };
        let n2 = ListNode { val: 1, next: Some(Box::new(n3)), };
        let n1 = ListNode { val: 2, next: Some(Box::new(n2)), };
        let head2 = Some(Box::new(n1));
        assert_eq!(Solution::swap_pairs(head1), head2);
    }
}