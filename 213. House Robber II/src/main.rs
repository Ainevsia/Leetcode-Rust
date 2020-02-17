fn main() {
    assert_eq!(Solution::rob(vec![2,3,2]), 3);
}

struct Solution {}

impl Solution {
    /// 1 dp * 2 times
    /// equals to this question: the final result will only be one of the two following
    /// situations:
    ///     - first is not selected, then freely select in the following
    ///     - last  is not selected, then freely select in the following
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 1 { return 0 }
        if nums.len() == 1 { return nums[0] }
        if nums.len() == 2 { return std::cmp::max(nums[0], nums[1]) }
        let mut dp = vec![0; nums.len()];
        let last = nums.pop().unwrap();
        dp[0] = nums[0];
        dp[1] = std::cmp::max(nums[0], nums[1]);
        for i in 2..nums.len() {
            dp[i] = std::cmp::max(dp[i-1], dp[i-2] + nums[i]);
        }
        let res = dp[nums.len() - 1];
        nums.push(last);
        nums.remove(0);
        dp[0] = nums[0];
        dp[1] = std::cmp::max(nums[0], nums[1]);
        for i in 2..nums.len() {
            dp[i] = std::cmp::max(dp[i-1], dp[i-2] + nums[i]);
        }
        std::cmp::max(res, dp[nums.len() - 1])
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::rob(vec![2,3,2]), 3);
        assert_eq!(Solution::rob(vec![1,2,3,1]), 4);
    }

    #[test]
    fn fail() {
        assert_eq!(Solution::rob(vec![1,2,1,1]), 3);
    }
}