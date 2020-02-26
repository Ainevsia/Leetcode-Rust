fn main() {
    Solution::subsets(vec![1,2,3]);
}

struct Solution {}

impl Solution {
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 { return vec![vec![]] }
        let last = nums.pop().unwrap();
        let remain_subsets = Self::subsets(nums);
        let mut res = remain_subsets.clone();
        for mut v in remain_subsets {
            v.push(last);
            res.push(v);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::subsets(vec![1,2,3]), vec![
            vec![3],
            vec![1],
            vec![2],
            vec![1,2,3],
            vec![1,3],
            vec![2,3],
            vec![1,2],
            vec![]
        ]);
    }
}