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
    /// false thought : only find the larget in a single path
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0 }
        let root = root.unwrap();
        let mut r = Robber::new(root.clone());
        r.search();
        r.get_gain()
    }
}

struct Robber {
    buf: Vec<Rc<RefCell<TreeNode>>>,
    pre1: i32,
    pre2: i32,
    gain: i32,
}

impl Robber {
    pub fn new(root: Rc<RefCell<TreeNode>>) -> Robber {
        Robber {
            buf: vec![root],
            pre1: 0,
            pre2: 0,
            gain: 0,
        }
    }
    
    pub fn search(&mut self) {
        if let Some(x) = self.buf.pop() {
            let gain = std::cmp::max(self.pre1, self.pre2 + x.borrow().val);
            println!("x.borrow().val = {:#?}", x.borrow().val);
            println!("self.pre1 = {:#?}, self.pre2 = {:#?}", self.pre1, self.pre2);
            if gain > self.gain { self.gain = gain }
            let tmp = self.pre2;
            self.pre2 = self.pre1;
            self.pre1 = gain;
            if let Some(y) = x.borrow().left.as_ref() {
                self.buf.push(y.clone());
                self.search();
            }
            if let Some(y) = x.borrow().right.as_ref() {
                self.buf.push(y.clone());
                self.search();
            }
            self.pre1 = self.pre2;
            self.pre2 = tmp;
        }
    }

    pub fn get_gain(&self) -> i32 {
        self.gain
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
