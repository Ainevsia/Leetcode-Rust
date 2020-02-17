fn main() {
    assert_eq!(Solution::rob(None), 0);
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

impl Solution {
    /// awsome solution : 0 ms
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (x, y) = Self::robb(root.as_ref());
        std::cmp::max(x, y)
    }

    /// return value (not robbed , robbed)
    pub fn robb(root: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() { return (0, 0) }
        let root = root.unwrap();
        let val = root.borrow().val;
        let l = Self::robb(root.borrow().left.as_ref());
        let r = Self::robb(root.borrow().right.as_ref());
        (std::cmp::max(l.0, l.1) + std::cmp::max(r.0, r.1), val + l.0 + r.0)
    }
}



#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic1() {
        let r = TreeNode::new(1);
        let r = TreeNode { val: 3, left: None, right: Some(Rc::new(RefCell::new(r))) };
        let l = TreeNode::new(3);
        let l = TreeNode { val: 2, left: None, right: Some(Rc::new(RefCell::new(l))) };
        let n = TreeNode { val: 3,
            left: Some(Rc::new(RefCell::new(l))),
            right: Some(Rc::new(RefCell::new(r))) };
        let n = Some(Rc::new(RefCell::new(n)));
        assert_eq!(Solution::rob(n), 7);
    }

    #[test]
    fn basic2() {
        let r = TreeNode::new(1);
        let r = TreeNode { val: 5, left: None, right: Some(Rc::new(RefCell::new(r))) };
        let rr = TreeNode::new(3);
        let ll = TreeNode::new(1);
        let l = TreeNode { val: 4,
            left: Some(Rc::new(RefCell::new(ll))),
            right: Some(Rc::new(RefCell::new(rr))) };
        let n = TreeNode { val: 3,
            left: Some(Rc::new(RefCell::new(l))),
            right: Some(Rc::new(RefCell::new(r)))
        };
        let n = Some(Rc::new(RefCell::new(n)));
        assert_eq!(Solution::rob(n), 9);
    }
}
