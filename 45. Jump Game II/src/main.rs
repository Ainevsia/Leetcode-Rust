fn main() {
    Solution::jump(vec![2,3,4,2,2,5,5,27,5,4,3,2,5,3,2,3,2,5,6,7,32]);
}

struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 { return 0 }
        let mut cnt = vec![i32::max_value(); nums.len()];
        cnt[0] = 0;
        for i in 0..nums.len() {
            if i + nums[i] as usize >= nums.len() - 1 {
                return cnt[i] + 1;
            }
            for idx in i+1..=i+nums[i] as usize {
                if cnt[i] + 1 < cnt[idx] {
                    cnt[idx] = cnt[i] + 1;
                }
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