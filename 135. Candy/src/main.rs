fn main() {
    assert_eq!(Solution::candy(vec![1,0,2]), 5);
}

struct Solution {}

impl Solution {
    /// two pass solution with O(n) time and O(n) space
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() <= 1 { return ratings.len() as i32 }
        let mut v = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i-1] {
                v[i] = v[i-1] + 1;
            }
        }
        for i in (0..ratings.len()-1).rev() {
            if ratings[i] > ratings[i+1] && v[i] < v[i+1] + 1 {
                v[i] = v[i+1] + 1;
            }
        }
        v.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::candy(vec![1,0,2]), 5);
        assert_eq!(Solution::candy(vec![1,2,2]), 4);
        assert_eq!(Solution::candy(vec![1,3,2,2,1]), 7);
    }

    #[test]
    fn fail() {
        assert_eq!(Solution::candy(vec![1,2,3,1,0]), 9);
    }
}