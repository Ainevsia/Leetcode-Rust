fn main() {
    Solution::can_partition(vec![1, 5, 11, 5]);
}

struct Solution {}

impl Solution {
    /// cannot understand why this dp is so slow
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = nums.iter().sum::<i32>() as usize;
        if sum % 2 == 1 { return false }
        else { sum /= 2 }
        let mut dp = vec![false; sum + 1];
        dp[0] = true;
        nums.into_iter().fold(dp, |fsum, x| {   // n times
            fsum.into_iter().enumerate().filter(|(_, val)| *val == true )
                .fold(vec![false; sum + 1], |mut dp_next, (i, _)| { // sum + 1 times
                    if i <= sum { dp_next[i] = true }
                    if i + x as usize <= sum { dp_next[i + x as usize] = true }
                    dp_next
            })
        }) [sum]
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
    }
}