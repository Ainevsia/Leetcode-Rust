fn main() {
    Solution::search_insert(vec![1,3,5,6], 5);
}

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for (i, x) in nums.iter().enumerate() {
            if *x >= target {
                return i as i32
            }
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 0), 0);

    }
}