fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

impl Solution {
    // bfs, use queue
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        if let Some(rc) = root { q.push_back(rc.clone()) }
        else { return depth }
        while !q.is_empty() {
            depth += 1;
            for _ in 0..q.len() {
                let rc = q.pop_front().unwrap();
                if rc.borrow().left.is_some() {
                    q.push_back(rc.borrow().left.as_ref().unwrap().clone());
                }
                if rc.borrow().right.is_some() {
                    q.push_back(rc.borrow().right.as_ref().unwrap().clone());
                }
                if rc.borrow().left.is_none() && rc.borrow().right.is_none() {
                    return depth
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let l = TreeNode::new(15);
        let r = TreeNode::new(7);
        let r = TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(l))),
            right: Some(Rc::new(RefCell::new(r))),
        };
        let l = TreeNode::new(9);
        let n = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(l))),
            right: Some(Rc::new(RefCell::new(r))),
        };
        let n = Some(Rc::new(RefCell::new(n)));
        assert_eq!(Solution::min_depth(n), 2);
        assert_eq!(Solution::min_depth(None), 0);
           
    }
}
