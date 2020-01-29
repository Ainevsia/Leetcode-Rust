fn main() {
    Solution::search_range(vec![5,7,7,8,8,10], 8);
}

struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() { return vec![-1,-1] }
        let mut s = 0;
        let mut e = nums.len() - 1;
        let mut m = (s + e) / 2;
        loop {
            if s > e { break vec![-1,-1] }
            if s == e { break if nums[s] == target { vec![s as i32,s as i32] } else { vec![-1,-1] } }
            if s + 1 == e {
                if nums[s] == target && nums[e] == target {
                    break vec![s as i32,e as i32]
                } else if nums[s] == target {
                    break vec![s as i32,s as i32]
                } else if nums[e] == target {
                    break vec![e as i32,e as i32]
                } else {
                    break vec![-1,-1]
                }
            }
            // now s < m < e
            if nums[s] == nums[e] { break if nums[s] == target { vec![s as i32, e as i32] } else { vec![-1,-1] } }
            println!("s = {:#?}, e = {:#?}", s, e);
            println!("m = {:#?}", m);
            if nums[m] < target { s = m; m = (s + e) / 2; }
            else if nums[m] > target { e = m; m = (s + e) / 2; }
            else {
                if nums[e] > target {
                    if nums[m] == target && nums[m + 1] > target {
                        e = m
                    } else {
                        m = (m + e) / 2
                    }
                } else if nums[s] < target {
                    if nums[m] == target && nums[m - 1] < target {
                        s = m
                    } else {
                        m = (s + m) / 2
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    // #[test]
    fn basic() {
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 8), vec![3,4]);
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 6), vec![-1,-1]);
        assert_eq!(Solution::search_range(vec![7,7,7,7,7,7], 7), vec![0,5]);
        assert_eq!(Solution::search_range(vec![7,7,7,7,7,7], 6), vec![-1,-1]);
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 7), vec![1,2]);
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 10), vec![5,5]);
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 5), vec![0,0]);
        // assert_eq!(Solution::search_range(vec![7,7,7,7,7,7], 6), vec![-1,-1]);

    }

    #[test]
    fn leetcode() {
        assert_eq!(Solution::search_range(vec![0,0,0,2,2,2,2,2,2,3,4,5,5,5,6,7,9,9,9,9,10,11,12], 9), vec![16,19]);
        
    }
}
