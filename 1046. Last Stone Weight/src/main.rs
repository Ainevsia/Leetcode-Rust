fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        let mut stones = BinaryHeap::from(stones);
        while stones.len() >= 2 {
            let x = stones.pop().unwrap();
            let y = stones.pop().unwrap();
            // x is guaranteed to >= y
            if x > y {
                stones.push(x - y);
            }
        }
        if let Some(x) = stones.pop() {
            x
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::last_stone_weight(vec![2,7,4,1,8,1]), 1);
    }
}