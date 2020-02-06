fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut sum = 0;
        for d in nums {
            sum += d;
            if sum > max_sum {
                max_sum = sum;
            }
            if sum < 0 {
                sum = 0;
            }
        }
        max_sum
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
        assert_eq!(Solution::max_sub_array(vec![-2,1,5,4,-8,2,1,-5,4]), 10);
        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
        assert_eq!(Solution::max_sub_array(vec![-2, -1]), -1);

    }
}