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
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut min_heap = BinaryHeap::new();
        let l = matrix.len();
        let mut idxs = vec![0usize; l];
        for i in 0..l {
            min_heap.push(Reverse(matrix[i][idxs[i]]));
        }
        let mut cnt = 0;
        let mut res = 0;
        while cnt < k as usize{
            res = min_heap.pop().unwrap().0;
            for i in 0..l {
                if matrix[i][idxs[i]] == res {
                    if idxs[i] == l - 1 {
                        let mut largest = i32::max_value();
                        let mut targeti = 0;
                        for j in 0..l {
                            if idxs[j] < l - 1 && matrix[j][idxs[j]] < largest {
                                largest = matrix[j][idxs[j]];
                                targeti = j;
                            }
                        }
                        if largest != i32::max_value() {
                            idxs[targeti] += 1;
                            min_heap.push(Reverse(matrix[targeti][idxs[targeti]]))
                        }
                    } else {
                        idxs[i] += 1;
                        min_heap.push(Reverse(matrix[i][idxs[i]]));
                    }
                }
            }
            cnt += 1;
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