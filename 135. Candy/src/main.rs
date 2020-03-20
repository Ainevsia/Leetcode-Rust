fn main() {
    assert_eq!(Solution::candy(vec![1,0,2]), 5);
}

struct Solution {}

impl Solution {
    /// two pass solution with O(n) time and O(n) space
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() <= 1 { return ratings.len() as i32 }
        let mut total = 1;
        let mut prev = 1;
        let mut count_down = 0;
        for i in 1..ratings.len() {
            if ratings[i] >= ratings[i-1] {
                if count_down > 0 {
                    total += count_down * (count_down + 1) / 2;
                    if count_down >= prev {
                        total += count_down - prev + 1;
                    }
                    count_down = 0;
                    prev = 1;
                }
                prev = if ratings[i] == ratings[i-1] { 1 } else { prev + 1 };
                total += prev;
            } else {
                count_down += 1;
            }
        }
        if count_down > 0 {
            total += count_down * (count_down + 1) / 2;
            if count_down >= prev {
                total += count_down - prev + 1;
            }
        }
        total as i32
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