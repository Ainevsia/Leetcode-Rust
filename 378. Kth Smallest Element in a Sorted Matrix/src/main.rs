fn main() {
    assert_eq!(Solution::kth_smallest(vec![
        vec![ 1,  5,  9],
        vec![10, 11, 13],
        vec![12, 13, 15]
     ], 8), 13);
}

struct Solution {}

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn kth_smallest(mut matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut min_heap = BinaryHeap::new();
        let l = matrix.len();
        for _ in (0..l).rev() {
            min_heap.push(Reverse(matrix.pop().unwrap()));
        }
        let mut cnt = 0;
        let mut res = 0;
        while cnt < k as usize {
            let mut x = min_heap.pop().unwrap().0;
            res = x.remove(0);
            if !x.is_empty() { min_heap.push(Reverse(x)) }
            cnt += 1
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::kth_smallest(vec![
            vec![ 1,  5,  9],
            vec![10, 11, 13],
            vec![12, 13, 15]
         ], 8), 13);
         assert_eq!(Solution::kth_smallest(vec![
            vec![ 1,  5,  9],
            vec![10, 11, 13],
            vec![12, 13, 15]
         ], 6), 12);
         assert_eq!(Solution::kth_smallest(vec![
            vec![ 1,  5,  9],
            vec![10, 11, 13],
            vec![12, 13, 15]
         ], 7), 13);
         assert_eq!(Solution::kth_smallest(vec![
            vec![ 1,  5,  9],
            vec![10, 11, 13],
            vec![12, 13, 15]
         ], 9), 15);
         assert_eq!(Solution::kth_smallest(vec![
            vec![ 1],
         ], 1), 1);
    }
}