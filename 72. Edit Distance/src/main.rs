fn main() {
    assert_eq!(Solution::min_distance(String::from("horse"), String::from("ros")), 3);
}

struct Solution {}

impl Solution {
    /// 2d dp -> reduce to O(n) space
    /// Time: O(mn)
    /// not clear and beautiful as before
    pub fn min_distance(word1: String, word2: String) -> i32 {
        use std::cmp::min;
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let mut dp = vec![vec![0; word2.len()+1]; 2];
        for j in 1..=word2.len() { dp[0][j] = j }
        dp[1][0] = 1;
        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                if word1[i-1] == word2[j-1] { dp[1][j] = dp[0][j-1] }
                else {
                    dp[1][j] = min(min(dp[0][j], dp[0][j-1]), dp[1][j-1]) + 1
                }
            }
            dp[0] = dp[1].clone();
            dp[1][0] = 1 + i;
        }
        dp[0][word2.len()] as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::min_distance(String::from("horse"), String::from("ros")), 3);
        assert_eq!(Solution::min_distance(String::from("intention"), String::from("execution")), 5);
        assert_eq!(Solution::min_distance(String::from("b"), String::from("")), 1);

    }
}
