fn main() {
    assert_eq!(Solution::candy(vec![1,0,2]), 5);
}

struct Solution {}

impl Solution {
    /// Bad solution O(n^2) time O(n) space
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() == 0 { return 0 }
        if ratings.len() == 1 { return 1 }
        let mut v = vec![0; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i-1] {
                v[i] = v[i-1] + 1;
            } else if ratings[i] <= ratings[i-1] {
                v[i] = i32::min(v[i-1]-1, 0);
            }
            if v[i] < 0 {
                // uplevel the previous
                let mut j = i - 1;
                while j != 0 && ratings[j] > ratings[j+1] && v[j] == v[j+1] + 1 { j -= 1 }
                if ratings[j] <= ratings[j + 1] || v[j] > v[j+1] + 1 { j += 1 }
                while j <= i {
                    v[j] += 1;
                    j += 1;
                }
            }
        }
        v.iter().sum::<i32>() + ratings.len() as i32
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