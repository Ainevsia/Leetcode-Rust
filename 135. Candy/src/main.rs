fn main() {
    assert_eq!(Solution::candy(vec![1,0,2]), 5);
}

struct Solution {}

impl Solution {
    /// a wrong solution
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() == 0 { return 0 }
        if ratings.len() == 1 { return 1 }
        let mut sum = 0;
        let mut prev_candy = 0;
        let mut min_candy = 0;
        for i in 1..ratings.len() {
            let this_candy;
            if ratings[i] <= ratings[i-1] {
                this_candy = i32::min(prev_candy - 1, 0);
            } else {
                this_candy = prev_candy + 1;
            }
            if this_candy < min_candy { min_candy = this_candy }
            sum += this_candy;
            prev_candy = this_candy;
        }
        sum + ratings.len() as i32 * (1 + i32::abs(min_candy))
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::candy(vec![1,0,2]), 5);
        assert_eq!(Solution::candy(vec![1,2,2]), 4);
    }

    #[test]
    fn fail() {
        assert_eq!(Solution::candy(vec![1,3,2,2,1]), 7);
    }
}