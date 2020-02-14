fn main() {
    assert_eq!(Solution::find_duplicate(vec![1,3,4,2,2]), 2);
}

struct Solution {}

impl Solution {
    /// 循环置换 fast/slow pointers find the entry point of the loop
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[0] as usize;
        let mut fast = nums[slow] as usize;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
            fast = nums[fast] as usize;
        }
        let mut ptr = 0usize;
        while nums[ptr] != nums [fast] {
            fast = nums[fast] as usize;
            ptr = nums[ptr] as usize;
        }
        nums[ptr]
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::find_duplicate(vec![1,3,4,2,2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3,1,3,4,2]), 3);
        assert_eq!(Solution::find_duplicate(vec![2,2,2]), 2);
        assert_eq!(Solution::find_duplicate(vec![2,5,9,6,9,3,8,9,7,1]), 9);

    }
}