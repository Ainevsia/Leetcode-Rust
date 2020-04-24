fn main() {
    assert_eq!(Solution::product_except_self(vec![1,2,3,4]), [24,12,8,6]);
}

struct Solution {}

impl Solution {
    /// use the output vector to achieve O(n) space
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1];
        let mut x = 1;
        for i in 0..nums.len() - 1 {
            x *= nums[i];
            res.push(x);
        }
        let mut x = 1;
        for i in (0..nums.len()).rev() {
            res[i] = res[i] * x;
            x *= nums[i];
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::product_except_self(vec![1,2,3,4]), [24,12,8,6]);
    }
}