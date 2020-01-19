fn main() {
    Solution::three_sum_closest(vec![-1, 2, 1, -4], 1);
}

struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let mut ret = nums[0] + nums[1] + nums[2];
        let mut diff = i32::abs(ret - target);
        for i in 0..nums.len()-2 {
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let (a, b, c) = (nums[i], nums[l], nums[r]);
                let sum = a + b + c;
                if i32::abs(sum - target) < diff {
                    diff = i32::abs(sum - target);
                    ret = sum;
                }
                if a + b + c < target {
                    l += 1;
                } else if a + b + c > target {
                    r -= 1;
                } else {
                    return target;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(
            Solution::three_sum_closest(vec![-1, 2, 1, -4], 1),
            2
        );
        assert_eq!(
            Solution::three_sum_closest(vec![-1, 2, 1, -4], 2),
            2
        );
        assert_eq!(
            Solution::three_sum_closest(vec![-1, 2, 1, -4], 3),
            3
        );
        assert_eq!(
            Solution::three_sum_closest(vec![1, 1, 1, 1], 5),
            3
        );
        assert_eq!(
            Solution::three_sum_closest(vec![1, 1, 1], 5),
            3
        );
    }
}