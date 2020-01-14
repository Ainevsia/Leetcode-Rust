fn main() {
    println!("Hello, world!");
    Solution::is_match(String::from(""), String::from(".*"));
}

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (ls, lp) = (s.len(), p.len());
        let mut dp = vec![vec![false; lp + 1]; ls + 1];
        dp[0][0] = true;
        for j in 0..lp {
            if p.bytes().nth(j).unwrap() == b'*' && dp[0][j-1] {
                dp[0][j+1] = true;
            }
        }
        // println!("dp = {:?}", dp);
        
        for i in 0..ls {
            for j in 0..lp {
                match p.bytes().nth(j).unwrap() {
                    b'.'  => {
                        dp[i+1][j+1] = dp[i][j];
                    }
                    b'*' => {
                        if p.bytes().nth(j-1).unwrap() != s.bytes().nth(i).unwrap() &&
                        p.bytes().nth(j-1).unwrap() != b'.' {
                            dp[i+1][j+1] = dp[i+1][j-1];
                        } else {
                            dp[i+1][j+1] = dp[i+1][j] || dp[i][j+1] || dp[i+1][j-1];
                        }
                    }
                    _ => {
                        if s.bytes().nth(i).unwrap() == p.bytes().nth(j).unwrap() {
                            dp[i+1][j+1] = dp[i][j];
                        }
                    }
                }
            }
        }
        dp[ls][lp]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a")), false);
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a*")), true);
        assert_eq!(Solution::is_match(String::from("ab"), String::from(".*")), true);
        assert_eq!(Solution::is_match(String::from("ab"), String::from("c*a*b")), true);
        assert_eq!(Solution::is_match(String::from("mississippi"), String::from("mis*is*p*.")), false);
        assert_eq!(Solution::is_match(String::from("mississippi"), String::from("mis*is*.p*.")), true);
    }

    #[test]
    fn null() {
        assert_eq!(Solution::is_match(String::from(""), String::from(".*")), true);
    }
}