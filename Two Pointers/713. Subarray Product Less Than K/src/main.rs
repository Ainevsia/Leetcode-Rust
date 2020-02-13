fn main() {
    Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100);
    println!("i32::max_value() = {:#?}", i32::max_value()*i32::max_value());
}

struct Solution {}

impl Solution {
    /// sliding window
    /// nums.len() > 0
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let (mut start, mut end) = (0, 0);
        let mut product = 1; 
        let mut res = 0;
        while end < nums.len() {
            product *= nums[end];
            if product < k {
                res += end - start + 1;
            } else {
                while product >= k && start <= end {
                    product /= nums[start];
                    start += 1;
                }
                // now product < k 
                res += end + 1 - start; // won't underflow 
            }
            end += 1;
            if end == nums.len() { break }
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100), 8);
        assert_eq!(Solution::num_subarray_product_less_than_k(vec![1], 100), 1);
        assert_eq!(Solution::num_subarray_product_less_than_k(vec![2], 1), 0);
    }
}
