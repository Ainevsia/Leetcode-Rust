fn main() {
    assert_eq!(Solution::merge_two_lists(None, None), None);
    
    let n3 = ListNode::new(4);
    let n2 = ListNode{ val:2, next:Some(Box::new(n3)) };
    let n1 = ListNode{ val:1, next:Some(Box::new(n2)) };
    let l1 = Some(Box::new(n1));

    let m3 = ListNode::new(4);
    let m2 = ListNode{ val:3, next:Some(Box::new(m3)) };
    let m1 = ListNode{ val:1, next:Some(Box::new(m2)) };
    let l2 = Some(Box::new(m1));

    let o6 = ListNode::new(4);
    let o5 = ListNode{ val:4, next:Some(Box::new(o6)) };
    let o4 = ListNode{ val:3, next:Some(Box::new(o5)) };
    let o3 = ListNode{ val:2, next:Some(Box::new(o4)) };
    let o2 = ListNode{ val:1, next:Some(Box::new(o3)) };
    let o1 = ListNode{ val:1, next:Some(Box::new(o2)) };
    let ret = Some(Box::new(o1));

    assert_eq!(
        Solution::merge_two_lists(l1, l2),
        ret
    );
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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>
    {
        // takes owership of each Box to read data and then crate new Boxes accordingly

        match (l1, l2) {    // gets of ownership of T in Option<T>
            (Some(n1), None) => Some(n1),
            (None, Some(n2)) => Some(n2),
            (Some(n1), Some(n2)) => {
                if n1.val < n2.val {
                    Some(n1).map(|mut x| {
                        x.next = Self::merge_two_lists(x.next.take(), Some(n2));
                        x
                    })
                } else {
                    Some(n2).map(|mut x| {
                        x.next = Self::merge_two_lists(Some(n1), x.next.take());
                        x
                    })
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let n3 = ListNode::new(4);
        let n2 = ListNode{ val:2, next:Some(Box::new(n3)) };
        let n1 = ListNode{ val:1, next:Some(Box::new(n2)) };
        let l1 = Some(Box::new(n1));

        let m3 = ListNode::new(4);
        let m2 = ListNode{ val:3, next:Some(Box::new(m3)) };
        let m1 = ListNode{ val:1, next:Some(Box::new(m2)) };
        let l2 = Some(Box::new(m1));

        let o6 = ListNode::new(4);
        let o5 = ListNode{ val:4, next:Some(Box::new(o6)) };
        let o4 = ListNode{ val:3, next:Some(Box::new(o5)) };
        let o3 = ListNode{ val:2, next:Some(Box::new(o4)) };
        let o2 = ListNode{ val:1, next:Some(Box::new(o3)) };
        let o1 = ListNode{ val:1, next:Some(Box::new(o2)) };
        let ret = Some(Box::new(o1));

        assert_eq!(
            Solution::merge_two_lists(l1, l2),
            ret
        );

        assert_eq!(Solution::merge_two_lists(None, None), None);

        let n3 = ListNode::new(4);
        let n2 = ListNode{ val:2, next:Some(Box::new(n3)) };
        let n1 = ListNode{ val:1, next:Some(Box::new(n2)) };
        let l1 = Some(Box::new(n1));

        let m3 = ListNode::new(4);
        let m2 = ListNode{ val:2, next:Some(Box::new(m3)) };
        let m1 = ListNode{ val:1, next:Some(Box::new(m2)) };
        let l2 = Some(Box::new(m1));

        assert_eq!(
            Solution::merge_two_lists(l1, None),
            l2
        );
    }
}