fn main() {
    assert_eq!(Solution::find_min_height_trees(4, vec![vec![1,0],vec![1,2],vec![1,3]]),vec![1]);
}

struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n < 2 { return vec![0] }
        let mut node_exist = vec![true; n as usize];
        let mut in_degrees = vec![0; n as usize];
        let mut edges_relate = vec![vec![]; n as usize];
        for e in edges {
            in_degrees[e[0] as usize] += 1;
            in_degrees[e[1] as usize] += 1;
            edges_relate[e[0] as usize].push(e[1] as usize);
            edges_relate[e[1] as usize].push(e[0] as usize);
        }
        let mut q = VecDeque::new();
        let mut remain = n as usize;
        for i in 0..n as usize {
            if in_degrees[i] == 1 { q.push_back(i) }
        }
        while remain > q.len() {
            // q should not be empty
            for _ in 0..q.len() {
                let leaf = q.pop_front().unwrap();
                node_exist[leaf] = false;
                remain -= 1;
                for &i in edges_relate[leaf].iter() {
                    if node_exist[i] {
                        in_degrees[i] -= 1;
                        if in_degrees[i] == 1 { q.push_back(i) }
                    }
                }
            }
        }
        q.into_iter().map(|x| x as i32).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::find_min_height_trees(4, vec![vec![1,0],vec![1,2],vec![1,3]]),vec![1]);
        assert_eq!(Solution::find_min_height_trees(6, vec![vec![0,3],vec![1,3],vec![2,3],vec![4,3],vec![5,4]]),vec![3,4]);
    }

    #[test]
    fn edge() {
        assert_eq!(Solution::find_min_height_trees(1, vec![vec![]]), vec![0]);
    }
}