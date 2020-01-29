fn main() {
    let x = Solution::search(vec![4,5,6,7,0,1,2,3], 3);
    println!("x = {:#?}", x);
}

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() { return -1 }
        let mut s = 0;
        let mut e = nums.len() - 1;
        let mut m;
        while s < e {
            if e - s == 1 {
                return 
                    if nums[s] == target { s as i32 }
                    else if nums[e] == target { e as i32 }
                    else { -1 }
            }
            m = (s + e) / 2;
            if nums[s] < nums[e] {
                // sorted : binary search
                if nums[m] < target { s = m }
                else { e = m }
            } else if nums[m] > nums[s] {
                if nums[s] <= target && target <= nums[m] {
                    e = m
                } else { s = m }
            } else {
                if nums[m] <= target && target <= nums[e] {
                    s = m;
                } else { e = m }
            }
        }
        if nums[s] != target { -1 } else { s as i32 }
    }
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 0), 4);
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 3), -1);
    }

    #[test]
    fn small() {
        assert_eq!(Solution::search(vec![], 3), -1);
        assert_eq!(Solution::search(vec![4], 3), -1);
        assert_eq!(Solution::search(vec![4], 4), 0);
        assert_eq!(Solution::search(vec![4], 5), -1);
        assert_eq!(Solution::search(vec![1,2], 1), 0);
        assert_eq!(Solution::search(vec![1,2], 2), 1);
        assert_eq!(Solution::search(vec![1,2], 3), -1);
        assert_eq!(Solution::search(vec![2,1], 1), 1);
        assert_eq!(Solution::search(vec![2,1], 2), 0);
        assert_eq!(Solution::search(vec![2,1], 3), -1);
        assert_eq!(Solution::search(vec![1,2,3], 0), -1);
        assert_eq!(Solution::search(vec![1,2,3], 1), 0);
        assert_eq!(Solution::search(vec![1,2,3], 2), 1);
        assert_eq!(Solution::search(vec![1,2,3], 3), 2);
        assert_eq!(Solution::search(vec![1,2,3], 4), -1);
    }

    #[test]
    fn even() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2,3], 4), 0);
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2,3], 5), 1);
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2,3], 6), 2);
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2,3], 7), 3);
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2,3], 0), 4);
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2,3], 1), 5);
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2,3], 2), 6);
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2,3], 3), 7);
    }

    #[test]
    fn odd() {
        assert_eq!(Solution::search(vec![4,6,7,0,1,2,3], 4), 0);
        assert_eq!(Solution::search(vec![4,6,7,0,1,2,3], 5), -1);
        assert_eq!(Solution::search(vec![4,6,7,0,1,2,3], 6), 1);
        assert_eq!(Solution::search(vec![4,6,7,0,1,2,3], 7), 2);
        assert_eq!(Solution::search(vec![4,6,7,0,1,2,3], 0), 3);
        assert_eq!(Solution::search(vec![4,6,7,0,1,2,3], 1), 4);
        assert_eq!(Solution::search(vec![4,6,7,0,1,2,3], 2), 5);
        assert_eq!(Solution::search(vec![4,6,7,0,1,2,3], 3), 6);
    }

    #[test]
    fn even_not_found() {
        assert_eq!(Solution::search(vec![10,12,14,16,2,4,6,8], 1), -1);
        assert_eq!(Solution::search(vec![10,12,14,16,2,4,6,8], 3), -1);
        assert_eq!(Solution::search(vec![10,12,14,16,2,4,6,8], 5), -1);
        assert_eq!(Solution::search(vec![10,12,14,16,2,4,6,8], 7), -1);
        assert_eq!(Solution::search(vec![10,12,14,16,2,4,6,8], 9), -1);
        assert_eq!(Solution::search(vec![10,12,14,16,2,4,6,8], 11), -1);
        assert_eq!(Solution::search(vec![10,12,14,16,2,4,6,8], 13), -1);
        assert_eq!(Solution::search(vec![10,12,14,16,2,4,6,8], 15), -1);
        assert_eq!(Solution::search(vec![10,12,14,16,2,4,6,8], 17), -1);
    }

    #[test]
    fn odd_not_found() {
        assert_eq!(Solution::search(vec![10,12,16,2,4,6,8], 1), -1);
        assert_eq!(Solution::search(vec![10,12,16,2,4,6,8], 3), -1);
        assert_eq!(Solution::search(vec![10,12,16,2,4,6,8], 5), -1);
        assert_eq!(Solution::search(vec![10,12,16,2,4,6,8], 7), -1);
        assert_eq!(Solution::search(vec![10,12,16,2,4,6,8], 9), -1);
        assert_eq!(Solution::search(vec![10,12,16,2,4,6,8], 11), -1);
        assert_eq!(Solution::search(vec![10,12,16,2,4,6,8], 13), -1);
        assert_eq!(Solution::search(vec![10,12,16,2,4,6,8], 15), -1);
        assert_eq!(Solution::search(vec![10,12,16,2,4,6,8], 17), -1);
    }
}