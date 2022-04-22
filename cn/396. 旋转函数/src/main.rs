fn main() {
    Solution::max_rotate_function(vec![100]);
}

struct Solution {}

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let s = nums.iter().sum::<i32>();
        let mut f = nums.iter().enumerate()
            .map(|(x, y)| { x as i32 * y })
            .sum::<i32>();
        let mut res = f;
        for idx in (1..n as usize).rev() {
            f += s - nums[idx] * n;
            if f > res { res = f; }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn basic() {
        assert_eq!(Solution::max_rotate_function(vec![4,3,2,6]),26);
        assert_eq!(Solution::max_rotate_function(vec![100]),0);
        assert_eq!(Solution::max_rotate_function(vec![4,3,2,6,100,-100,1,100,-99]),789);
    }
}