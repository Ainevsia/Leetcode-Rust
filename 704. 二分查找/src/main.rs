fn main() {
    Solution::search(vec![-1,0,3,5,9,12], 13   );
}

struct Solution {}


impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0isize;
        let mut right = nums.len() as isize - 1;
        while left <= right {
            let mid = (left + (right - left) / 2) as usize;
            if nums[mid] < target {
                left = mid as isize + 1
            } else if nums[mid] > target {
                right = mid as isize - 1;
            } else {
                return mid as i32
            }
        }
        -1
    }
}

impl Solution {
    pub fn search1(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1
            } else if nums[mid] > target {
                right = mid;
            } else {
                return mid as i32
            }
        }
        -1
    }
}

impl Solution {
    pub fn search2(nums: Vec<i32>, target: i32) -> i32 {
        let x = nums.partition_point(|&x| x < target);
        if x == nums.len() { return -1 }
        if nums[x] == target { return x as i32 }
        -1
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn basic() {
        assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 9), 4);
        assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 2), -1);
    }
}