fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut n = nums.clone();
        let mut res = vec![];
        res.push(n.clone());
        Self::next_permutation(&mut n);
        while n != nums {
            res.push(n.clone());
            Self::next_permutation(&mut n);
        }
        res
    }

    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() <= 1 { return }
        let mut i = nums.len() - 2;
        while let false = nums[i] < nums[i + 1] {
            if i != 0 {
                i -= 1
            } else {
                nums.reverse();
                return
            }
        }
        let mut j = nums.len() - 1;
        while let false = nums[i] < nums[j] {
            j -= 1;
        }
        nums.swap(i, j);
        nums[i+1..].reverse()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(
            Solution::permute_unique(vec![1,1,2]),
            vec![
                vec![1,1,2],
                vec![1,2,1],
                vec![2,1,1]
            ]
        );
    }

    #[test]
    fn basic2() {
        assert_eq!(
            Solution::permute_unique(vec![1,1,1]),
            vec![
                vec![1,1,1]
            ]
        );
    }

    #[test]
    fn edge() {
        assert_eq!(
            Solution::permute_unique(vec![1]),
            vec![
                vec![1],
            ]
        );
        assert_eq!(
            Solution::permute_unique(vec![]),
            vec![
                vec![],
            ]
        );
    }

    #[test]
    fn leetcode() {
        assert_eq!(
            Solution::permute_unique(vec![2,2,1,1]),
            vec![
                vec![1,1,2,2],
                vec![1,2,1,2],
                vec![1,2,2,1],
                vec![2,1,1,2],
                vec![2,1,2,1],
                vec![2,2,1,1]
            ]
        );
    }
}