fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return 0 }
        let mut sum = 0;
        let mut height = Vec::with_capacity(nums.len() + 1);
        height.push(0);
        for i in nums {
            sum += if i == 0 { -1 } else { 1 };
            height.push(sum);
        }
        let mut max_idx = 1;
        let mut res = 0;
        for i in 0..height.len() {
            if i + 1 > max_idx {
                max_idx = i + 1;
            }
            for j in max_idx..height.len() {
                if height[j] == height[i] && j - i > res {
                    res = j - i;
                    max_idx = j;
                }
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::find_max_length(vec![0,1]), 2);
        assert_eq!(Solution::find_max_length(vec![0,1,1,1,1,0,1,1,1,1]), 2);
    }
}