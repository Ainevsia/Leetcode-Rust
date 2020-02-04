fn main() {
    Solution::jump(vec![2,3,4,2,2,5,5,27,5,4,3,2,5,3,2,3,2,5,6,7,32]);
}

struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let nums: Vec<usize> = nums.iter().map(|x| *x as usize).collect();
        let mut position = 0;
        let mut cur_ladder = 0;
        let mut next_ladder = nums[0];
        let mut jump_cnt = 0;
        while position < nums.len() - 1 {
            if position + nums[position] > next_ladder {
                next_ladder = position + nums[position];
            }
            if position == cur_ladder {
                jump_cnt += 1;
                cur_ladder = next_ladder;
                if cur_ladder >= nums.len() - 1 {
                    return jump_cnt
                }
            }
            position += 1;
        }
        0
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