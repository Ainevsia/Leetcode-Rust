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

enum Proceed { L, R }

impl Solution {
    /// two stack BFS zigzag traverse
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut slf: Vec<Rc<RefCell<TreeNode>>> = Vec::new();   // stack left child first
        let mut srf: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut res = vec![];
        if let Some(rc) = root { slf.push(rc) }
        else { return res }
        let mut proceed = Some(Proceed::L);
        while let Some(direction) = proceed {
            res.push(vec![]);
            let last = res.len() - 1;
            match direction {
                Proceed::L => {
                    while !slf.is_empty() {
                        let rc = slf.pop().unwrap();
                        res[last].push(rc.borrow().val);
                        if rc.borrow().left.is_some() {
                            srf.push(rc.borrow().left.as_ref().unwrap().clone());
                        }
                        if rc.borrow().right.is_some() {
                            srf.push(rc.borrow().right.as_ref().unwrap().clone());
                        }
                    }
                    proceed = Some(Proceed::R);
                    if srf.is_empty() { proceed = None }
                }
                Proceed::R => {
                    while !srf.is_empty() {
                        let rc = srf.pop().unwrap();
                        res[last].push(rc.borrow().val);
                        if rc.borrow().right.is_some() {
                            slf.push(rc.borrow().right.as_ref().unwrap().clone());
                        }
                        if rc.borrow().left.is_some() {
                            slf.push(rc.borrow().left.as_ref().unwrap().clone());
                        }
                    }
                    proceed = Some(Proceed::L);
                    if slf.is_empty() { proceed = None }
                }
            }
        }
        res
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
        assert_eq!(Solution::zigzag_level_order(n),
        vec![
            vec![3],
            vec![20,9],
            vec![15,7]
          ]);
    }
}
