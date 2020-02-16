fn main() {
    Solution::merge_k_lists(
        vec![
            Some(Box::new(
                ListNode {
                    val: 1,
                    next: Some(Box::new(
                        ListNode {
                            val: 4,
                            next: Some(Box::new(
                                ListNode {
                                    val: 5,
                                    next: None,
                                }
                            ))
                        }
                    ))
                }
            )),
            Some(Box::new(
                ListNode {
                    val: 1,
                    next: Some(Box::new(
                        ListNode {
                            val: 3,
                            next: Some(Box::new(
                                ListNode {
                                    val: 4,
                                    next: None,
                                }
                            ))
                        }
                    ))
                }
            )),
            Some(Box::new(
                ListNode {
                    val: 2,
                    next: Some(Box::new(
                        ListNode {
                            val: 6,
                            next: None
                        }
                    ))
                }
            )),
        ]
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

use std::collections::BinaryHeap;
use std::cmp::Ordering;


/// implement the PartialOrd and Ord trait reversely to make a min heap
impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl Solution {
    /// this is not written by myself XD, refresh
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Box<ListNode>> = BinaryHeap::new();
        for mut node in lists { // move inside
            if node.is_some() {
                heap.push(node.take()?);
                // ? can be used when returning -> Option<Box<ListNode>>
            }
        }
        // now all useful boxes' ownership are all in the heap
        let mut head = heap.pop()?; // min
        let mut ptr = &mut head;
        while !heap.is_empty() {
            if ptr.next.is_some() {
                heap.push(ptr.next.take()?);
            }
            ptr.next = Some(heap.pop()?);
            ptr = ptr.next.as_mut()?;
        }
        Some(head)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let n1 = ListNode::new(6);
        let n2 = ListNode { val: 5, next: Some(Box::new(n1)) };
        let n3 = ListNode { val: 4, next: Some(Box::new(n2)) };
        let n4 = ListNode { val: 4, next: Some(Box::new(n3)) };
        let n5 = ListNode { val: 3, next: Some(Box::new(n4)) };
        let n6 = ListNode { val: 2, next: Some(Box::new(n5)) };
        let n7 = ListNode { val: 1, next: Some(Box::new(n6)) };
        let n8 = ListNode { val: 1, next: Some(Box::new(n7)) };

        assert_eq!(
            Some(Box::new(n8)),
            Solution::merge_k_lists(
                vec![
                    Some(Box::new(
                        ListNode {
                            val: 1,
                            next: Some(Box::new(
                                ListNode {
                                    val: 4,
                                    next: Some(Box::new(
                                        ListNode {
                                            val: 5,
                                            next: None,
                                        }
                                    ))
                                }
                            ))
                        }
                    )),
                    Some(Box::new(
                        ListNode {
                            val: 1,
                            next: Some(Box::new(
                                ListNode {
                                    val: 3,
                                    next: Some(Box::new(
                                        ListNode {
                                            val: 4,
                                            next: None,
                                        }
                                    ))
                                }
                            ))
                        }
                    )),
                    Some(Box::new(
                        ListNode {
                            val: 2,
                            next: Some(Box::new(
                                ListNode {
                                    val: 6,
                                    next: None
                                }
                            ))
                        }
                    )),
                ]
            )
        )

    }
}