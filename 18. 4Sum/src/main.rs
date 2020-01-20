fn main() {
    Solution::four_sum(vec![-1,-5,-5,-3,2,5,0,4], -7);
}

struct Solution {}

impl Solution {
    /// this function owns the nums and ret
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 { return vec![] }
        nums.sort();
        println!("nums = {:?}", nums);
        let mut ret = vec![];
        for i in 0..nums.len() - 3 {
            if i > 0 && nums[i - 1] == nums[i] { continue }
            Self::three_sum(nums[i], &nums[i + 1..nums.len()], target, &mut ret);
        }
        ret
    }

    /// implement the default O(n^2) three sum
    /// nums must be at least 3 element long
    fn three_sum(first: i32, nums: &[i32], target: i32, ret: &mut Vec<Vec<i32>>) {
        let target = target - first;
        for i in 0..nums.len() - 2 {
            let a = nums[i];
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            if i > 0 && nums[i - 1] == nums[i] { continue }
            while l < r {
                let (b, c) = (nums[l], nums[r]);
                if a + b + c < target {
                    l += 1;
                } else if a + b + c > target {
                    r -= 1;
                } else {
                    ret.push(vec![first, a, b, c]);
                    while l < r && nums[r] == nums[r - 1] { r -= 1 }
                    r -= 1;
                    while l < r && nums[l] == nums[l + 1] { l += 1 }
                    l += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![
                vec![-2, -1, 1, 2],
                vec![-2,  0, 0, 2],
                vec![-1,  0, 0, 1],
            ]
        )
    }

    #[test]
    fn wa() {
        assert_eq!(
            // when first are the same, cause duplicated output
            Solution::four_sum(vec![-1,-5,-5,-3,2,5,0,4], -7),
            vec![
                vec![-5,-5,-1,4],
                vec![-5,-3,-1,2],
            ]
        )
    }

    #[test]
    fn three_sum_test() {
        let mut ret: Vec<Vec<i32>> = vec![];
        let mut v = vec![3,0,-2,-1,1,2];
        v.sort();
        Solution::three_sum(0, &v, 0, &mut ret);
        assert_eq!(
            ret,
            vec![vec![0,-2,-1,3], vec![0,-2,0,2], vec![0,-1,0,1]]
        );
    }
}