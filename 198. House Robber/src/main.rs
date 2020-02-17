fn main() {
    assert_eq!(Solution::rob(vec![1,2,3,1]), 4);
}

struct Solution {}

impl Solution {

    // 1 dp
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0 }
        if nums.len() <= 1 { return nums[0] }
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = std::cmp::max(nums[0], nums[1]);
        for i in 2..nums.len() {
            dp[i] = std::cmp::max(dp[i-1], dp[i-2] + nums[i]);
        }
        dp[nums.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::rob(vec![1,2,3,1]), 4);
        assert_eq!(Solution::rob(vec![2,7,9,3,1]), 12);
        assert_eq!(Solution::rob(vec![4]), 4);
    }
}