fn main() {
    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}

struct Solution {}

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in nums {
            map.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        for (num, freq) in map {
            if min_heap.len() < k as usize {
                min_heap.push(Reverse((freq, num)));
            } else {
                let cnt = min_heap.peek().unwrap().0 .0;
                if freq > cnt {
                    min_heap.pop();
                    min_heap.push(Reverse((freq, num)));
                }
            }
        }
        let mut res = vec![];
        while !min_heap.is_empty() {
            res.insert(0, min_heap.pop().unwrap().0 .1);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::top_k_frequent(vec![1,1,1,2,2,3], 2), vec![1,2]);
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
        assert_eq!(Solution::top_k_frequent(vec![1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,3,3], 2), vec![1,2]);
        
    }
}