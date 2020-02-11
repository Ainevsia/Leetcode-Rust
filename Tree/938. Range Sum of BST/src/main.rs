use std::rc::Rc;
use std::cell::RefCell;

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
    pub fn new(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Self {
        TreeNode {
            val, left, right
        }
    }
}

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        let mut sum = 0;
        Self::sum(&root, l, r, &mut sum);
        sum
    }

    pub fn sum(root: &Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32, sum: &mut i32) {
        if let Some(tree_node) = root {
             let tree = tree_node.borrow();
             if tree.val >= l && tree.val <= r {
                 *sum += tree.val;
             }
             Self::sum(&tree.left, l, r, sum);
             Self::sum(&tree.right, l, r, sum);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let node = TreeNode::new(18, None, None);
        let noder = TreeNode::new(15, None, Some(Rc::new(RefCell::new(node))));
        let nodel1 = TreeNode::new(3, None, None);
        let nodel2 = TreeNode::new(7, None, None);
        let nodel = TreeNode::new(5, Some(Rc::new(RefCell::new(nodel1))), Some(Rc::new(RefCell::new(nodel2))));
        let node = TreeNode::new(10, Some(Rc::new(RefCell::new(nodel))), Some(Rc::new(RefCell::new(noder))));
        assert_eq!(Solution::range_sum_bst(Some(Rc::new(RefCell::new(node))), 7, 15), 32);
    }
}