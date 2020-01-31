fn main() {
    Solution::first_missing_positive(vec![3,4,-1,1]);
}

struct Solution {}

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        nums = nums.into_iter().filter(|&x| x > 0).collect();
        if nums.is_empty() { return 1 }
        let k = nums.len();
        // ret should within [1..k+1]
        for i in 0..k {
            let n = i32::abs(nums[i]) as usize;
            if n >= k + 1 { continue }
            // 1<=n<=k k numbers
            if nums[n-1] > 0 { nums[n-1] = - nums[n-1] }
        }
        for i in 0..k {
            if nums[i] > 0 { return (i + 1) as i32 }
        }
        (k + 1) as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::first_missing_positive(vec![1,2,0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3,4,-1,1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7,8,9,11,12]), 1);

    }
}