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

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        if root.is_none() { return vec![] }
        let mut bfs = PathSum::new(sum);
        bfs.traverse(root.unwrap());
        bfs.get_res()
    }
}

struct PathSum {
    res: Vec<Vec<i32>>,
    buf: Vec<i32>,
    sum: i32,
}

impl PathSum {
    pub fn new(sum: i32) -> PathSum {
        PathSum { res: vec![], buf: vec![], sum }
    }

    pub fn get_res(&self) -> Vec<Vec<i32>> { self.res.clone() }
    
    pub fn traverse(&mut self, root: Rc<RefCell<TreeNode>>) {
        self.buf.push(root.borrow().val);
        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            if self.buf.iter().sum::<i32>() == self.sum {
                self.res.push(self.buf.clone());
            }
        }
        if root.borrow().left.is_some() {
            self.traverse(root.borrow().left.as_ref().unwrap().clone());
        }
        if root.borrow().right.is_some() {
            self.traverse(root.borrow().right.as_ref().unwrap().clone());
        }
        self.buf.pop();
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let l = TreeNode::new(5);
        let r = TreeNode::new(1);
        let r = TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(l))),
            right: Some(Rc::new(RefCell::new(r))),
        };
        let l = TreeNode::new(13);
        let rr = TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(l))),
            right: Some(Rc::new(RefCell::new(r))),
        };
        let l = TreeNode::new(7);
        let r = TreeNode::new(2);
        let l = TreeNode {
            val: 11,
            left: Some(Rc::new(RefCell::new(l))),
            right: Some(Rc::new(RefCell::new(r))),
        };
        let ll = TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(l))),
            right: None,
        };
        let n = TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(ll))),
            right: Some(Rc::new(RefCell::new(rr))),
        };
        let n = Some(Rc::new(RefCell::new(n)));
        assert_eq!(Solution::path_sum(n, 22), vec![
            vec![5,4,11,2],
            vec![5,8,4,5]]);
    }
}
