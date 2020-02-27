fn main() {
    assert_eq!(Solution::search(vec![2,5,6,0,0,1,2], 0), true);
}

struct Solution {}

impl Solution {
    /// I just wonder why i use recursive ?  two complex
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        Self::s(&nums, target, 0, nums.len())
    }

    pub fn s(nums: & Vec<i32>, target: i32, start: usize, end: usize) -> bool {
        if start == end { return false }
        if start + 1 == end { return target == nums[start] }
        if start + 2 == end { return nums[start] == target || nums[start + 1] == target }
        let m = (start + end - 1) / 2;
        if nums[start] == nums[end - 1] {
            return Self::s(nums, target, start + 1, end);
        }
        if nums[start] < nums[end - 1] {
            if nums[m] == target { return true }
            if nums[m] < target { return Self::s(nums, target, m+1, end) }
            if nums[m] > target { return Self::s(nums, target, start, m) }
        }
        if nums[start] > nums[end - 1] {
            if target == nums[m] { return true }
            if nums[start] <= nums[m] {
                if nums[start] <= target && target <= nums[m] {
                    return Self::s(nums, target, start, m)
                } else { return Self::s(nums, target, m+1, end) }
            }
            else {
                if nums[m] <= target && target <= nums[end - 1] {
                    return Self::s(nums, target, m+1, end) 
                } else { return Self::s(nums, target, start, m) }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::search(vec![2,5,6,0,0,1,2], 0), true);
        assert_eq!(Solution::search(vec![2,5,6,0,0,1,2], 3), false);
        assert_eq!(Solution::search(vec![1,2,1], 1), true);
        assert_eq!(Solution::search(vec![3,1,1], 3), true);
        // assert_eq!(Solution::search(vec![3,1,1,1,1], 3), true);


    }

    #[test]
    fn fail() {
        assert_eq!(Solution::search(vec![3,1,1,1,1], 3), true);

    }
}