fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut total: i32 = nums.iter().sum();
        let mut sum = 0;
        let mut res = vec![];
        for &n in nums.iter().rev() {
            sum += n;
            total -= n;
            res.push(n);
            if sum > total {
                break
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::min_subsequence(vec![4,3,10,9,8]), [10,9]);
        assert_eq!(Solution::min_subsequence(vec![4,4,7,6,7]), [7,7,6]);
        assert_eq!(Solution::min_subsequence(vec![6]), [6]);

    }
}