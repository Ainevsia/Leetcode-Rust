fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        let mut s: Vec<char> = s.chars().collect();
        s.insert(0, ' ');
        let mut p: Vec<char> = p.chars().collect();
        p.insert(0, ' ');
        println!("dp = {:#?}", dp);
        println!("s = {:#?}, p = {:#?}", s, p);
        for j in 1..p.len() {
            if p[j] != '*' { break }
            else { dp[0][j] = true }
        }
        for j in 1..p.len() {
            for i in 1..s.len() {
                if p[j] == '*' { dp[i][j] = dp[i-1][j-1] || dp[i][j-1] ||dp[i-1][j] }
                else if p[j] == '?' { dp[i][j] = dp[i-1][j-1] }
                else if s[i] == p[j] { dp[i][j] = dp[i-1][j-1] }
            }
        }
        println!("dp = {:#?}", dp);
        dp[s.len() - 1][p.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic_t() {
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a")), false);
        assert_eq!(Solution::is_match(String::from("cb"), String::from("?a")), false);
        assert_eq!(Solution::is_match(String::from("acdcb"), String::from("a*c?b")), false);
    }

    #[test]
    fn basic_f() {
        assert_eq!(Solution::is_match(String::from("aa"), String::from("*")), true);
        assert_eq!(Solution::is_match(String::from("adceb"), String::from("*a*b")), true);
    }

    #[test]
    fn basic() {
        assert_eq!(Solution::is_match(String::from("abccssbsbsbsdbb"), String::from("a*b?cs*sdb?")), true);
        assert_eq!(Solution::is_match(String::from("adbd"), String::from("*a******bd")), true);
    }
}
