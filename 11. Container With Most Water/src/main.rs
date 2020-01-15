fn main() {
    println!("Hello, world!");
    Solution::max_area(vec![1,2,3]);
}

struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut area_max = 0;
        for window_len in 1..n {
            let mut area = 0;
            for i in 0..n - window_len {
                let min_height = if height[i] > height[i + window_len] {
                    height[i + window_len]
                } else {
                    height[i]
                };
                area = if min_height * (window_len as i32) > area {
                    min_height * (window_len as i32)
                } else {
                    area
                };
            }
            area_max =  if area_max < area {
                area
            } else {
                area_max
            };
        }
        area_max
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