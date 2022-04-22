fn main() {
    Solution::max_rotate_function(vec![100]);
}

struct Solution {}

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        // use std::iter::FromIterator;
        let mut arr = Vec::from_iter(0..n);
        let mut res = i32::MIN;
        for _ in 0..n {
            let f = nums.iter().zip(arr.iter())
                .map(|(x, y)| { x * y })
                .sum::<i32>();
            if f > res { res = f; }
            arr.rotate_right(1);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn basic() {
        assert_eq!(Solution::max_rotate_function(vec![4,3,2,6]),26);
        assert_eq!(Solution::max_rotate_function(vec![100]),0);
        assert_eq!(Solution::max_rotate_function(vec![4,3,2,6,100,-100,1,100,-99]),789);
    }
}