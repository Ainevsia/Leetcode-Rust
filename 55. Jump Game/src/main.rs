fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut long = 0;
        let nums: Vec<usize> = nums.into_iter().map(|x| x as usize).collect();
        for i in 0..nums.len() {
            if i + nums[i] > long { long = i + nums[i] }
            if long >= nums.len() - 1 { return true }
            if nums[i] == 0 && i == long { return false }
        }
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::can_jump(vec![2,3,1,1,4]), true);
        assert_eq!(Solution::can_jump(vec![3,2,1,0,4]), false);
        assert_eq!(Solution::can_jump(vec![0]), true);

    }
}