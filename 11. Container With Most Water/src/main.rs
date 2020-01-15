fn main() {
    println!("Hello, world!");
    Solution::max_area(vec![1,2,3]);
}

struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp;
        let (mut l, mut r) = (0, height.len() - 1);
        let mut area = 0;
        while l < r {
            area = cmp::max(area, cmp::min(height[l], height[r]) * (r - l) as i32);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        area
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
        // assert_eq!(Solution::max_area(vec![1,2,3,4,5,6,7,8,9]), 8);

    }
}