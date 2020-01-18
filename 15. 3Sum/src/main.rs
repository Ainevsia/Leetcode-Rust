fn main() {
    Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
}

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();
        // first sort the array from small to large
        let mut ret: Vec<Vec<i32>> = Vec::new();
        if nums.len() < 3 { return ret }

        for i in 0..nums.len()-2 {
            if nums[i] > 0 { break }
            else if i > 0 && nums[i] == nums[i - 1] { continue }
            else {
                let mut l = i + 1;
                let mut r = nums.len() - 1;
                while l < r {
                    if nums[i] + nums[l] + nums[r] < 0 {
                        l += 1
                    } else if nums[i] + nums[l] + nums[r] > 0 {
                        r -= 1
                    } else {
                        ret.push(vec![nums[i], nums[l], nums[r]]);
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1
                        }
                        l += 1;
                        while l < r && nums[r - 1] == nums[r] {
                            r -= 1
                        }
                        r -= 1;
                    }
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn basic() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            Solution::three_sum(vec![0, 0]),
            Vec::<Vec<_>>::new()
        );
        assert_eq!(
            Solution::three_sum(vec![-1,0,1,0]),
            vec![vec![-1,0,1]]
        );
        assert_eq!(
            Solution::three_sum(vec![3,0,-2,-1,1,2]),
            vec![vec![-2,-1,3], vec![-2,0,2], vec![-1,0,1]]
        );
        assert_eq!(
            Solution::three_sum(vec![]),
            Vec::<Vec<_>>::new()
        );
    }
}