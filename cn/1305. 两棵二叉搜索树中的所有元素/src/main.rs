use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("Hello, world!");
    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b =  Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c =  Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

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
      right: None
    }
  }
}

struct Solution {}

impl Solution {
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res1 = vec![];
        let mut res2 = vec![];
        let mut res = vec![];
        Self::traverse(&root1, &mut res1);
        Self::traverse(&root2, &mut res2);
        let mut i = 0;
        let mut j = 0;
        while i < res1.len() || j < res2.len() {
            if i == res1.len() { res.push(res2[j]); j += 1; }
            else if j == res2.len() { res.push(res1[i]); i += 1; }
            else if res1[i] < res2[j] { res.push(res1[i]); i += 1; }
            else { res.push(res2[j]); j += 1; }
        }
        res
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(x) = root {
            let node = x
                .as_ref()
                .borrow();
            Self::traverse(&node.left, res);
            res.push(node.val);
            Self::traverse(&node.right, res);
        } else { return }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic1() {
        let node1 = TreeNode::new(1);
        let node4 = TreeNode::new(4);
        let mut node2 = TreeNode::new(2);
        node2.left = Some(Rc::new(RefCell::new(node1)));
        node2.right = Some(Rc::new(RefCell::new(node4)));
        let tree1 = Some(Rc::new(RefCell::new(node2)));

        let node1 = TreeNode::new(0);
        let node4 = TreeNode::new(3);
        let mut node2 = TreeNode::new(1);
        node2.left = Some(Rc::new(RefCell::new(node1)));
        node2.right = Some(Rc::new(RefCell::new(node4)));
        let tree2 = Some(Rc::new(RefCell::new(node2)));
        
        assert_eq!(Solution::get_all_elements(tree1,tree2),vec![0,1,1,2,3,4]);
    }

    #[test]
    fn basic2() {
        let node1 = TreeNode::new(8);
        let mut node2 = TreeNode::new(1);
        node2.right = Some(Rc::new(RefCell::new(node1)));
        let tree1 = Some(Rc::new(RefCell::new(node2)));

        let node1 = TreeNode::new(1);
        let mut node2 = TreeNode::new(8);
        node2.left = Some(Rc::new(RefCell::new(node1)));
        let tree2 = Some(Rc::new(RefCell::new(node2)));
        
        assert_eq!(Solution::get_all_elements(tree1,tree2),vec![1,1,8,8]);
    }
}