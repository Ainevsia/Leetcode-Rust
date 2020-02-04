fn main() {
    Solution::jump(vec![2,3,4,2,2,5,5,27,5,4,3,2,5,3,2,3,2,5,6,7,32]);
}

struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        Self::jump_cnt(&nums)
    }

    pub fn jump_cnt(nums: &[i32]) -> i32 {
        if nums.len() == 1 { return 0 }
        for i in 0..nums.len() {
            if i + nums[i] as usize >= nums.len() - 1 {
                return Self::jump_cnt(&nums[0..i+1]) + 1;
            }
        }
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2);
        assert_eq!(Solution::jump(vec![1,1,1,1,4]), 4);
        assert_eq!(Solution::jump(vec![1000,3,1,1,4]), 1);
        assert_eq!(Solution::jump(vec![2,3,4,2,2,5,5,27,5,4,3,2,5,3,2,3,2,5,6,7,32]), 4);
    }

    #[test]
    fn edge() {
        assert_eq!(Solution::jump(vec![2]), 0);
        assert_eq!(Solution::jump(vec![1,1]), 1);
        
    }
}