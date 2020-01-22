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
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => {*v = 1},
        None => {},
    }
    assert_eq!(x, Some(1));
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut node_vals: Vec<i32> = lists.iter().map(Self::node2vec).flatten().collect();
        node_vals.sort();
        Self::vec2node(&node_vals)
    }

    fn node2vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = vec![];
        let mut nodeptr = head;
        while let Some(node) = nodeptr {
            vec.push(node.val);
            nodeptr = &node.next;
        }
        vec
    }

    fn vec2node(vals: &[i32]) -> Option<Box<ListNode>> {
        if vals.is_empty() { return None }
        let mut nodeptr = ListNode::new(*vals.last().unwrap());
        for i in (0..vals.len()-1).rev() {
            nodeptr = ListNode { val: vals[i], next: Some(Box::new(nodeptr))}
        }
        Some(Box::new(nodeptr))
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