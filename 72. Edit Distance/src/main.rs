fn main() {
    assert_eq!(Solution::min_distance(String::from("horse"), String::from("ros")), 3);
}

struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        use std::cmp::min;
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let mut dp = vec![vec![0; word2.len()+1]; word1.len()+1];
        // dp need init
        for i in 1..=word1.len() { dp[i][0] = i }
        for j in 1..=word2.len() { dp[0][j] = j }
        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                if word1[i-1] == word2[j-1] { dp[i][j] = dp[i-1][j-1] }
                else {
                    dp[i][j] = min(min(dp[i-1][j], dp[i][j-1]), dp[i-1][j-1]) + 1
                }
            }
        }
        dp[word1.len()][word2.len()] as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::min_distance(String::from("horse"), String::from("ros")), 3);
        assert_eq!(Solution::min_distance(String::from("intention"), String::from("execution")), 5);

    }
}
